use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Database connection pool configuration
pub struct DbConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub connection_timeout: Duration,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://cratebin:cratebin@localhost:5432/cratebin".to_string()),
            max_connections: 10,
            connection_timeout: Duration::from_secs(30),
        }
    }
}

/// Create a database connection pool
pub async fn create_pool(config: DbConfig) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(config.connection_timeout)
        .connect(&config.database_url)
        .await
}

/// Initialize database connection pool from environment
pub async fn init_pool() -> Result<PgPool, sqlx::Error> {
    let config = DbConfig::default();
    create_pool(config).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_config_default() {
        let config = DbConfig::default();
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.connection_timeout, Duration::from_secs(30));
    }
}
