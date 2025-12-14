use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Represents the state and type of a hold on the Kilter board
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HoldState {
    /// Hold is not lit - not used in the problem
    NotUsed,
    /// Hold is lit and used in the problem with specific constraints
    Used(HoldType),
}

/// Types of holds when they are lit (used in the problem)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HoldType {
    /// Start hold - lit green, must be touched first to establish on problem
    /// Constraint: 1-2 start holds per problem, hand holds only
    Start,
    /// Foot hold - lit yellow, can only be touched with feet
    Foot,
    /// Hand hold - lit blue, can be touched with hands or feet
    Hand,
    /// Finish hold - lit pink, must be touched to complete the problem
    /// Constraint: 1-2 finish holds per problem, hand holds only
    Finish,
}

/// Configuration of all holds on the board for a specific problem
/// This represents the "matrix" of hold states across the entire board
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldConfiguration {
    /// Map of hold_id -> HoldState for all holds on the board
    /// Only holds that are "Used" need to be stored; missing entries are NotUsed
    pub holds: HashMap<String, HoldState>,
}

/// Main boulder problem entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BoulderProblem {
    pub id: Uuid,
    pub name: String,
    pub creator_id: Uuid,
    pub difficulty: String, // V-scale grade (V0, V1, V2, etc.)
    pub hold_configuration: serde_json::Value, // Stored as JSONB in database
    pub tags: Vec<String>,
    pub ascent_count: i32,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request/Response DTOs for API endpoints

/// Request to create a new boulder problem
#[derive(Debug, Deserialize)]
pub struct CreateBoulderProblemRequest {
    pub name: String,
    pub difficulty: String,
    pub hold_configuration: HoldConfiguration,
    pub tags: Option<Vec<String>>,
    pub is_published: Option<bool>,
}

/// Request to update an existing boulder problem
#[derive(Debug, Deserialize)]
pub struct UpdateBoulderProblemRequest {
    pub name: Option<String>,
    pub difficulty: Option<String>,
    pub hold_configuration: Option<HoldConfiguration>,
    pub tags: Option<Vec<String>>,
    pub is_published: Option<bool>,
}

/// Public boulder problem information (safe to expose)
#[derive(Debug, Serialize)]
pub struct PublicBoulderProblem {
    pub id: Uuid,
    pub name: String,
    pub creator_username: String, // Resolved from creator_id
    pub difficulty: String,
    pub hold_configuration: HoldConfiguration,
    pub tags: Vec<String>,
    pub ascent_count: i32,
    pub average_rating: Option<f64>, // Calculated from votes
    pub created_at: DateTime<Utc>,
}

/// Detailed boulder problem with additional metadata
#[derive(Debug, Serialize)]
pub struct BoulderProblemDetails {
    pub problem: PublicBoulderProblem,
    pub user_has_voted: bool,
    pub user_has_attempted: bool,
    pub user_has_completed: bool,
}

impl HoldConfiguration {
    /// Create a new empty hold configuration
    pub fn new() -> Self {
        Self {
            holds: HashMap::new(),
        }
    }

    /// Add a hold to the configuration
    pub fn add_hold(&mut self, hold_id: String, hold_state: HoldState) {
        match hold_state {
            HoldState::NotUsed => {
                // Remove from map if it exists (NotUsed is the default)
                self.holds.remove(&hold_id);
            }
            HoldState::Used(_) => {
                self.holds.insert(hold_id, hold_state);
            }
        }
    }

    /// Get the state of a specific hold
    pub fn get_hold_state(&self, hold_id: &str) -> HoldState {
        self.holds.get(hold_id).cloned().unwrap_or(HoldState::NotUsed)
    }

