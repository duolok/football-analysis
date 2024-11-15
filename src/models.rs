use serde::{Deserialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Event {
    pub event_id:   i32,
    pub event_timestamp: i64,
    pub event_type: EventType,
    pub event_data: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
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
