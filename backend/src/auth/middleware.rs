use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::auth::jwt::{verify_token, Claims};

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
    S: Send + Sync + AsRef<crate::auth::JwtConfig>,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
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

        // Use cached JWT config from app state instead of loading from env every time
        let jwt_config = state.as_ref();
        
        tracing::debug!("Validating JWT token with secret: {}", &jwt_config.secret[..10]);
        tracing::debug!("Token to validate: {}", &token[..token.len().min(20)]);
        
        verify_token(token, jwt_config)
            .map_err(|e| {
                tracing::error!("JWT validation failed: {:?}", e);
                match e.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::ExpiredToken,
                    _ => AuthError::InvalidToken,
                }
            })?;

        Ok(RequireAuth)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync + AsRef<crate::auth::JwtConfig>,
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

        // Use cached JWT config from app state
        let jwt_config = state.as_ref();
        
        tracing::debug!("Extracting user from JWT token");

        // Verify the JWT token
        let claims = verify_token(token, jwt_config)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::ExpiredToken,
                _ => AuthError::InvalidToken,
            })?;

        // Convert claims to AuthUser
        AuthUser::from_claims(claims)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{HeaderValue, Method};
    use crate::auth::JwtConfig;

    struct TestState {
        jwt_config: JwtConfig,
    }

    impl AsRef<JwtConfig> for TestState {
        fn as_ref(&self) -> &JwtConfig {
            &self.jwt_config
        }
    }

    fn create_test_parts() -> Parts {
        let request = axum::http::Request::builder()
            .method(Method::GET)
            .uri("/")
            .body(())
            .unwrap();
        let (parts, _) = request.into_parts();
        parts
    }

    fn create_test_state() -> TestState {
        TestState {
            jwt_config: JwtConfig {
                secret: "test-secret".to_string(),
                expiration_hours: 24,
                algorithm: jsonwebtoken::Algorithm::HS256,
            }
        }
    }

    #[tokio::test]
    async fn test_missing_auth_header() {
        let mut parts = create_test_parts();
        let state = create_test_state();
        let result = RequireAuth::from_request_parts(&mut parts, &state).await;
        assert!(matches!(result, Err(AuthError::MissingToken)));
    }

    #[tokio::test]
    async fn test_invalid_auth_format() {
        let mut parts = create_test_parts();
        let state = create_test_state();
        parts.headers.insert("Authorization", HeaderValue::from_static("InvalidFormat"));
        let result = RequireAuth::from_request_parts(&mut parts, &state).await;
        assert!(matches!(result, Err(AuthError::InvalidToken)));
    }

    #[tokio::test]
    async fn test_empty_bearer_token() {
        let mut parts = create_test_parts();
        let state = create_test_state();
        parts.headers.insert("Authorization", HeaderValue::from_static("Bearer "));
        let result = RequireAuth::from_request_parts(&mut parts, &state).await;
        assert!(matches!(result, Err(AuthError::InvalidToken)));
    }

    #[tokio::test]
    async fn test_bearer_token_format_accepted() {
        // This test just checks that the Bearer format is accepted
        // The actual JWT validation will fail, but we should get past the format check
        let mut parts = create_test_parts();
        let state = create_test_state();
        parts.headers.insert("Authorization", HeaderValue::from_static("Bearer some-token"));
        let result = RequireAuth::from_request_parts(&mut parts, &state).await;
        // Should fail on JWT validation, not format validation
        assert!(matches!(result, Err(AuthError::InvalidToken)));
    }
}