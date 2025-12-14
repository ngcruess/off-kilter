use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub algorithm: Algorithm,
    pub expiration_hours: i64,
}

impl JwtConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| {
                tracing::warn!("JWT_SECRET not set, using default (not secure for production)");
                "your-secret-key-change-this-in-production".to_string()
            });

        let expiration_hours = env::var("JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse()
            .unwrap_or(24);

        Ok(Self {
            secret,
            algorithm: Algorithm::HS256,
            expiration_hours,
        })
    }

    pub fn encoding_key(&self) -> EncodingKey {
        EncodingKey::from_secret(self.secret.as_ref())
    }

    pub fn decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.secret.as_ref())
    }

    pub fn validation(&self) -> Validation {
        let mut validation = Validation::new(self.algorithm);
        validation.validate_exp = true;
        validation.validate_nbf = false;
        validation
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,    // Subject (user ID)
    pub email: String,  // User email
    pub username: String, // Username
    pub exp: i64,       // Expiration time
    pub iat: i64,       // Issued at
}

impl Claims {
    pub fn new(user_id: Uuid, email: String, username: String, config: &JwtConfig) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(config.expiration_hours);

        Self {
            sub: user_id.to_string(),
            email,
            username,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }

    pub fn user_id(&self) -> Result<Uuid, uuid::Error> {
        Uuid::parse_str(&self.sub)
    }
}

pub fn create_token(user_id: Uuid, email: String, username: String, config: &JwtConfig) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id, email, username, config);
    let header = Header::new(config.algorithm);
    encode(&header, &claims, &config.encoding_key())
}

pub fn verify_token(token: &str, config: &JwtConfig) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(token, &config.decoding_key(), &config.validation())?;
    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> JwtConfig {
        JwtConfig {
            secret: "test-secret".to_string(),
            algorithm: Algorithm::HS256,
            expiration_hours: 1,
        }
    }

    #[test]
    fn test_create_and_verify_token() {
        let config = test_config();
        let user_id = Uuid::new_v4();
        let email = "test@example.com".to_string();
        let username = "testuser".to_string();

        let token = create_token(user_id, email.clone(), username.clone(), &config).unwrap();
        let claims = verify_token(&token, &config).unwrap();

        assert_eq!(claims.user_id().unwrap(), user_id);
        assert_eq!(claims.email, email);
        assert_eq!(claims.username, username);
    }

    #[test]
    fn test_invalid_token() {
        let config = test_config();
        let result = verify_token("invalid.token.here", &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_secret() {
        let config1 = test_config();
        let mut config2 = test_config();
        config2.secret = "different-secret".to_string();

        let user_id = Uuid::new_v4();
        let token = create_token(user_id, "test@example.com".to_string(), "testuser".to_string(), &config1).unwrap();
        let result = verify_token(&token, &config2);
        assert!(result.is_err());
    }
}