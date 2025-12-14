use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use tracing::{error, info};
use tracing_subscriber;

use kilter_board_backend::{
    config::AppConfig,
    database::connection::{create_pool, run_migrations, health_check},
};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = AppConfig::from_env()?;
    info!("Starting Kilter Board API server");

    // Create database connection pool
    let db_pool = create_pool(&config.database).await?;
    
    // Run database migrations
    run_migrations(&db_pool).await?;

    // Create application state
    let app_state = AppState { db: db_pool };

    // Build our application with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .with_state(app_state)
        .layer(CorsLayer::permissive());

    // Start the server
    let listener = tokio::net::TcpListener::bind(&config.server_address()).await?;
    info!("Server running on http://{}", config.server_address());
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn root() -> &'static str {
    "Kilter Board API"
}

async fn health(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    match health_check(&state.db).await {
        Ok(_) => Ok(Json(json!({
            "status": "healthy",
            "database": "connected",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))),
        Err(e) => {
            error!("Database health check failed: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}