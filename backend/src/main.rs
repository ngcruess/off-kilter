use axum::{
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .layer(CorsLayer::permissive());

    // Run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Kilter Board API"
}