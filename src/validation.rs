use crate::models::{Event, EventType};

pub fn is_valid_event(event: &Event) -> bool {
    match &event.event_type {
        EventType::Registration => {
            event.event_data.get("user_id").is_some() 
            && event.event_data.get("country").is_some()
            && event.event_data.get("device_os").is_some()
        }
        EventType::SessionPing => {
            event.event_data.get("user_id").is_some()
        }
        EventType::Match => {
            event.event_data.get("match_id").is_some()
            && event.event_data.get("home_user_id").is_some()
            && event.event_data.get("away_user_id").is_some()
        }
    }
}
