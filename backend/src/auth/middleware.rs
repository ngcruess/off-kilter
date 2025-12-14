use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::auth::jwt::{verify_token, Claims, JwtConfig};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub username: String,
}

impl AuthUser {
    pub fn from_claims(claims: Claims) -> Result<Self, AuthError> {
        let id = claims.user_id().map_err(|_| AuthError::InvalidToken)?;
        Ok(Self {
            id,
            email: claims.email,
            username: claims.username,
        })
    }
}

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    ExpiredToken,
    InternalError,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authentication token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid authentication token"),
            AuthError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Authentication token expired"),
            AuthError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal authentication error"),
        };

        let body = Json(json!({
            "error": message,
            "code": status.as_u16()
        }));

        (status, body).into_response()
    }
}

pub struct RequireAuth;

#[async_trait]
impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or(AuthError::MissingToken)?;

        let auth_str = auth_header
            .to_str()
            .map_err(|_| AuthError::InvalidToken)?;

        // Check for Bearer token format
        if !auth_str.starts_with("Bearer ") {
            return Err(AuthError::InvalidToken);
        }

        let token = &auth_str[7..]; // Remove "Bearer " prefix

        if token.is_empty() {
            return Err(AuthError::InvalidToken);
        }

        // Validate the JWT token
        let jwt_config = jwt_config_from_env();
        verify_token(token, &jwt_config)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::ExpiredToken,
                _ => AuthError::InvalidToken,
            })?;

        Ok(RequireAuth)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync + Clone,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // First ensure we have a valid auth token
        let _auth = RequireAuth::from_request_parts(parts, state).await?;

        // Extract the Authorization header again
        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or(AuthError::MissingToken)?;

        let auth_str = auth_header
            .to_str()
            .map_err(|_| AuthError::InvalidToken)?;

        let token = &auth_str[7..]; // Remove "Bearer " prefix

        // Get JWT config from environment
        let jwt_config = jwt_config_from_env();

        // Verify the JWT token
        let claims = verify_token(token, &jwt_config)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::ExpiredToken,
                _ => AuthError::InvalidToken,
            })?;

        // Convert claims to AuthUser
        AuthUser::from_claims(claims)
    }
}

// Helper function to create JWT config from environment
fn jwt_config_from_env() -> JwtConfig {
    JwtConfig {
        secret: std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-change-this-in-production".to_string()),
        algorithm: jsonwebtoken::Algorithm::HS256,
        expiration_hours: std::env::var("JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse()
            .unwrap_or(24),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_missing_auth_header() {
        let mut parts = Parts::default();
        let result = RequireAuth::from_request_parts(&mut parts, &()).await;
        assert!(matches!(result, Err(AuthError::MissingToken)));
    }

    #[tokio::test]
    async fn test_invalid_auth_format() {
        let mut parts = Parts::default();
        parts.headers.insert("Authorization", "InvalidFormat".parse().unwrap());
        let result = RequireAuth::from_request_parts(&mut parts, &()).await;
        assert!(matches!(result, Err(AuthError::InvalidToken)));
    }

    #[tokio::test]
    async fn test_valid_bearer_token() {
        let mut parts = Parts::default();
        parts.headers.insert("Authorization", "Bearer valid-token-here".parse().unwrap());
        let result = RequireAuth::from_request_parts(&mut parts, &()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_empty_bearer_token() {
        let mut parts = Parts::default();
        parts.headers.insert("Authorization", "Bearer ".parse().unwrap());
        let result = RequireAuth::from_request_parts(&mut parts, &()).await;
        assert!(matches!(result, Err(AuthError::InvalidToken)));
    }
}