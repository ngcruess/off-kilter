use sqlx::PgPool;
use crate::auth::JwtConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_config: JwtConfig,
}

impl AsRef<crate::auth::JwtConfig> for AppState {
    fn as_ref(&self) -> &crate::auth::JwtConfig {
        &self.jwt_config
    }
}