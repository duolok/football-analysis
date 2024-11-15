mod models;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::models::{Event, EventType, RegistrationData, SessionPingData, MatchData};
use serde_json::from_str;

pub fn process_events(file_path: &str) {
    let file = File::open(file_path).expect("Could not open a file.");
    let reader = BufReader::new(file);
    let mut unique_event_id = HashSet::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line.");
        if let Ok(event) = from_str::<Event>(&line) {
            if unique_event_id.insert(event.event_id) {
                println!("Processing event: {:?}", event);
                handle_event(&event);
            } else {
                println!("Err: FOUND DUPLICATE {:?}", event.event_id);
            }
        }
    }
}

fn is_valid_event(event: &Event) -> bool {
    match event.event_type {
        EventType::Registration => event.event_data.get("user_id").is_some(),
        EventType::SessionPing => event.event_data.get("user_id").is_some(),
        EventType::Match => event.event_data.get("match_id").is_some(),
    }
}

fn handle_event(event: &Event) {
    match event.event_type {
        EventType::Registration => {
            if let Ok(reg_data) = serde_json::from_value::<RegistrationData>(event.event_data.clone()) {
                handle_registration(reg_data);
            }
        }
        EventType::SessionPing => {
            if let Ok(session_data) = serde_json::from_value::<SessionPingData>(event.event_data.clone()) {
                handle_session_ping(session_data);
            }
        }
        EventType::Match => {
            if let Ok(match_data) = serde_json::from_value::<MatchData>(event.event_data.clone()) {
                handle_match(match_data);
            }
        }

    }
}

fn handle_registration(data: RegistrationData) {
    println!("Registration event: {:?}", data);
}
fn handle_session_ping(data: SessionPingData) { 
    println!("SessionPingData: {:?}", data);

}

fn handle_match(data: MatchData) {
    println!("MatchData: {:?}", data);
}

fn main() {
    process_events("events.jsonl")
}
