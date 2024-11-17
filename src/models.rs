use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Event {
    pub event_id:   i32,
    pub event_timestamp: i64,
    pub event_type: EventType,
    pub event_data: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Registration,
    SessionPing,
    Match
}

#[derive(Debug, Deserialize)]
pub struct RegistrationData {
    pub country: String,
    pub user_id: String,
    pub device_os: String,
}

#[derive(Debug, Deserialize)]
pub struct SessionPingData {
    pub user_id: String,
    pub session_type: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct MatchData {
    pub match_id: String,
    pub home_user_id: String,
    pub away_user_id: String,
    pub home_goals_scored: Option<i32>,
    pub away_goals_scored: Option<i32>,
}

#[derive(Serialize)]
pub struct UserLevelStats {
    pub country: String,
    pub registration_datetime: String,
    pub days_since_lasat_login: i64,
    pub session_count: i64,
    pub game_time: i64,
    pub match_points: i64,
    pub match_time_percentage: i64,
}

#[derive(Serialize)]
pub struct GameLevelStats {
    pub daily_active_users: i64,
    pub session_count: i64,
    pub avg_sessions: f64,
    pub top_users: Vec<String>,
}
