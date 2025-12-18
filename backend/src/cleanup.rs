use crate::service::SnippetService;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use tracing::{error, info};

/// Background cleanup service
pub struct CleanupService {
    snippet_service: Arc<SnippetService>,
    interval: Duration,
}

impl CleanupService {
    pub fn new(snippet_service: Arc<SnippetService>, interval_secs: u64) -> Self {
        Self {
            snippet_service,
            interval: Duration::from_secs(interval_secs),
        }
    }

    /// Run periodic cleanup task
    pub async fn run_periodic_cleanup(self) {
        let mut interval = time::interval(self.interval);

        loop {
            interval.tick().await;
            
            match self.delete_expired_snippets().await {
                Ok(count) => {
                    if count > 0 {
                        info!("Cleanup task deleted {} expired snippets", count);
                    }
                }
                Err(e) => {
                    error!("Cleanup task failed: {}", e);
                }
            }
        }
    }

    /// Delete all expired snippets
    async fn delete_expired_snippets(&self) -> Result<usize, String> {
        self.snippet_service
            .delete_expired()
            .await
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::SnippetRepository;
    use sqlx::PgPool;

    async fn setup_test_service() -> Arc<SnippetService> {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://cratebin:cratebin@localhost:5432/cratebin_test".to_string());
        
        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to test database");
        
        let repository = SnippetRepository::new(pool);
        Arc::new(SnippetService::new(repository, 524288))
    }

    #[tokio::test]
    #[ignore] // Requires database
    async fn test_cleanup_service_creation() {
        let service = setup_test_service().await;
        let cleanup = CleanupService::new(service, 3600);
        
        assert_eq!(cleanup.interval, Duration::from_secs(3600));
    }

    #[tokio::test]
    #[ignore] // Requires database
    async fn test_delete_expired_snippets() {
        let service = setup_test_service().await;
        let cleanup = CleanupService::new(service, 3600);
        
        // Should not error even if no expired snippets
        let result = cleanup.delete_expired_snippets().await;
        assert!(result.is_ok());
    }
}
