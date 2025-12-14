use crate::database::connection::DatabaseConfig;
use std::env;
use tracing::warn;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server_host: String,
    pub server_port: u16,
    pub log_level: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        // Load .env file if it exists
        if let Err(e) = dotenvy::dotenv() {
            warn!("Could not load .env file: {}", e);
        }

        let database = DatabaseConfig::from_env()?;
        
        let server_host = env::var("SERVER_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());
        
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);
        
        let log_level = env::var("RUST_LOG")
            .unwrap_or_else(|_| "info".to_string());

        Ok(Self {
            database,
            server_host,
            server_port,
            log_level,
        })
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}