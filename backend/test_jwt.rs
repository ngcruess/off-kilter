use kilter_board_backend::auth::jwt::{create_token, verify_token, JwtConfig};
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the same config as the app
    std::env::set_var("JWT_SECRET", "your-super-secret-jwt-key-change-this-in-production-please");
    std::env::set_var("JWT_EXPIRATION_HOURS", "24");
    
    let config = JwtConfig::from_env()?;
    let user_id = Uuid::new_v4();
    let email = "test@example.com".to_string();
    let username = "testuser".to_string();
    
    println!("Creating token with config:");
    println!("Secret: {}", config.secret);
    println!("Algorithm: {:?}", config.algorithm);
    println!("Expiration hours: {}", config.expiration_hours);
    
    let token = create_token(user_id, email.clone(), username.clone(), &config)?;
    println!("Generated token: {}", token);
    
    let claims = verify_token(&token, &config)?;
    println!("Verified claims:");
    println!("User ID: {}", claims.user_id()?);
    println!("Email: {}", claims.email);
    println!("Username: {}", claims.username);
    
    Ok(())
}