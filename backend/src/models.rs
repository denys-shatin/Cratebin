use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Visibility level for snippet access control
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Unlisted,
    Private,
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Public => write!(f, "public"),
            Visibility::Unlisted => write!(f, "unlisted"),
            Visibility::Private => write!(f, "private"),
        }
    }
}

/// Core snippet model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Snippet {
    pub id: String,
    pub content: String,
    pub visibility: Visibility,
    pub expires_at: Option<DateTime<Utc>>,
    pub password_hash: Option<String>,
    pub delete_token: String,
    pub created_at: DateTime<Utc>,
    pub size: i32,
}

/// Request model for creating a snippet
#[derive(Debug, Clone, Deserialize)]
pub struct CreateSnippetRequest {
    pub content: String,
    pub visibility: Visibility,
    pub ttl: Option<String>,
    pub password: Option<String>,
}

/// Response model for snippet creation
#[derive(Debug, Clone, Serialize)]
pub struct CreateSnippetResponse {
    pub id: String,
    pub url: String,
    pub delete_token: String,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Query parameters for retrieving a snippet
#[derive(Debug, Clone, Deserialize)]
pub struct GetSnippetQuery {
    pub password: Option<String>,
}

/// Request model for deleting a snippet
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteSnippetRequest {
    pub delete_token: String,
}

/// Public snippet model (without sensitive fields)
#[derive(Debug, Clone, Serialize)]
pub struct PublicSnippet {
    pub id: String,
    pub content: String,
    pub visibility: Visibility,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub size: i32,
}

impl From<Snippet> for PublicSnippet {
    fn from(snippet: Snippet) -> Self {
        Self {
            id: snippet.id,
            content: snippet.content,
            visibility: snippet.visibility,
            expires_at: snippet.expires_at,
            created_at: snippet.created_at,
            size: snippet.size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility_display() {
        assert_eq!(Visibility::Public.to_string(), "public");
        assert_eq!(Visibility::Unlisted.to_string(), "unlisted");
        assert_eq!(Visibility::Private.to_string(), "private");
    }

    #[test]
    fn test_visibility_equality() {
        assert_eq!(Visibility::Public, Visibility::Public);
        assert_ne!(Visibility::Public, Visibility::Private);
    }

    #[test]
    fn test_public_snippet_conversion() {
        let snippet = Snippet {
            id: "test123".to_string(),
            content: "Hello".to_string(),
            visibility: Visibility::Public,
            expires_at: None,
            password_hash: Some("secret_hash".to_string()),
            delete_token: "delete123".to_string(),
            created_at: Utc::now(),
            size: 5,
        };

        let public: PublicSnippet = snippet.clone().into();
        assert_eq!(public.id, snippet.id);
        assert_eq!(public.content, snippet.content);
        // Ensure sensitive fields are not included
        assert_eq!(serde_json::to_string(&public).unwrap().contains("password_hash"), false);
        assert_eq!(serde_json::to_string(&public).unwrap().contains("delete_token"), false);
    }
}
