use proptest::prelude::*;
use uuid::Uuid;
use crate::models::user::{User, UserProfile, UserStatistics, ProfileData, PrivacySettings};

/// Property Test 1: User Account Lifecycle Consistency
/// 
/// This property test validates that user account operations maintain consistency
/// across the entire lifecycle: creation → profile updates → statistics tracking → deletion
/// 
/// Properties tested:
/// - User creation always produces valid, unique users
/// - Profile updates preserve user identity while allowing data changes
/// - Statistics tracking maintains mathematical consistency
/// - All operations preserve data integrity constraints
#[cfg(test)]
mod user_account_lifecycle {
    use super::*;
    use std::collections::HashMap;

    // Strategy for generating valid email addresses
    fn email_strategy() -> impl Strategy<Value = String> {
        prop::string::string_regex(r"[a-z]{3,10}@[a-z]{3,8}\.(com|org|net)")
            .expect("Valid email regex")
    }

    // Strategy for generating valid usernames
    fn username_strategy() -> impl Strategy<Value = String> {
        prop::string::string_regex(r"[a-zA-Z0-9_]{3,20}")
            .expect("Valid username regex")
    }

    // Strategy for generating profile data
    fn profile_data_strategy() -> impl Strategy<Value = ProfileData> {
        (
            prop::option::of(prop::string::string_regex(r"[A-Za-z]{2,20}").unwrap()),
            prop::option::of(prop::string::string_regex(r"[A-Za-z]{2,30}").unwrap()),
            prop::option::of(prop::string::string_regex(r"[A-Za-z0-9_]{3,30}").unwrap()),
            prop::option::of(prop::string::string_regex(r".{10,200}").unwrap()),
            prop::option::of(prop::string::string_regex(r"https?://[a-z0-9.-]+/[a-z0-9._-]+").unwrap()),
            prop::option::of(prop::string::string_regex(r"[A-Za-z\s]{3,50}").unwrap()),
            prop::option::of(prop::string::string_regex(r"(metric|imperial)").unwrap()),
            privacy_settings_strategy(),
        ).prop_map(|(first_name, last_name, display_name, bio, avatar_url, location, preferred_units, privacy_settings)| {
            ProfileData {
                first_name,
                last_name,
                display_name,
                bio,
                avatar_url,
                location,
                preferred_units,
                privacy_settings,
            }
        })
    }

    // Strategy for generating privacy settings
    fn privacy_settings_strategy() -> impl Strategy<Value = PrivacySettings> {
        (
            prop::string::string_regex(r"(public|friends|private)").unwrap(),
            prop::string::string_regex(r"(public|friends|private)").unwrap(),
            prop::string::string_regex(r"(public|friends|private)").unwrap(),
        ).prop_map(|(profile_visibility, statistics_visibility, history_visibility)| {
            PrivacySettings {
                profile_visibility,
                statistics_visibility,
                history_visibility,
            }
        })
    }

    // Strategy for generating climbing grades
    fn grade_strategy() -> impl Strategy<Value = String> {
        (0..18u8).prop_map(|n| format!("V{}", n))
    }

