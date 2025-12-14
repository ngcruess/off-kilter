use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc, Datelike};

/// Core user entity from the database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User profile data stored as JSONB
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserProfile {
    pub user_id: Uuid,
    pub profile_data: serde_json::Value,
    pub updated_at: DateTime<Utc>,
}

/// User climbing statistics stored as JSONB
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserStatistics {
    pub user_id: Uuid,
    pub total_attempts: i32,
    pub total_ascents: i32,
    pub personal_best_grade: Option<String>,
    pub statistics_data: serde_json::Value,
    pub updated_at: DateTime<Utc>,
}

/// Structured profile data that gets serialized to JSONB
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileData {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub location: Option<String>,
    pub preferred_units: Option<String>, // "metric" or "imperial"
    pub privacy_settings: PrivacySettings,
}

/// Privacy settings for user profiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    pub profile_visibility: String, // "public", "friends", "private"
    pub statistics_visibility: String,
    pub history_visibility: String,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            profile_visibility: "public".to_string(),
            statistics_visibility: "public".to_string(),
            history_visibility: "public".to_string(),
        }
    }
}

/// Extended statistics data that gets serialized to JSONB
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatisticsData {
    pub grade_distribution: std::collections::HashMap<String, i32>,
    pub monthly_progress: Vec<MonthlyProgress>,
    pub streak_records: Vec<StreakRecord>,
    pub milestones: Vec<Milestone>,
    pub favorite_problem_types: Vec<String>,
}

/// Monthly climbing progress tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyProgress {
    pub year: i32,
    pub month: i32,
    pub attempts: i32,
    pub ascents: i32,
    pub unique_problems: i32,
}

/// Streak tracking for various achievements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreakRecord {
    pub streak_type: String, // "daily_climb", "weekly_ascent", etc.
    pub current_count: i32,
    pub best_count: i32,
    pub start_date: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
}

/// Achievement milestones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String, // "ascents", "grades", "streaks", etc.
    pub achieved_at: DateTime<Utc>,
    pub value: i32, // The value that triggered the milestone
}

/// Request/Response DTOs for API endpoints

/// User registration request
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub profile: Option<ProfileData>,
}

/// User profile update request
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub profile: Option<ProfileData>,
}

/// Public user information (safe to expose)
#[derive(Debug, Serialize)]
pub struct PublicUser {
    pub id: Uuid,
    pub username: String,
    pub profile: ProfileData,
    pub statistics: PublicStatistics,
    pub created_at: DateTime<Utc>,
}

/// Public statistics (respects privacy settings)
#[derive(Debug, Serialize)]
pub struct PublicStatistics {
    pub total_attempts: Option<i32>,
    pub total_ascents: Option<i32>,
    pub personal_best_grade: Option<String>,
    pub grade_distribution: Option<std::collections::HashMap<String, i32>>,
}

/// Complete user data with all related information
#[derive(Debug, Serialize)]
pub struct UserWithDetails {
    pub user: User,
    pub profile: ProfileData,
    pub statistics: UserStatistics,
    pub statistics_data: StatisticsData,
}

impl User {
    /// Create a new user (for registration)
    pub fn new(email: String, username: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            username,
            created_at: now,
            updated_at: now,
        }
    }
}

impl UserProfile {
    /// Create a new user profile with default data
    pub fn new(user_id: Uuid, profile_data: Option<ProfileData>) -> Self {
        Self {
            user_id,
            profile_data: serde_json::to_value(profile_data.unwrap_or_default()).unwrap(),
            updated_at: Utc::now(),
        }
    }

    /// Get the structured profile data
    pub fn get_profile_data(&self) -> Result<ProfileData, serde_json::Error> {
        serde_json::from_value(self.profile_data.clone())
    }

    /// Update the profile data
    pub fn update_profile_data(&mut self, profile_data: ProfileData) -> Result<(), serde_json::Error> {
        self.profile_data = serde_json::to_value(profile_data)?;
        self.updated_at = Utc::now();
        Ok(())
    }
}

