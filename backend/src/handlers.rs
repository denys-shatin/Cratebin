use crate::models::{CreateSnippetRequest, CreateSnippetResponse, DeleteSnippetRequest, GetSnippetQuery, PublicSnippet};
use crate::service::{ServiceError, SnippetService};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::sync::Arc;

pub type AppState = Arc<SnippetService>;

/// API error response
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": {
                "message": self.message,
            }
        }));

        (self.status, body).into_response()
    }
}

impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::NotFound => ApiError {
                status: StatusCode::NOT_FOUND,
                message: "Snippet not found or has expired".to_string(),
            },
            ServiceError::Forbidden => ApiError {
                status: StatusCode::FORBIDDEN,
                message: "Access forbidden - incorrect password".to_string(),
            },
            ServiceError::ContentTooLarge => ApiError {
                status: StatusCode::PAYLOAD_TOO_LARGE,
                message: "Content exceeds maximum size of 512 KB".to_string(),
            },
            ServiceError::InvalidUtf8 => ApiError {
                status: StatusCode::BAD_REQUEST,
                message: "Content must be valid UTF-8".to_string(),
            },
            ServiceError::TtlError(e) => ApiError {
                status: StatusCode::BAD_REQUEST,
                message: format!("Invalid TTL: {}", e),
            },
            ServiceError::PasswordHashError(e) => ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Password processing error: {}", e),
            },
            ServiceError::RepositoryError(e) => ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Database error: {}", e),
            },
        }
    }
}

/// POST /snippets - Create a new snippet
pub async fn create_snippet(
    State(service): State<AppState>,
    Json(request): Json<CreateSnippetRequest>,
) -> Result<(StatusCode, Json<CreateSnippetResponse>), ApiError> {
    let (snippet, delete_token) = service
        .create(
            request.content,
            request.visibility,
            request.ttl,
            request.password,
        )
        .await?;

    let response = CreateSnippetResponse {
        id: snippet.id.clone(),
        url: format!("/snippets/{}", snippet.id),
        delete_token,
        expires_at: snippet.expires_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

/// GET /snippets/{id} - Get a snippet
pub async fn get_snippet(
    State(service): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<GetSnippetQuery>,
) -> Result<Json<PublicSnippet>, ApiError> {
    let snippet = service.get_by_id(&id, query.password).await?;
    Ok(Json(snippet))
}

/// GET /snippets/{id}/raw - Get raw snippet content
pub async fn get_snippet_raw(
    State(service): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<GetSnippetQuery>,
) -> Result<String, ApiError> {
    let snippet = service.get_by_id(&id, query.password).await?;
    Ok(snippet.content)
}

/// DELETE /snippets/{id} - Delete a snippet
pub async fn delete_snippet(
    State(service): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<DeleteSnippetRequest>,
) -> Result<StatusCode, ApiError> {
    service.delete(&id, &request.delete_token).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Visibility;
    use crate::repository::SnippetRepository;
    use sqlx::PgPool;

    async fn setup_test_state() -> AppState {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://cratebin:cratebin@localhost:5432/cratebin_test".to_string());
        
        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to test database");
        
        let repository = SnippetRepository::new(pool);
        let service = SnippetService::new(repository, 524288);
        Arc::new(service)
    }

    #[tokio::test]
    #[ignore] // Requires database
    async fn test_create_and_get_snippet() {
        let state = setup_test_state().await;

        // Create snippet
        let request = CreateSnippetRequest {
            content: "Test content".to_string(),
            visibility: Visibility::Public,
            ttl: Some("1h".to_string()),
            password: None,
        };

        let result = create_snippet(State(state.clone()), Json(request)).await;
        assert!(result.is_ok());
        
        let (status, response) = result.unwrap();
        assert_eq!(status, StatusCode::CREATED);
        assert!(!response.id.is_empty());

        // Get snippet
        let query = GetSnippetQuery { password: None };
        let result = get_snippet(State(state.clone()), Path(response.id.clone()), Query(query)).await;
        assert!(result.is_ok());
        
        let snippet = result.unwrap().0;
        assert_eq!(snippet.content, "Test content");

        // Cleanup
        let delete_req = DeleteSnippetRequest {
            delete_token: response.delete_token.clone(),
        };
        delete_snippet(State(state), Path(response.id.clone()), Json(delete_req)).await.unwrap();
    }
}