    proptest! {
        /// Property 1: User Account Lifecycle Consistency
        /// 
        /// Tests that user account operations maintain consistency throughout
        /// the entire lifecycle from creation to deletion.
        #[test]
        fn user_account_lifecycle_consistency(
            email in email_strategy(),
            username in username_strategy(),
            initial_profile in profile_data_strategy(),
            updated_profile in profile_data_strategy(),
            attempts in prop::collection::vec((grade_strategy(), any::<bool>()), 0..20)
        ) {
            // Property 1.1: User creation produces valid users
            let user = User::new(email.clone(), username.clone());
            
            // User should have valid UUID
            prop_assert!(!user.id.is_nil());
            
            // User should preserve input data
            prop_assert_eq!(user.email, email);
            prop_assert_eq!(user.username, username);
            
            // Timestamps should be reasonable (within last minute)
            let now = chrono::Utc::now();
            let time_diff = now.signed_duration_since(user.created_at);
            prop_assert!(time_diff.num_seconds() >= 0 && time_diff.num_seconds() < 60);
            prop_assert_eq!(user.created_at, user.updated_at);

            // Property 1.2: Profile creation and updates maintain consistency
            let mut profile = UserProfile::new(user.id, Some(initial_profile.clone()));
            
            // Profile should be linked to user
            prop_assert_eq!(profile.user_id, user.id);
            
            // Profile data should be retrievable
            let retrieved_profile = profile.get_profile_data().unwrap();
            prop_assert_eq!(retrieved_profile.first_name, initial_profile.first_name);
            prop_assert_eq!(retrieved_profile.privacy_settings.profile_visibility, 
                          initial_profile.privacy_settings.profile_visibility);

            // Profile updates should work
            let old_timestamp = profile.updated_at;
            std::thread::sleep(std::time::Duration::from_millis(1)); // Ensure timestamp difference
            profile.update_profile_data(updated_profile.clone()).unwrap();
            
            // Updated profile should have new data and timestamp
            let updated_retrieved = profile.get_profile_data().unwrap();
            prop_assert_eq!(updated_retrieved.display_name, updated_profile.display_name);
            prop_assert!(profile.updated_at > old_timestamp);

            // Property 1.3: Statistics tracking maintains mathematical consistency
            let mut statistics = UserStatistics::new(user.id);
            
            // Initial statistics should be zero
            prop_assert_eq!(statistics.total_attempts, 0);
            prop_assert_eq!(statistics.total_ascents, 0);
            prop_assert_eq!(&statistics.personal_best_grade, &None);
            
            // Track attempts and verify mathematical consistency
            let mut expected_attempts = 0;
            let mut expected_ascents = 0;
            let mut expected_best_grade: Option<String> = None;
            let mut grade_distribution: HashMap<String, i32> = HashMap::new();
            
            for (grade, success) in attempts {
                statistics.record_attempt(&grade, success).unwrap();
                expected_attempts += 1;
                
                if success {
                    expected_ascents += 1;
                    
                    // Update expected best grade
                    if expected_best_grade.is_none() || is_harder_grade(&grade, expected_best_grade.as_ref().unwrap()) {
                        expected_best_grade = Some(grade.clone());
                    }
                }
                
                // Update expected grade distribution
                *grade_distribution.entry(grade).or_insert(0) += 1;
                
                // Verify statistics consistency after each attempt
                prop_assert_eq!(statistics.total_attempts, expected_attempts);
                prop_assert_eq!(statistics.total_ascents, expected_ascents);
                prop_assert_eq!(&statistics.personal_best_grade, &expected_best_grade);
                
                // Verify grade distribution
                let stats_data = statistics.get_statistics_data().unwrap();
                for (grade, count) in &grade_distribution {
                    prop_assert_eq!(stats_data.grade_distribution.get(grade), Some(count));
                }
            }

            // Property 1.4: Data integrity constraints are maintained
            // User ID should remain consistent across all related entities
            prop_assert_eq!(user.id, profile.user_id);
            prop_assert_eq!(user.id, statistics.user_id);
            
            // Timestamps should be monotonic (newer operations have later timestamps)
            prop_assert!(statistics.updated_at >= user.created_at);
        }

        /// Property 2: Profile Privacy Settings Consistency
        /// 
        /// Tests that privacy settings are consistently applied and respected
        /// across all profile operations.
        #[test]
        fn profile_privacy_consistency(
            profile_data in profile_data_strategy(),
            visibility_changes in prop::collection::vec(
                (
                    prop::string::string_regex(r"(public|friends|private)").unwrap(),
                    prop::string::string_regex(r"(public|friends|private)").unwrap(),
                    prop::string::string_regex(r"(public|friends|private)").unwrap(),
                ), 1..5
            )
        ) {
            let user_id = Uuid::new_v4();
            let mut profile = UserProfile::new(user_id, Some(profile_data));
            
            // Test privacy setting changes maintain consistency
            for (profile_vis, stats_vis, history_vis) in visibility_changes {
                let mut updated_data = profile.get_profile_data().unwrap();
                updated_data.privacy_settings = PrivacySettings {
                    profile_visibility: profile_vis.clone(),
                    statistics_visibility: stats_vis.clone(),
                    history_visibility: history_vis.clone(),
                };
                
                profile.update_profile_data(updated_data).unwrap();
                let retrieved = profile.get_profile_data().unwrap();
                
                // Privacy settings should be exactly as set
                prop_assert_eq!(retrieved.privacy_settings.profile_visibility, profile_vis);
                prop_assert_eq!(retrieved.privacy_settings.statistics_visibility, stats_vis);
                prop_assert_eq!(retrieved.privacy_settings.history_visibility, history_vis);
            }
        }

        /// Property 3: Statistics Grade Progression Consistency
        /// 
        /// Tests that grade progression tracking maintains logical consistency
        /// and mathematical accuracy.
        #[test]
        fn statistics_grade_progression_consistency(
            attempts in prop::collection::vec((grade_strategy(), any::<bool>()), 1..50)
        ) {
            let user_id = Uuid::new_v4();
            let mut statistics = UserStatistics::new(user_id);
            
            let mut max_grade_achieved: Option<String> = None;
            
            for (grade, success) in attempts {
                let old_best = statistics.personal_best_grade.clone();
                statistics.record_attempt(&grade, success).unwrap();
                
                if success {
                    // Update our tracking of max grade
                    if max_grade_achieved.is_none() || is_harder_grade(&grade, max_grade_achieved.as_ref().unwrap()) {
                        max_grade_achieved = Some(grade.clone());
                    }
                    
                    // Personal best should never decrease
                    if let Some(old) = &old_best {
                        if let Some(new) = &statistics.personal_best_grade {
                            prop_assert!(is_harder_grade(new, old) || new == old, 
                                       "Personal best grade should never decrease: {} -> {}", old, new);
                        }
                    }
                    
                    // Personal best should match our max grade tracking
                    prop_assert_eq!(&statistics.personal_best_grade, &max_grade_achieved);
                }
                
                // Attempts should always be >= ascents
                prop_assert!(statistics.total_attempts >= statistics.total_ascents);
                
                // Statistics should be non-negative
                prop_assert!(statistics.total_attempts >= 0);
                prop_assert!(statistics.total_ascents >= 0);
            }
        }
    }

