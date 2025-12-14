use sqlx::PgPool;
use crate::auth::JwtConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_config: JwtConfig,
}