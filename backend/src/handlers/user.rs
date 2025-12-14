use axum::{
    extract::{Path, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    auth::AuthUser,
    error::AppError,
    models::user::{CreateUserRequest, UpdateUserRequest, ProfileData, PublicUser, PublicStatistics, UserStatistics, StatisticsData},
    repositories::user::UserRepository,
    state::AppState,
};

/// Helper function to create public statistics based on privacy settings
fn create_public_statistics(
    statistics: &UserStatistics,
    statistics_data: &StatisticsData,
    visibility: &str,
) -> PublicStatistics {
    match visibility {
        "public" => PublicStatistics {
            total_attempts: Some(statistics.total_attempts),
            total_ascents: Some(statistics.total_ascents),
            personal_best_grade: statistics.personal_best_grade.clone(),
            grade_distribution: Some(statistics_data.grade_distribution.clone()),
        },
        _ => PublicStatistics {
            total_attempts: None,
            total_ascents: None,
            personal_best_grade: None,
            grade_distribution: None,
        },
    }
}

/// Helper function to filter profile data based on privacy settings for public access
fn filter_profile_for_public(profile_data: &ProfileData) -> ProfileData {
    match profile_data.privacy_settings.profile_visibility.as_str() {
        "public" => profile_data.clone(),
        "friends" => ProfileData {
            first_name: None,
            last_name: None,
            display_name: profile_data.display_name.clone(),
            bio: None,
            avatar_url: profile_data.avatar_url.clone(),
            location: None,
            preferred_units: None,
            privacy_settings: profile_data.privacy_settings.clone(),
        },
        _ => ProfileData {
            display_name: Some("Private User".to_string()),
            ..Default::default()
        },
    }
}

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(register_user))
        .route("/users/me", get(get_current_user))
        .route("/users/me", put(update_current_user))
        .route("/users/me", delete(delete_current_user))
        .route("/users/:id", get(get_user_by_id))
}

/// Register a new user
async fn register_user(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<Value>, AppError> {
    // Fast-fail validation first (in-memory operations)
    
    // Validate email format (basic validation)
    if !request.email.contains('@') {
        return Err(AppError::BadRequest("Invalid email format".to_string()));
    }

    // Validate username (basic validation)
    if request.username.len() < 3 || request.username.len() > 50 {
        return Err(AppError::BadRequest("Username must be between 3 and 50 characters".to_string()));
    }

    // Now perform database operations
    let repo = UserRepository::new(state.db);

    // Create the user (this will check for duplicates inside a transaction)
    let user = repo.create_user(
        request.email,
        request.username,
        request.profile,
    ).await?;

    Ok(Json(json!({
        "message": "User registered successfully",
        "user": {
            "id": user.id,
            "email": user.email,
            "username": user.username,
            "created_at": user.created_at
        }
    })))
}

/// Get current user's profile
async fn get_current_user(
    user: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<PublicUser>, AppError> {
    let repo = UserRepository::new(state.db);

    let (user_data, profile, statistics) = repo.get_user_with_details(user.id).await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let profile_data = profile.get_profile_data()
        .map_err(|e| AppError::InternalError(format!("Failed to parse profile data: {}", e)))?;

    let statistics_data = statistics.get_statistics_data()
        .map_err(|e| AppError::InternalError(format!("Failed to parse statistics data: {}", e)))?;

    // Respect privacy settings using helper function
    let public_stats = create_public_statistics(
        &statistics,
        &statistics_data,
        &profile_data.privacy_settings.statistics_visibility,
    );

    let public_user = PublicUser {
        id: user_data.id,
        username: user_data.username,
        profile: profile_data,
        statistics: public_stats,
        created_at: user_data.created_at,
    };

    Ok(Json(public_user))
}

/// Update current user's profile
async fn update_current_user(
    user: AuthUser,
    State(state): State<AppState>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<Value>, AppError> {
    // Fast-fail validation before database operations
    if let Some(ref profile_data) = request.profile {
        // Validate display name length
        if let Some(ref display_name) = profile_data.display_name {
            if display_name.len() > 50 {
                return Err(AppError::BadRequest("Display name must be 50 characters or less".to_string()));
            }
        }
        
        // Validate bio length
        if let Some(ref bio) = profile_data.bio {
            if bio.len() > 500 {
                return Err(AppError::BadRequest("Bio must be 500 characters or less".to_string()));
            }
        }
        
        // Validate privacy settings
        let valid_visibility_options = ["public", "friends", "private"];
        if !valid_visibility_options.contains(&profile_data.privacy_settings.profile_visibility.as_str()) {
            return Err(AppError::BadRequest("Invalid profile visibility setting".to_string()));
        }
        if !valid_visibility_options.contains(&profile_data.privacy_settings.statistics_visibility.as_str()) {
            return Err(AppError::BadRequest("Invalid statistics visibility setting".to_string()));
        }
        if !valid_visibility_options.contains(&profile_data.privacy_settings.history_visibility.as_str()) {
            return Err(AppError::BadRequest("Invalid history visibility setting".to_string()));
        }
        
        // Validate preferred units
        if let Some(ref units) = profile_data.preferred_units {
            if !["metric", "imperial"].contains(&units.as_str()) {
                return Err(AppError::BadRequest("Preferred units must be 'metric' or 'imperial'".to_string()));
            }
        }
    }

    let repo = UserRepository::new(state.db);

    if let Some(profile_data) = request.profile {
        repo.update_profile(user.id, profile_data).await?;
    }

    Ok(Json(json!({
        "message": "Profile updated successfully"
    })))
}

/// Delete current user's account
async fn delete_current_user(
    user: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let repo = UserRepository::new(state.db);

    repo.delete_user(user.id).await?;

    Ok(Json(json!({
        "message": "Account deleted successfully"
    })))
}

/// Get public user profile by ID
async fn get_user_by_id(
    Path(user_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<PublicUser>, AppError> {
    let repo = UserRepository::new(state.db);

    let (user_data, profile, statistics) = repo.get_user_with_details(user_id).await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let profile_data = profile.get_profile_data()
        .map_err(|e| AppError::InternalError(format!("Failed to parse profile data: {}", e)))?;

    let statistics_data = statistics.get_statistics_data()
        .map_err(|e| AppError::InternalError(format!("Failed to parse statistics data: {}", e)))?;

    // Check privacy settings for public access using helper functions
    let filtered_profile = filter_profile_for_public(&profile_data);
    let public_stats = create_public_statistics(
        &statistics,
        &statistics_data,
        &profile_data.privacy_settings.statistics_visibility,
    );

    let public_user = PublicUser {
        id: user_data.id,
        username: if profile_data.privacy_settings.profile_visibility == "private" {
            "Private User".to_string()
        } else {
            user_data.username
        },
        profile: filtered_profile,
        statistics: public_stats,
        created_at: user_data.created_at,
    };

    Ok(Json(public_user))
}

#[cfg(test)]
mod tests {
    // Note: These tests would require a test database setup
    // For now, they're just structure examples

    #[tokio::test]
    #[ignore]
    async fn test_register_user() {
        // Test implementation would go here
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_current_user() {
        // Test implementation would go here
    }

    #[tokio::test]
    #[ignore]
    async fn test_update_user_profile() {
        // Test implementation would go here
    }

    #[tokio::test]
    #[ignore]
    async fn test_delete_user() {
        // Test implementation would go here
    }
}