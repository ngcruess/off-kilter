use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://postgres:postgres@localhost:5432/kilter_board".to_string(),
            max_connections: 10,
            min_connections: 1,
            connect_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600), // 10 minutes
        }
    }
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| {
                warn!("DATABASE_URL not set, using default");
                "postgres://postgres:postgres@localhost:5432/kilter_board".to_string()
            });

        let max_connections = std::env::var("DB_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10);

        let min_connections = std::env::var("DB_MIN_CONNECTIONS")
            .unwrap_or_else(|_| "1".to_string())
            .parse()
            .unwrap_or(1);

        Ok(Self {
            url,
            max_connections,
            min_connections,
            connect_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
        })
    }
}

pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    info!("Connecting to database: {}", mask_password(&config.url));
    
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(config.connect_timeout)
        .idle_timeout(config.idle_timeout)
        .connect(&config.url)
        .await?;

    info!(
        "Database connection pool created with {}-{} connections",
        config.min_connections, config.max_connections
    );

    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(pool).await?;
    info!("Database migrations completed successfully");
    Ok(())
}

pub async fn health_check(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").fetch_one(pool).await?;
    Ok(())
}

fn mask_password(url: &str) -> String {
    if let Ok(parsed) = url::Url::parse(url) {
        let mut masked = parsed.clone();
        if parsed.password().is_some() {
            let _ = masked.set_password(Some("***"));
        }
        masked.to_string()
    } else {
        url.to_string()
    }
}