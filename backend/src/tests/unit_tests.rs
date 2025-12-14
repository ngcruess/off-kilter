use crate::models::user::{User, UserProfile, UserStatistics, ProfileData, PrivacySettings};
use uuid::Uuid;
use chrono::{Utc, Datelike};

#[cfg(test)]
mod user_model_tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let email = "test@example.com".to_string();
        let username = "testuser".to_string();
        
        let user = User::new(email.clone(), username.clone());
        
        assert_eq!(user.email, email);
        assert_eq!(user.username, username);
        assert!(!user.id.is_nil());
        
        // Timestamps should be recent and equal
        let now = Utc::now();
        let time_diff = now.signed_duration_since(user.created_at);
        assert!(time_diff.num_seconds() >= 0 && time_diff.num_seconds() < 60);
        assert_eq!(user.created_at, user.updated_at);
    }

    #[test]
    fn test_user_profile_creation_with_default_data() {
        let user_id = Uuid::new_v4();
        let profile = UserProfile::new(user_id, None);
        
        assert_eq!(profile.user_id, user_id);
        
        let profile_data = profile.get_profile_data().unwrap();
        assert_eq!(profile_data.first_name, None);
        assert_eq!(profile_data.last_name, None);
        assert_eq!(profile_data.privacy_settings.profile_visibility, "public");
        assert_eq!(profile_data.privacy_settings.statistics_visibility, "public");
        assert_eq!(profile_data.privacy_settings.history_visibility, "public");
    }

    #[test]
    fn test_user_profile_creation_with_custom_data() {
        let user_id = Uuid::new_v4();
        let custom_profile = ProfileData {
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            display_name: Some("JohnD".to_string()),
            bio: Some("Climbing enthusiast".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            location: Some("Boulder, CO".to_string()),
            preferred_units: Some("imperial".to_string()),
            privacy_settings: PrivacySettings {
                profile_visibility: "friends".to_string(),
                statistics_visibility: "private".to_string(),
                history_visibility: "friends".to_string(),
            },
        };
        
        let profile = UserProfile::new(user_id, Some(custom_profile.clone()));
        
        assert_eq!(profile.user_id, user_id);
        
        let retrieved_data = profile.get_profile_data().unwrap();
        assert_eq!(retrieved_data.first_name, custom_profile.first_name);
        assert_eq!(retrieved_data.last_name, custom_profile.last_name);
        assert_eq!(retrieved_data.display_name, custom_profile.display_name);
        assert_eq!(retrieved_data.bio, custom_profile.bio);
        assert_eq!(retrieved_data.avatar_url, custom_profile.avatar_url);
        assert_eq!(retrieved_data.location, custom_profile.location);
        assert_eq!(retrieved_data.preferred_units, custom_profile.preferred_units);
        assert_eq!(retrieved_data.privacy_settings.profile_visibility, "friends");
        assert_eq!(retrieved_data.privacy_settings.statistics_visibility, "private");
        assert_eq!(retrieved_data.privacy_settings.history_visibility, "friends");
    }

    #[test]
    fn test_user_profile_update() {
        let user_id = Uuid::new_v4();
        let mut profile = UserProfile::new(user_id, None);
        
        let original_timestamp = profile.updated_at;
        
        // Wait a small amount to ensure timestamp difference
        std::thread::sleep(std::time::Duration::from_millis(1));
        
        let updated_data = ProfileData {
            first_name: Some("Jane".to_string()),
            last_name: Some("Smith".to_string()),
            display_name: Some("JaneS".to_string()),
            bio: Some("Rock climbing coach".to_string()),
            avatar_url: None,
            location: Some("Seattle, WA".to_string()),
            preferred_units: Some("metric".to_string()),
            privacy_settings: PrivacySettings {
                profile_visibility: "public".to_string(),
                statistics_visibility: "friends".to_string(),
                history_visibility: "private".to_string(),
            },
        };
        
        profile.update_profile_data(updated_data.clone()).unwrap();
        
        // Timestamp should be updated
        assert!(profile.updated_at > original_timestamp);
        
        // Data should be updated
        let retrieved_data = profile.get_profile_data().unwrap();
        assert_eq!(retrieved_data.first_name, updated_data.first_name);
        assert_eq!(retrieved_data.last_name, updated_data.last_name);
        assert_eq!(retrieved_data.display_name, updated_data.display_name);
        assert_eq!(retrieved_data.bio, updated_data.bio);
        assert_eq!(retrieved_data.location, updated_data.location);
        assert_eq!(retrieved_data.preferred_units, updated_data.preferred_units);
        assert_eq!(retrieved_data.privacy_settings.profile_visibility, "public");
        assert_eq!(retrieved_data.privacy_settings.statistics_visibility, "friends");
        assert_eq!(retrieved_data.privacy_settings.history_visibility, "private");
    }

    #[test]
    fn test_user_statistics_creation() {
        let user_id = Uuid::new_v4();
        let statistics = UserStatistics::new(user_id);
        
        assert_eq!(statistics.user_id, user_id);
        assert_eq!(statistics.total_attempts, 0);
        assert_eq!(statistics.total_ascents, 0);
        assert_eq!(statistics.personal_best_grade, None);
        
        let stats_data = statistics.get_statistics_data().unwrap();
        assert!(stats_data.grade_distribution.is_empty());
        assert!(stats_data.monthly_progress.is_empty());
        assert!(stats_data.streak_records.is_empty());
        assert!(stats_data.milestones.is_empty());
        assert!(stats_data.favorite_problem_types.is_empty());
    }

    #[test]
    fn test_statistics_attempt_recording_success() {
        let user_id = Uuid::new_v4();
        let mut statistics = UserStatistics::new(user_id);
        
        // Record a successful attempt
        statistics.record_attempt("V3", true).unwrap();
        
        assert_eq!(statistics.total_attempts, 1);
        assert_eq!(statistics.total_ascents, 1);
        assert_eq!(statistics.personal_best_grade, Some("V3".to_string()));
        
        let stats_data = statistics.get_statistics_data().unwrap();
        assert_eq!(stats_data.grade_distribution.get("V3"), Some(&1));
        assert_eq!(stats_data.monthly_progress.len(), 1);
        
        let monthly = &stats_data.monthly_progress[0];
        assert_eq!(monthly.attempts, 1);
        assert_eq!(monthly.ascents, 1);
        assert_eq!(monthly.unique_problems, 1);
    }

    #[test]
    fn test_statistics_attempt_recording_failure() {
        let user_id = Uuid::new_v4();
        let mut statistics = UserStatistics::new(user_id);
        
        // Record a failed attempt
        statistics.record_attempt("V5", false).unwrap();
        
        assert_eq!(statistics.total_attempts, 1);
        assert_eq!(statistics.total_ascents, 0);
        assert_eq!(statistics.personal_best_grade, None);
        
        let stats_data = statistics.get_statistics_data().unwrap();
        assert_eq!(stats_data.grade_distribution.get("V5"), Some(&1));
        assert_eq!(stats_data.monthly_progress.len(), 1);
        
        let monthly = &stats_data.monthly_progress[0];
        assert_eq!(monthly.attempts, 1);
        assert_eq!(monthly.ascents, 0);
        assert_eq!(monthly.unique_problems, 1);
    }

    #[test]
    fn test_statistics_grade_progression() {
        let user_id = Uuid::new_v4();
        let mut statistics = UserStatistics::new(user_id);
        
        // Record attempts in various orders
        statistics.record_attempt("V2", true).unwrap();
        assert_eq!(statistics.personal_best_grade, Some("V2".to_string()));
        
        statistics.record_attempt("V1", true).unwrap();
        assert_eq!(statistics.personal_best_grade, Some("V2".to_string())); // Should not decrease
        
        statistics.record_attempt("V4", true).unwrap();
        assert_eq!(statistics.personal_best_grade, Some("V4".to_string())); // Should increase
        
        statistics.record_attempt("V3", true).unwrap();
        assert_eq!(statistics.personal_best_grade, Some("V4".to_string())); // Should not decrease
        
        statistics.record_attempt("V6", false).unwrap();
        assert_eq!(statistics.personal_best_grade, Some("V4".to_string())); // Failed attempts don't affect PB
        
        statistics.record_attempt("V5", true).unwrap();
        assert_eq!(statistics.personal_best_grade, Some("V5".to_string())); // Should increase
        
        // Verify final statistics
        assert_eq!(statistics.total_attempts, 6);
        assert_eq!(statistics.total_ascents, 5);
        
        let stats_data = statistics.get_statistics_data().unwrap();
        assert_eq!(stats_data.grade_distribution.get("V1"), Some(&1));
        assert_eq!(stats_data.grade_distribution.get("V2"), Some(&1));
        assert_eq!(stats_data.grade_distribution.get("V3"), Some(&1));
        assert_eq!(stats_data.grade_distribution.get("V4"), Some(&1));
        assert_eq!(stats_data.grade_distribution.get("V5"), Some(&1));
        assert_eq!(stats_data.grade_distribution.get("V6"), Some(&1));
    }

    #[test]
    fn test_statistics_multiple_attempts_same_grade() {
        let user_id = Uuid::new_v4();
        let mut statistics = UserStatistics::new(user_id);
        
        // Record multiple attempts on the same grade
        statistics.record_attempt("V3", false).unwrap();
        statistics.record_attempt("V3", false).unwrap();
        statistics.record_attempt("V3", true).unwrap();
        statistics.record_attempt("V3", true).unwrap();
        
        assert_eq!(statistics.total_attempts, 4);
        assert_eq!(statistics.total_ascents, 2);
        assert_eq!(statistics.personal_best_grade, Some("V3".to_string()));
        
        let stats_data = statistics.get_statistics_data().unwrap();
        assert_eq!(stats_data.grade_distribution.get("V3"), Some(&4));
    }

    #[test]
    fn test_grade_comparison_logic() {
        let user_id = Uuid::new_v4();
        let mut statistics = UserStatistics::new(user_id);
        
        // Test the internal grade comparison logic through behavior
        statistics.record_attempt("V0", true).unwrap();
        assert_eq!(statistics.personal_best_grade, Some("V0".to_string()));
        
        statistics.record_attempt("V10", true).unwrap();
        assert_eq!(statistics.personal_best_grade, Some("V10".to_string()));
        
        statistics.record_attempt("V5", true).unwrap();
        assert_eq!(statistics.personal_best_grade, Some("V10".to_string())); // Should not decrease
        
        statistics.record_attempt("V17", true).unwrap();
        assert_eq!(statistics.personal_best_grade, Some("V17".to_string()));
    }

    #[test]
    fn test_monthly_progress_tracking() {
        let user_id = Uuid::new_v4();
        let mut statistics = UserStatistics::new(user_id);
        
        // Record several attempts
        statistics.record_attempt("V2", true).unwrap();
        statistics.record_attempt("V3", false).unwrap();
        statistics.record_attempt("V2", true).unwrap();
        
        let stats_data = statistics.get_statistics_data().unwrap();
        assert_eq!(stats_data.monthly_progress.len(), 1);
        
        let monthly = &stats_data.monthly_progress[0];
        assert_eq!(monthly.attempts, 3);
        assert_eq!(monthly.ascents, 2);
        assert_eq!(monthly.unique_problems, 1); // This is a simplified implementation
        
        let now = Utc::now();
        assert_eq!(monthly.year, now.year());
        assert_eq!(monthly.month, now.month() as i32);
    }

    #[test]
    fn test_privacy_settings_default() {
        let privacy = PrivacySettings::default();
        
        assert_eq!(privacy.profile_visibility, "public");
        assert_eq!(privacy.statistics_visibility, "public");
        assert_eq!(privacy.history_visibility, "public");
    }

    #[test]
    fn test_profile_data_default() {
        let profile = ProfileData::default();
        
        assert_eq!(profile.first_name, None);
        assert_eq!(profile.last_name, None);
        assert_eq!(profile.display_name, None);
        assert_eq!(profile.bio, None);
        assert_eq!(profile.avatar_url, None);
        assert_eq!(profile.location, None);
        assert_eq!(profile.preferred_units, None);
        assert_eq!(profile.privacy_settings.profile_visibility, "public");
    }
}

