mod cleanup;
mod db;
mod handlers;
mod middleware;
mod models;
mod repository;
mod service;
mod ttl;
mod utils;
mod validation;

#[cfg(test)]
mod cleanup_test;
#[cfg(test)]
mod config_test;
#[cfg(test)]
mod handlers_test;
#[cfg(test)]
mod middleware_test;
#[cfg(test)]
mod models_test;
#[cfg(test)]
mod service_test;
#[cfg(test)]
mod ttl_test;
#[cfg(test)]
mod utils_test;
#[cfg(test)]
mod validation_test;

use axum::{
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cratebin=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Configuration
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://cratebin:cratebin@localhost:5432/cratebin".to_string());
    let max_content_size: usize = std::env::var("MAX_CONTENT_SIZE")
        .unwrap_or_else(|_| "524288".to_string())
        .parse()
        .expect("MAX_CONTENT_SIZE must be a number");
    let cleanup_interval: u64 = std::env::var("CLEANUP_INTERVAL")
        .unwrap_or_else(|_| "3600".to_string())
        .parse()
        .expect("CLEANUP_INTERVAL must be a number");
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let server_port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());

    tracing::info!("Connecting to database...");
    let pool = sqlx::PgPool::connect(&database_url).await?;

    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Initialize services
    let repository = repository::SnippetRepository::new(pool);
    let service = Arc::new(service::SnippetService::new(repository, max_content_size));

    // Start cleanup task
    let cleanup_service = cleanup::CleanupService::new(service.clone(), cleanup_interval);
    tokio::spawn(async move {
        cleanup_service.run_periodic_cleanup().await;
    });

    // Build router
    let app = Router::new()
        .route("/snippets", post(handlers::create_snippet))
        .route("/snippets/:id", get(handlers::get_snippet))
        .route("/snippets/:id/raw", get(handlers::get_snippet_raw))
        .route("/snippets/:id", delete(handlers::delete_snippet))
        .layer(middleware::cors_layer())
        .with_state(service);

    // Start server
    let addr = format!("{}:{}", server_host, server_port);
    tracing::info!("Starting server on {}", addr);
    
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