impl UserStatistics {
    /// Create new user statistics with default values
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            total_attempts: 0,
            total_ascents: 0,
            personal_best_grade: None,
            statistics_data: serde_json::to_value(StatisticsData::default()).unwrap(),
            updated_at: Utc::now(),
        }
    }

    /// Get the structured statistics data
    pub fn get_statistics_data(&self) -> Result<StatisticsData, serde_json::Error> {
        serde_json::from_value(self.statistics_data.clone())
    }

    /// Update statistics after an attempt
    pub fn record_attempt(&mut self, grade: &str, success: bool) -> Result<(), serde_json::Error> {
        self.total_attempts += 1;
        
        if success {
            self.total_ascents += 1;
            
            // Update personal best if this is a harder grade
            if self.personal_best_grade.is_none() || self.is_harder_grade(grade) {
                self.personal_best_grade = Some(grade.to_string());
            }
        }

        // Update extended statistics
        let mut stats_data = self.get_statistics_data()?;
        
        // Update grade distribution
        *stats_data.grade_distribution.entry(grade.to_string()).or_insert(0) += 1;
        
        // Update monthly progress
        let now = Utc::now();
        if let Some(current_month) = stats_data.monthly_progress.last_mut() {
            if current_month.year == now.year() && current_month.month == now.month() as i32 {
                current_month.attempts += 1;
                if success {
                    current_month.ascents += 1;
                }
            } else {
                // New month
                stats_data.monthly_progress.push(MonthlyProgress {
                    year: now.year(),
                    month: now.month() as i32,
                    attempts: 1,
                    ascents: if success { 1 } else { 0 },
                    unique_problems: 1,
                });
            }
        } else {
            // First entry
            stats_data.monthly_progress.push(MonthlyProgress {
                year: now.year(),
                month: now.month() as i32,
                attempts: 1,
                ascents: if success { 1 } else { 0 },
                unique_problems: 1,
            });
        }

        self.statistics_data = serde_json::to_value(stats_data)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Check if a grade is harder than the current personal best
    fn is_harder_grade(&self, grade: &str) -> bool {
        if let Some(current_best) = &self.personal_best_grade {
            // Simple V-scale comparison (V0 < V1 < V2 ... < V17)
            let current_num = current_best.trim_start_matches('V').parse::<i32>().unwrap_or(0);
            let new_num = grade.trim_start_matches('V').parse::<i32>().unwrap_or(0);
            new_num > current_num
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new("test@example.com".to_string(), "testuser".to_string());
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.username, "testuser");
        assert!(!user.id.is_nil());
    }

    #[test]
    fn test_user_profile_creation() {
        let user_id = Uuid::new_v4();
        let profile_data = ProfileData {
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            ..Default::default()
        };
        
        let profile = UserProfile::new(user_id, Some(profile_data.clone()));
        assert_eq!(profile.user_id, user_id);
        
        let retrieved_data = profile.get_profile_data().unwrap();
        assert_eq!(retrieved_data.first_name, profile_data.first_name);
        assert_eq!(retrieved_data.last_name, profile_data.last_name);
    }

    #[test]
    fn test_statistics_attempt_recording() {
        let user_id = Uuid::new_v4();
        let mut stats = UserStatistics::new(user_id);
        
        // Record a successful attempt
        stats.record_attempt("V3", true).unwrap();
        assert_eq!(stats.total_attempts, 1);
        assert_eq!(stats.total_ascents, 1);
        assert_eq!(stats.personal_best_grade, Some("V3".to_string()));
        
        // Record a failed attempt
        stats.record_attempt("V5", false).unwrap();
        assert_eq!(stats.total_attempts, 2);
        assert_eq!(stats.total_ascents, 1);
        assert_eq!(stats.personal_best_grade, Some("V3".to_string())); // Unchanged
        
        // Record a harder successful attempt
        stats.record_attempt("V4", true).unwrap();
        assert_eq!(stats.total_attempts, 3);
        assert_eq!(stats.total_ascents, 2);
        assert_eq!(stats.personal_best_grade, Some("V4".to_string())); // Updated
    }

    #[test]
    fn test_grade_comparison() {
        let user_id = Uuid::new_v4();
        let mut stats = UserStatistics::new(user_id);
        
        stats.record_attempt("V2", true).unwrap();
        assert!(stats.is_harder_grade("V3"));
        assert!(!stats.is_harder_grade("V1"));
        assert!(!stats.is_harder_grade("V2"));
    }
}