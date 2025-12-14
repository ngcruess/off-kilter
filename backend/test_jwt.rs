// Simple script to generate a test JWT token
use uuid::Uuid;

fn main() {
    let config = kilter_board_backend::auth::jwt::JwtConfig {
        secret: "your-super-secret-jwt-key-change-this-in-production-please".to_string(),
        algorithm: jsonwebtoken::Algorithm::HS256,
        expiration_hours: 24,
    };

    let user_id = Uuid::new_v4();
    let email = "test@example.com".to_string();
    let username = "testuser".to_string();

    match kilter_board_backend::auth::jwt::create_token(user_id, email, username, &config) {
        Ok(token) => {
            println!("Test JWT Token:");
            println!("{}", token);
            println!("\nUser ID: {}", user_id);
        }
        Err(e) => {
            eprintln!("Error creating token: {}", e);
        }
    }
}