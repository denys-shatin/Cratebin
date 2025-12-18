use crate::models::{Snippet, Visibility};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};

#[derive(Debug)]
pub enum RepositoryError {
    DatabaseError(sqlx::Error),
    NotFound,
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::DatabaseError(e) => write!(f, "Database error: {}", e),
            RepositoryError::NotFound => write!(f, "Snippet not found"),
        }
    }
}

impl std::error::Error for RepositoryError {}

impl From<sqlx::Error> for RepositoryError {
    fn from(err: sqlx::Error) -> Self {
        RepositoryError::DatabaseError(err)
    }
}

pub struct SnippetRepository {
    pool: PgPool,
}

impl SnippetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Insert a new snippet into the database
    pub async fn insert(&self, snippet: &Snippet) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO snippets (id, content, visibility, expires_at, password_hash, delete_token, created_at, size)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(&snippet.id)
        .bind(&snippet.content)
        .bind(&snippet.visibility.to_string())
        .bind(&snippet.expires_at)
        .bind(&snippet.password_hash)
        .bind(&snippet.delete_token)
        .bind(&snippet.created_at)
        .bind(&snippet.size)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Find a snippet by ID
    pub async fn find_by_id(&self, id: &str) -> Result<Option<Snippet>, RepositoryError> {
        let row = sqlx::query(
            r#"
            SELECT id, content, visibility, expires_at, password_hash, delete_token, created_at, size
            FROM snippets
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let visibility_str: String = row.get("visibility");
                let visibility = match visibility_str.as_str() {
                    "public" => Visibility::Public,
                    "unlisted" => Visibility::Unlisted,
                    "private" => Visibility::Private,
                    _ => return Err(RepositoryError::DatabaseError(sqlx::Error::Decode(
                        format!("Invalid visibility value: {}", visibility_str).into(),
                    ))),
                };

                Ok(Some(Snippet {
                    id: row.get("id"),
                    content: row.get("content"),
                    visibility,
                    expires_at: row.get("expires_at"),
                    password_hash: row.get("password_hash"),
                    delete_token: row.get("delete_token"),
                    created_at: row.get("created_at"),
                    size: row.get("size"),
                }))
            }
            None => Ok(None),
        }
    }

    /// Delete a snippet by ID
    pub async fn delete_by_id(&self, id: &str) -> Result<(), RepositoryError> {
        let result = sqlx::query(
            r#"
            DELETE FROM snippets
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }

    /// Delete all expired snippets
    pub async fn delete_expired(&self) -> Result<usize, RepositoryError> {
        let now: DateTime<Utc> = Utc::now();
        
        let result = sqlx::query(
            r#"
            DELETE FROM snippets
            WHERE expires_at IS NOT NULL AND expires_at < $1
            "#,
        )
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() as usize)
    }

    /// Count total snippets (for testing)
    #[cfg(test)]
    pub async fn count(&self) -> Result<i64, RepositoryError> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM snippets")
            .fetch_one(&self.pool)
            .await?;
        
        Ok(row.get("count"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Visibility;
    use chrono::Duration;

    // Note: These tests require a running PostgreSQL database
    // They are integration tests and should be run with a test database

    async fn setup_test_db() -> PgPool {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://cratebin:cratebin@localhost:5432/cratebin_test".to_string());
        
        PgPool::connect(&database_url).await.expect("Failed to connect to test database")
    }

    #[tokio::test]
    #[ignore] // Requires database
    async fn test_insert_and_find() {
        let pool = setup_test_db().await;
        let repo = SnippetRepository::new(pool);

        let snippet = Snippet {
            id: "test123".to_string(),
            content: "Hello, World!".to_string(),
            visibility: Visibility::Public,
            expires_at: None,
            password_hash: None,
            delete_token: "delete123".to_string(),
            created_at: Utc::now(),
            size: 13,
        };

        // Insert
        repo.insert(&snippet).await.unwrap();

        // Find
        let found = repo.find_by_id("test123").await.unwrap();
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.id, snippet.id);
        assert_eq!(found.content, snippet.content);

        // Cleanup
        repo.delete_by_id("test123").await.unwrap();
    }

    #[tokio::test]
    #[ignore] // Requires database
    async fn test_delete_expired() {
        let pool = setup_test_db().await;
        let repo = SnippetRepository::new(pool);

        // Create expired snippet
        let expired_snippet = Snippet {
            id: "expired123".to_string(),
            content: "Expired".to_string(),
            visibility: Visibility::Public,
            expires_at: Some(Utc::now() - Duration::hours(1)),
            password_hash: None,
            delete_token: "delete123".to_string(),
            created_at: Utc::now() - Duration::hours(2),
            size: 7,
        };

        repo.insert(&expired_snippet).await.unwrap();

        // Delete expired
        let deleted_count = repo.delete_expired().await.unwrap();
        assert!(deleted_count >= 1);

        // Verify deleted
        let found = repo.find_by_id("expired123").await.unwrap();
        assert!(found.is_none());
    }
}
