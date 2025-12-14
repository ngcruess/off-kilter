use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::user::{User, UserProfile, UserStatistics, ProfileData};
use crate::error::AppError;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new user with initial profile and statistics
    pub async fn create_user(&self, email: String, username: String, profile_data: Option<ProfileData>) -> Result<User, AppError> {
        let mut tx = self.pool.begin().await?;
        
        // Create the user
        let row = sqlx::query!(
            r#"
            INSERT INTO users (email, username)
            VALUES ($1, $2)
            RETURNING id, email, username, created_at, updated_at
            "#,
            email,
            username
        )
        .fetch_one(&mut *tx)
        .await?;

        let user = User {
            id: row.id,
            email: row.email,
            username: row.username,
            created_at: row.created_at.unwrap_or_else(|| Utc::now()),
            updated_at: row.updated_at.unwrap_or_else(|| Utc::now()),
        };

        // Create initial profile
        let profile = UserProfile::new(user.id, profile_data);
        sqlx::query!(
            r#"
            INSERT INTO user_profiles (user_id, profile_data, updated_at)
            VALUES ($1, $2, $3)
            "#,
            profile.user_id,
            profile.profile_data,
            profile.updated_at
        )
        .execute(&mut *tx)
        .await?;

        // Create initial statistics
        let statistics = UserStatistics::new(user.id);
        sqlx::query!(
            r#"
            INSERT INTO user_statistics (user_id, total_attempts, total_ascents, personal_best_grade, statistics_data, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            statistics.user_id,
            statistics.total_attempts,
            statistics.total_ascents,
            statistics.personal_best_grade,
            statistics.statistics_data,
            statistics.updated_at
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(user)
    }

    /// Find user by ID
    pub async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, AppError> {
        let row = sqlx::query!(
            "SELECT id, email, username, created_at, updated_at FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.id,
            email: r.email,
            username: r.username,
            created_at: r.created_at.unwrap_or_else(|| Utc::now()),
            updated_at: r.updated_at.unwrap_or_else(|| Utc::now()),
        }))
    }

    /// Find user by email
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let row = sqlx::query!(
            "SELECT id, email, username, created_at, updated_at FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.id,
            email: r.email,
            username: r.username,
            created_at: r.created_at.unwrap_or_else(|| Utc::now()),
            updated_at: r.updated_at.unwrap_or_else(|| Utc::now()),
        }))
    }

    /// Find user by username
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let row = sqlx::query!(
            "SELECT id, email, username, created_at, updated_at FROM users WHERE username = $1",
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.id,
            email: r.email,
            username: r.username,
            created_at: r.created_at.unwrap_or_else(|| Utc::now()),
            updated_at: r.updated_at.unwrap_or_else(|| Utc::now()),
        }))
    }

    /// Get user profile
    pub async fn get_profile(&self, user_id: Uuid) -> Result<Option<UserProfile>, AppError> {
        let row = sqlx::query!(
            "SELECT user_id, profile_data, updated_at FROM user_profiles WHERE user_id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| UserProfile {
            user_id: r.user_id,
            profile_data: r.profile_data,
            updated_at: r.updated_at.unwrap_or_else(|| Utc::now()),
        }))
    }

    /// Update user profile
    pub async fn update_profile(&self, user_id: Uuid, profile_data: ProfileData) -> Result<UserProfile, AppError> {
        let profile_json = serde_json::to_value(profile_data)?;
        
        let row = sqlx::query!(
            r#"
            UPDATE user_profiles 
            SET profile_data = $2, updated_at = NOW()
            WHERE user_id = $1
            RETURNING user_id, profile_data, updated_at
            "#,
            user_id,
            profile_json
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(UserProfile {
            user_id: row.user_id,
            profile_data: row.profile_data,
            updated_at: row.updated_at.unwrap_or_else(|| Utc::now()),
        })
    }

    /// Get user statistics
    pub async fn get_statistics(&self, user_id: Uuid) -> Result<Option<UserStatistics>, AppError> {
        let row = sqlx::query!(
            r#"
            SELECT user_id, total_attempts, total_ascents, personal_best_grade, statistics_data, updated_at 
            FROM user_statistics 
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| UserStatistics {
            user_id: r.user_id,
            total_attempts: r.total_attempts.unwrap_or(0),
            total_ascents: r.total_ascents.unwrap_or(0),
            personal_best_grade: r.personal_best_grade,
            statistics_data: r.statistics_data,
            updated_at: r.updated_at.unwrap_or_else(|| Utc::now()),
        }))
    }

    /// Update user statistics after an attempt
    pub async fn record_attempt(&self, user_id: Uuid, grade: &str, success: bool) -> Result<UserStatistics, AppError> {
        // Get current statistics
        let mut statistics = self.get_statistics(user_id).await?
            .ok_or_else(|| AppError::NotFound("User statistics not found".to_string()))?;

        // Update the statistics
        statistics.record_attempt(grade, success)?;

        // Save back to database
        let row = sqlx::query!(
            r#"
            UPDATE user_statistics 
            SET total_attempts = $2, total_ascents = $3, personal_best_grade = $4, 
                statistics_data = $5, updated_at = $6
            WHERE user_id = $1
            RETURNING user_id, total_attempts, total_ascents, personal_best_grade, statistics_data, updated_at
            "#,
            user_id,
            statistics.total_attempts,
            statistics.total_ascents,
            statistics.personal_best_grade,
            statistics.statistics_data,
            statistics.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(UserStatistics {
            user_id: row.user_id,
            total_attempts: row.total_attempts.unwrap_or(0),
            total_ascents: row.total_ascents.unwrap_or(0),
            personal_best_grade: row.personal_best_grade,
            statistics_data: row.statistics_data,
            updated_at: row.updated_at.unwrap_or_else(|| Utc::now()),
        })
    }

    /// Delete user and all related data
    pub async fn delete_user(&self, user_id: Uuid) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await?;

        // Delete user statistics (will cascade delete due to foreign key)
        sqlx::query!("DELETE FROM user_statistics WHERE user_id = $1", user_id)
            .execute(&mut *tx)
            .await?;

        // Delete user profile (will cascade delete due to foreign key)
        sqlx::query!("DELETE FROM user_profiles WHERE user_id = $1", user_id)
            .execute(&mut *tx)
            .await?;

        // Delete user (this will cascade delete other related data)
        sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    /// Check if email is already taken
    pub async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM users WHERE email = $1",
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.unwrap_or(0) > 0)
    }

    /// Check if username is already taken
    pub async fn username_exists(&self, username: &str) -> Result<bool, AppError> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM users WHERE username = $1",
            username
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.unwrap_or(0) > 0)
    }

    /// Get complete user data with profile and statistics
    pub async fn get_user_with_details(&self, user_id: Uuid) -> Result<Option<(User, UserProfile, UserStatistics)>, AppError> {
        let user = match self.find_by_id(user_id).await? {
            Some(user) => user,
            None => return Ok(None),
        };

        let profile = self.get_profile(user_id).await?
            .ok_or_else(|| AppError::NotFound("User profile not found".to_string()))?;

        let statistics = self.get_statistics(user_id).await?
            .ok_or_else(|| AppError::NotFound("User statistics not found".to_string()))?;

        Ok(Some((user, profile, statistics)))
    }
}

#[cfg(test)]
mod tests {
    // Note: These tests would require a test database setup
    // For now, they're just structure examples

    #[tokio::test]
    #[ignore] // Ignore until we have test database setup
    async fn test_create_user() {
        // This would require a test database
        // let pool = PgPool::connect("test_database_url").await.unwrap();
        // let repo = UserRepository::new(pool);
        // let user = repo.create_user("test@example.com".to_string(), "testuser".to_string(), None).await.unwrap();
        // assert_eq!(user.email, "test@example.com");
    }

    #[tokio::test]
    #[ignore]
    async fn test_find_user_by_email() {
        // Test implementation would go here
    }

    #[tokio::test]
    #[ignore]
    async fn test_record_attempt() {
        // Test implementation would go here
    }
}