    /// Get all holds of a specific type
    pub fn get_holds_by_type(&self, hold_type: HoldType) -> Vec<String> {
        self.holds
            .iter()
            .filter_map(|(hold_id, state)| {
                if let HoldState::Used(ref t) = state {
                    if *t == hold_type {
                        Some(hold_id.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    /// Validate the hold configuration according to Kilter board rules
    pub fn validate(&self) -> Result<(), String> {
        let start_holds = self.get_holds_by_type(HoldType::Start);
        let finish_holds = self.get_holds_by_type(HoldType::Finish);

        // Check start holds constraint (1-2 start holds)
        if start_holds.is_empty() {
            return Err("Problem must have at least 1 start hold".to_string());
        }
        if start_holds.len() > 2 {
            return Err("Problem cannot have more than 2 start holds".to_string());
        }

        // Check finish holds constraint (1-2 finish holds)
        if finish_holds.is_empty() {
            return Err("Problem must have at least 1 finish hold".to_string());
        }
        if finish_holds.len() > 2 {
            return Err("Problem cannot have more than 2 finish holds".to_string());
        }

        // Ensure we have at least some holds to climb on
        let total_used_holds = self.holds.len();
        if total_used_holds < 2 {
            return Err("Problem must have at least 2 holds (start and finish)".to_string());
        }

        Ok(())
    }

    /// Get a summary of hold types for display
    pub fn get_hold_summary(&self) -> HoldSummary {
        let mut summary = HoldSummary::default();
        
        for state in self.holds.values() {
            if let HoldState::Used(hold_type) = state {
                match hold_type {
                    HoldType::Start => summary.start_holds += 1,
                    HoldType::Foot => summary.foot_holds += 1,
                    HoldType::Hand => summary.hand_holds += 1,
                    HoldType::Finish => summary.finish_holds += 1,
                }
            }
        }
        
        summary
    }
}

/// Summary of hold types in a configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HoldSummary {
    pub start_holds: i32,
    pub foot_holds: i32,
    pub hand_holds: i32,
    pub finish_holds: i32,
}

impl BoulderProblem {
    /// Create a new boulder problem
    pub fn new(
        name: String,
        creator_id: Uuid,
        difficulty: String,
        hold_configuration: HoldConfiguration,
        tags: Vec<String>,
    ) -> Result<Self, String> {
        // Validate the hold configuration
        hold_configuration.validate()?;

        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            creator_id,
            difficulty,
            hold_configuration: serde_json::to_value(hold_configuration)
                .map_err(|e| format!("Failed to serialize hold configuration: {}", e))?,
            tags,
            ascent_count: 0,
            is_published: false,
            created_at: now,
            updated_at: now,
        })
    }

    /// Get the hold configuration as a structured object
    pub fn get_hold_configuration(&self) -> Result<HoldConfiguration, serde_json::Error> {
        serde_json::from_value(self.hold_configuration.clone())
    }

    /// Update the hold configuration
    pub fn update_hold_configuration(&mut self, config: HoldConfiguration) -> Result<(), String> {
        // Validate the new configuration
        config.validate()?;
        
        self.hold_configuration = serde_json::to_value(config)
            .map_err(|e| format!("Failed to serialize hold configuration: {}", e))?;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Check if a grade is valid (V0-V17)
    pub fn is_valid_grade(grade: &str) -> bool {
        if !grade.starts_with('V') {
            return false;
        }
        
        if let Ok(num) = grade[1..].parse::<i32>() {
            (0..=17).contains(&num)
        } else {
            false
        }
    }
}

impl Default for HoldConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hold_configuration_creation() {
        let mut config = HoldConfiguration::new();
        
        // Add some holds
        config.add_hold("hold_1".to_string(), HoldState::Used(HoldType::Start));
        config.add_hold("hold_2".to_string(), HoldState::Used(HoldType::Hand));
        config.add_hold("hold_3".to_string(), HoldState::Used(HoldType::Finish));
        
        assert_eq!(config.holds.len(), 3);
        assert_eq!(config.get_hold_state("hold_1"), HoldState::Used(HoldType::Start));
        assert_eq!(config.get_hold_state("hold_4"), HoldState::NotUsed);
    }

    #[test]
    fn test_hold_configuration_validation_success() {
        let mut config = HoldConfiguration::new();
        config.add_hold("start_1".to_string(), HoldState::Used(HoldType::Start));
        config.add_hold("hand_1".to_string(), HoldState::Used(HoldType::Hand));
        config.add_hold("finish_1".to_string(), HoldState::Used(HoldType::Finish));
        
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_hold_configuration_validation_no_start() {
        let mut config = HoldConfiguration::new();
        config.add_hold("hand_1".to_string(), HoldState::Used(HoldType::Hand));
        config.add_hold("finish_1".to_string(), HoldState::Used(HoldType::Finish));
        
        assert!(config.validate().is_err());
        assert!(config.validate().unwrap_err().contains("start hold"));
    }

    #[test]
    fn test_hold_configuration_validation_too_many_starts() {
        let mut config = HoldConfiguration::new();
        config.add_hold("start_1".to_string(), HoldState::Used(HoldType::Start));
        config.add_hold("start_2".to_string(), HoldState::Used(HoldType::Start));
        config.add_hold("start_3".to_string(), HoldState::Used(HoldType::Start));
        config.add_hold("finish_1".to_string(), HoldState::Used(HoldType::Finish));
        
        assert!(config.validate().is_err());
        assert!(config.validate().unwrap_err().contains("more than 2 start holds"));
    }

    #[test]
    fn test_get_holds_by_type() {
        let mut config = HoldConfiguration::new();
        config.add_hold("start_1".to_string(), HoldState::Used(HoldType::Start));
        config.add_hold("hand_1".to_string(), HoldState::Used(HoldType::Hand));
        config.add_hold("hand_2".to_string(), HoldState::Used(HoldType::Hand));
        config.add_hold("finish_1".to_string(), HoldState::Used(HoldType::Finish));
        
        let hand_holds = config.get_holds_by_type(HoldType::Hand);
        assert_eq!(hand_holds.len(), 2);
        assert!(hand_holds.contains(&"hand_1".to_string()));
        assert!(hand_holds.contains(&"hand_2".to_string()));
    }

    #[test]
    fn test_hold_summary() {
        let mut config = HoldConfiguration::new();
        config.add_hold("start_1".to_string(), HoldState::Used(HoldType::Start));
        config.add_hold("hand_1".to_string(), HoldState::Used(HoldType::Hand));
        config.add_hold("hand_2".to_string(), HoldState::Used(HoldType::Hand));
        config.add_hold("foot_1".to_string(), HoldState::Used(HoldType::Foot));
        config.add_hold("finish_1".to_string(), HoldState::Used(HoldType::Finish));
        
        let summary = config.get_hold_summary();
        assert_eq!(summary.start_holds, 1);
        assert_eq!(summary.hand_holds, 2);
        assert_eq!(summary.foot_holds, 1);
        assert_eq!(summary.finish_holds, 1);
    }

    #[test]
    fn test_boulder_problem_creation() {
        let mut config = HoldConfiguration::new();
        config.add_hold("start_1".to_string(), HoldState::Used(HoldType::Start));
        config.add_hold("hand_1".to_string(), HoldState::Used(HoldType::Hand));
        config.add_hold("finish_1".to_string(), HoldState::Used(HoldType::Finish));
        
        let creator_id = Uuid::new_v4();
        let problem = BoulderProblem::new(
            "Test Problem".to_string(),
            creator_id,
            "V3".to_string(),
            config,
            vec!["overhang".to_string(), "crimpy".to_string()],
        );
        
        assert!(problem.is_ok());
        let problem = problem.unwrap();
        assert_eq!(problem.name, "Test Problem");
        assert_eq!(problem.creator_id, creator_id);
        assert_eq!(problem.difficulty, "V3");
        assert_eq!(problem.tags, vec!["overhang", "crimpy"]);
        assert!(!problem.is_published);
        assert_eq!(problem.ascent_count, 0);
    }

    #[test]
    fn test_valid_grades() {
        assert!(BoulderProblem::is_valid_grade("V0"));
        assert!(BoulderProblem::is_valid_grade("V5"));
        assert!(BoulderProblem::is_valid_grade("V17"));
        
        assert!(!BoulderProblem::is_valid_grade("V18"));
        assert!(!BoulderProblem::is_valid_grade("5.10a"));
        assert!(!BoulderProblem::is_valid_grade("VB"));
        assert!(!BoulderProblem::is_valid_grade(""));
    }

    #[test]
    fn test_hold_configuration_serialization() {
        let mut config = HoldConfiguration::new();
        config.add_hold("start_1".to_string(), HoldState::Used(HoldType::Start));
        config.add_hold("hand_1".to_string(), HoldState::Used(HoldType::Hand));
        
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: HoldConfiguration = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.holds.len(), deserialized.holds.len());
        assert_eq!(config.get_hold_state("start_1"), deserialized.get_hold_state("start_1"));
        assert_eq!(config.get_hold_state("hand_1"), deserialized.get_hold_state("hand_1"));
    }
}