#[cfg(test)]
mod user_validation_tests {
    use super::*;

    #[test]
    fn test_user_id_uniqueness() {
        let user1 = User::new("user1@example.com".to_string(), "user1".to_string());
        let user2 = User::new("user2@example.com".to_string(), "user2".to_string());
        
        assert_ne!(user1.id, user2.id);
    }

    #[test]
    fn test_profile_data_serialization_roundtrip() {
        let original = ProfileData {
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            display_name: Some("TestUser".to_string()),
            bio: Some("Test bio with special chars: àáâãäå".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            location: Some("Test City, Test State".to_string()),
            preferred_units: Some("metric".to_string()),
            privacy_settings: PrivacySettings {
                profile_visibility: "friends".to_string(),
                statistics_visibility: "private".to_string(),
                history_visibility: "public".to_string(),
            },
        };
        
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ProfileData = serde_json::from_str(&json).unwrap();
        
        assert_eq!(original.first_name, deserialized.first_name);
        assert_eq!(original.last_name, deserialized.last_name);
        assert_eq!(original.display_name, deserialized.display_name);
        assert_eq!(original.bio, deserialized.bio);
        assert_eq!(original.avatar_url, deserialized.avatar_url);
        assert_eq!(original.location, deserialized.location);
        assert_eq!(original.preferred_units, deserialized.preferred_units);
        assert_eq!(original.privacy_settings.profile_visibility, deserialized.privacy_settings.profile_visibility);
        assert_eq!(original.privacy_settings.statistics_visibility, deserialized.privacy_settings.statistics_visibility);
        assert_eq!(original.privacy_settings.history_visibility, deserialized.privacy_settings.history_visibility);
    }

    #[test]
    fn test_statistics_data_serialization_roundtrip() {
        let user_id = Uuid::new_v4();
        let mut statistics = UserStatistics::new(user_id);
        
        // Add some data
        statistics.record_attempt("V3", true).unwrap();
        statistics.record_attempt("V4", false).unwrap();
        statistics.record_attempt("V5", true).unwrap();
        
        let json = serde_json::to_string(&statistics).unwrap();
        let deserialized: UserStatistics = serde_json::from_str(&json).unwrap();
        
        assert_eq!(statistics.user_id, deserialized.user_id);
        assert_eq!(statistics.total_attempts, deserialized.total_attempts);
        assert_eq!(statistics.total_ascents, deserialized.total_ascents);
        assert_eq!(statistics.personal_best_grade, deserialized.personal_best_grade);
        
        let original_data = statistics.get_statistics_data().unwrap();
        let deserialized_data = deserialized.get_statistics_data().unwrap();
        
        assert_eq!(original_data.grade_distribution, deserialized_data.grade_distribution);
        assert_eq!(original_data.monthly_progress.len(), deserialized_data.monthly_progress.len());
    }
}