    // Helper function to compare grades (V0 < V1 < V2 ... < V17)
    fn is_harder_grade(grade1: &str, grade2: &str) -> bool {
        let num1 = grade1.trim_start_matches('V').parse::<i32>().unwrap_or(0);
        let num2 = grade2.trim_start_matches('V').parse::<i32>().unwrap_or(0);
        num1 > num2
    }
}

#[cfg(test)]
mod json_serialization_tests {
    use super::*;
    use serde_json;

    proptest! {
        /// Property 4: JSON Serialization Round-Trip Consistency
        /// 
        /// Tests that all user-related data structures can be serialized to JSON
        /// and deserialized back without data loss.
        #[test]
        fn json_serialization_roundtrip(
            email in prop::string::string_regex(r"[a-z]{3,10}@[a-z]{3,8}\.(com|org|net)").unwrap(),
            username in prop::string::string_regex(r"[a-zA-Z0-9_]{3,20}").unwrap(),
            profile_data in any::<ProfileData>(),
        ) {
            // Test User serialization
            let user = User::new(email, username);
            let user_json = serde_json::to_string(&user).unwrap();
            let user_deserialized: User = serde_json::from_str(&user_json).unwrap();
            
            prop_assert_eq!(user.id, user_deserialized.id);
            prop_assert_eq!(user.email, user_deserialized.email);
            prop_assert_eq!(user.username, user_deserialized.username);

            // Test ProfileData serialization
            let profile_json = serde_json::to_string(&profile_data).unwrap();
            let profile_deserialized: ProfileData = serde_json::from_str(&profile_json).unwrap();
            
            prop_assert_eq!(profile_data.first_name, profile_deserialized.first_name);
            prop_assert_eq!(profile_data.privacy_settings.profile_visibility, 
                          profile_deserialized.privacy_settings.profile_visibility);

            // Test UserStatistics serialization
            let statistics = UserStatistics::new(user.id);
            let stats_json = serde_json::to_string(&statistics).unwrap();
            let stats_deserialized: UserStatistics = serde_json::from_str(&stats_json).unwrap();
            
            prop_assert_eq!(statistics.user_id, stats_deserialized.user_id);
            prop_assert_eq!(statistics.total_attempts, stats_deserialized.total_attempts);
        }
    }
}

// Custom strategy implementations for complex types
impl Arbitrary for ProfileData {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        (
            prop::option::of(prop::string::string_regex(r"[A-Za-z]{2,20}").unwrap()),
            prop::option::of(prop::string::string_regex(r"[A-Za-z]{2,30}").unwrap()),
            prop::option::of(prop::string::string_regex(r"[A-Za-z0-9_]{3,30}").unwrap()),
            prop::option::of(prop::string::string_regex(r".{10,200}").unwrap()),
            prop::option::of(prop::string::string_regex(r"https?://[a-z0-9.-]+/[a-z0-9._-]+").unwrap()),
            prop::option::of(prop::string::string_regex(r"[A-Za-z\s]{3,50}").unwrap()),
            prop::option::of(prop::string::string_regex(r"(metric|imperial)").unwrap()),
            any::<PrivacySettings>(),
        ).prop_map(|(first_name, last_name, display_name, bio, avatar_url, location, preferred_units, privacy_settings)| {
            ProfileData {
                first_name,
                last_name,
                display_name,
                bio,
                avatar_url,
                location,
                preferred_units,
                privacy_settings,
            }
        }).boxed()
    }
}

impl Arbitrary for PrivacySettings {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        (
            prop::string::string_regex(r"(public|friends|private)").unwrap(),
            prop::string::string_regex(r"(public|friends|private)").unwrap(),
            prop::string::string_regex(r"(public|friends|private)").unwrap(),
        ).prop_map(|(profile_visibility, statistics_visibility, history_visibility)| {
            PrivacySettings {
                profile_visibility,
                statistics_visibility,
                history_visibility,
            }
        }).boxed()
    }
}