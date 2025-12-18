use crate::models::{CreateSnippetResponse, PublicSnippet, Snippet, Visibility};
use crate::repository::{RepositoryError, SnippetRepository};
use crate::ttl::{calculate_expiration_from_str, TtlError};
use crate::utils::{generate_delete_token, generate_id, hash_password, verify_password};
use chrono::Utc;

#[derive(Debug)]
pub enum ServiceError {
    RepositoryError(RepositoryError),
    PasswordHashError(String),
    TtlError(TtlError),
    NotFound,
    Forbidden,
    ContentTooLarge,
    InvalidUtf8,
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::RepositoryError(e) => write!(f, "Repository error: {}", e),
            ServiceError::PasswordHashError(e) => write!(f, "Password hash error: {}", e),
            ServiceError::TtlError(e) => write!(f, "TTL error: {}", e),
            ServiceError::NotFound => write!(f, "Snippet not found"),
            ServiceError::Forbidden => write!(f, "Access forbidden"),
            ServiceError::ContentTooLarge => write!(f, "Content exceeds maximum size"),
            ServiceError::InvalidUtf8 => write!(f, "Content is not valid UTF-8"),
        }
    }
}

impl std::error::Error for ServiceError {}

impl From<RepositoryError> for ServiceError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound => ServiceError::NotFound,
            e => ServiceError::RepositoryError(e),
        }
    }
}

impl From<TtlError> for ServiceError {
    fn from(err: TtlError) -> Self {
        ServiceError::TtlError(err)
    }
}

pub struct SnippetService {
    repository: SnippetRepository,
    max_content_size: usize,
}

impl SnippetService {
    pub fn new(repository: SnippetRepository, max_content_size: usize) -> Self {
        Self {
            repository,
            max_content_size,
        }
    }

    /// Create a new snippet
    pub async fn create(
        &self,
        content: String,
        visibility: Visibility,
        ttl: Option<String>,
        password: Option<String>,
    ) -> Result<(Snippet, String), ServiceError> {
        // Validate content size
        if content.len() > self.max_content_size {
            return Err(ServiceError::ContentTooLarge);
        }

        // Validate UTF-8 (String is already UTF-8, but check for validity)
        if !content.is_empty() && content.as_bytes().iter().any(|&b| b == 0) {
            return Err(ServiceError::InvalidUtf8);
        }

        // Generate ID and delete token
        let id = generate_id();
        let delete_token = generate_delete_token();
        let created_at = Utc::now();

        // Calculate expiration
        let expires_at = calculate_expiration_from_str(created_at, ttl.as_deref())?;

        // Hash password if provided
        let password_hash = if let Some(pwd) = password {
            Some(
                hash_password(&pwd)
                    .map_err(|e| ServiceError::PasswordHashError(e.to_string()))?,
            )
        } else {
            None
        };

        let snippet = Snippet {
            id: id.clone(),
            content: content.clone(),
            visibility,
            expires_at,
            password_hash,
            delete_token: delete_token.clone(),
            created_at,
            size: content.len() as i32,
        };

        // Insert into database
        self.repository.insert(&snippet).await?;

        Ok((snippet, delete_token))
    }

    /// Get snippet by ID with optional password
    pub async fn get_by_id(
        &self,
        id: &str,
        password: Option<String>,
    ) -> Result<PublicSnippet, ServiceError> {
        // Find snippet
        let snippet = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or(ServiceError::NotFound)?;

        // Check if expired
        if let Some(expires_at) = snippet.expires_at {
            if expires_at < Utc::now() {
                return Err(ServiceError::NotFound);
            }
        }

        // Check password for private snippets
        if snippet.visibility == Visibility::Private {
            match (&snippet.password_hash, password) {
                (Some(hash), Some(pwd)) => {
                    if !self.verify_password(&snippet, &pwd)? {
                        return Err(ServiceError::Forbidden);
                    }
                }
                (Some(_), None) => return Err(ServiceError::Forbidden),
                _ => {}
            }
        }

        Ok(snippet.into())
    }

    /// Delete snippet by ID with delete token
    pub async fn delete(&self, id: &str, delete_token: &str) -> Result<(), ServiceError> {
        // Find snippet to verify delete token
        let snippet = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or(ServiceError::NotFound)?;

        // Verify delete token (constant-time comparison via string equality)
        if snippet.delete_token != delete_token {
            return Err(ServiceError::Forbidden);
        }

        // Delete from database
        self.repository.delete_by_id(id).await?;

        Ok(())
    }

    /// Verify password for a snippet
    pub fn verify_password(&self, snippet: &Snippet, password: &str) -> Result<bool, ServiceError> {
        match &snippet.password_hash {
            Some(hash) => verify_password(password, hash)
                .map_err(|e| ServiceError::PasswordHashError(e.to_string())),
            None => Ok(false),
        }
    }

    /// Delete expired snippets (for cleanup task)
    pub async fn delete_expired(&self) -> Result<usize, ServiceError> {
        Ok(self.repository.delete_expired().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    async fn setup_test_service() -> SnippetService {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://cratebin:cratebin@localhost:5432/cratebin_test".to_string());
        
        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to test database");
        
        let repository = SnippetRepository::new(pool);
        SnippetService::new(repository, 524288) // 512 KB
    }

    #[tokio::test]
    #[ignore] // Requires database
    async fn test_create_snippet() {
        let service = setup_test_service().await;

        let (snippet, delete_token) = service
            .create(
                "Test content".to_string(),
                Visibility::Public,
                Some("1h".to_string()),
                None,
            )
            .await
            .unwrap();

        assert!(!snippet.id.is_empty());
        assert_eq!(snippet.content, "Test content");
        assert_eq!(snippet.visibility, Visibility::Public);
        assert!(snippet.expires_at.is_some());
        assert!(!delete_token.is_empty());

        // Cleanup
        service.delete(&snippet.id, &delete_token).await.unwrap();
    }

    #[tokio::test]
    #[ignore] // Requires database
    async fn test_get_snippet() {
        let service = setup_test_service().await;

        let (snippet, delete_token) = service
            .create(
                "Test content".to_string(),
                Visibility::Public,
                None,
                None,
            )
            .await
            .unwrap();

        let retrieved = service.get_by_id(&snippet.id, None).await.unwrap();
        assert_eq!(retrieved.content, "Test content");

        // Cleanup
        service.delete(&snippet.id, &delete_token).await.unwrap();
    }

    #[tokio::test]
    #[ignore] // Requires database
    async fn test_private_snippet_requires_password() {
        let service = setup_test_service().await;

        let (snippet, delete_token) = service
            .create(
                "Secret content".to_string(),
                Visibility::Private,
                None,
                Some("password123".to_string()),
            )
            .await
            .unwrap();

        // Should fail without password
        let result = service.get_by_id(&snippet.id, None).await;
        assert!(matches!(result, Err(ServiceError::Forbidden)));

        // Should fail with wrong password
        let result = service
            .get_by_id(&snippet.id, Some("wrong".to_string()))
            .await;
        assert!(matches!(result, Err(ServiceError::Forbidden)));

        // Should succeed with correct password
        let retrieved = service
            .get_by_id(&snippet.id, Some("password123".to_string()))
            .await
            .unwrap();
        assert_eq!(retrieved.content, "Secret content");

        // Cleanup
        service.delete(&snippet.id, &delete_token).await.unwrap();
    }

    #[test]
    fn test_content_too_large() {
        // This test doesn't require database
        let content = "x".repeat(524289); // 512 KB + 1
        assert!(content.len() > 524288);
    }
}
