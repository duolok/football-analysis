mod models;

use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::models::{Event, EventType, RegistrationData, SessionPingData, MatchData};
use serde_json::from_str;

const THREAD_NUM: usize = 4;

pub fn process_events(file_path: &str) {
    let file = File::open(file_path).expect("Could not open a file.");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
        .map(|line| line.expect("Could not read line."))
        .collect();

    let mut unique_event_id = Arc::new(Mutex::new(HashSet::new()));
    let chunk_size = (lines.len() + THREAD_NUM - 1) / THREAD_NUM;

    let mut handles = vec![];

    for chunk in lines.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let unique_event_id = Arc::clone(&unique_event_id);

        let handle = thread::spawn(move || {
            for line in chunk {
                if let Ok(event) = from_str::<Event>(&line) {
                    let mut unique_event_id = unique_event_id.lock().unwrap();
                    if unique_event_id.insert(event.event_id) {
                        handle_event(&event);
                    } else {
                        println!("ERR: Found Duplicate Event {:?}", event.event_id);
                    }
                }
            }
        });

        handles.push(handle)
    }

    for handle in handles {
        handle.join().expect("Thread panicked.");
    }
}

fn is_valid_event(event: &Event) -> bool {
    match event.event_type {
        EventType::Registration => {
            event.event_data.get("user_id").is_some() 
            && event.event_data.get("country").is_some()
            && event.event_data.get("device_os").is_some()

        } 
        EventType::SessionPing => {
            event.event_data.get("user_id").is_some()
        },
        EventType::Match => {
            event.event_data.get("match_id").is_some() 
            && event.event_data.get("home_user_id").is_some()
            && event.event_data.get("away_user_id").is_some()
        }
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
