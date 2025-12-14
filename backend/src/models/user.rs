use serde::{Deserialize, Serialize};
// use sqlx::FromRow; // Will be added when we add SQLx
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub profile: serde_json::Value,
    pub statistics: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClimbingStatistics {
    pub total_attempts: i32,
    pub total_ascents: i32,
    pub personal_best_grade: String,
    pub streak_records: Vec<StreakRecord>,
    pub milestones: Vec<Milestone>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreakRecord {
    pub streak_type: String,
    pub count: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Milestone {
    pub id: String,
    pub name: String,
    pub description: String,
    pub achieved_at: DateTime<Utc>,
}