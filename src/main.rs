mod models;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::sync::Arc;
use crate::models::{Event, EventType, RegistrationData, SessionPingData, MatchData};
use std::env;
use dotenv::dotenv;
use serde_json::from_str;
use sqlx::{PgPool, query};
use tokio::sync::Mutex;

const THREAD_NUM: usize = 4;

pub async fn process_events(file_path: &str, pool: Arc<PgPool>) {
    let file = File::open(file_path).expect("Could not open file.");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
        .map(|line| line.expect("Could not read line."))
        .collect();

    let unique_event_id = Arc::new(Mutex::new(HashSet::new()));
    let chunk_size = (lines.len() + 4 - 1) / 4;

    let mut handles = vec![];

    for chunk in lines.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let unique_event_id = Arc::clone(&unique_event_id);
        let pool = Arc::clone(&pool);

        let handle = tokio::spawn(async move {
            for line in chunk {
                if let Ok(event) = from_str::<Event>(&line) {
                    if is_valid_event(&event) {
                        let mut unique_event_id = unique_event_id.lock().await;
                        if unique_event_id.insert(event.event_id) {
                            drop(unique_event_id); // Release lock
                            handle_event(event, &pool).await;
                        } else {
                            eprintln!("Duplicate event ID: {:?}", event.event_id);
                        }
                    }
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.expect("Task panicked.");
    }
}

fn is_valid_event(event: &Event) -> bool {
    match &event.event_type {
        EventType::Registration => {
            event.event_data.get("user_id").is_some() 
                && event.event_data.get("country").is_some()
                && event.event_data.get("device_os").is_some()
        }
        EventType::SessionPing => event.event_data.get("user_id").is_some(),
        EventType::Match => {
            event.event_data.get("match_id").is_some()
                && event.event_data.get("home_user_id").is_some()
                && event.event_data.get("away_user_id").is_some()
        }
    }
}

async fn handle_event(event: Event, pool: &PgPool) {
    match event.event_type {
        EventType::Registration => {
            if let Ok(data) = serde_json::from_value::<RegistrationData>(event.event_data) {
                handle_registration(data, pool, event.event_timestamp).await.unwrap();
            }
        }
        EventType::SessionPing => {
            if let Ok(data) = serde_json::from_value::<SessionPingData>(event.event_data) {
                handle_session_ping(data, pool, event.event_timestamp).await.unwrap();
            }
        }
        EventType::Match => {
            if let Ok(data) = serde_json::from_value::<MatchData>(event.event_data) {
                handle_match(data, pool, event.event_timestamp).await.unwrap();
            }
        }
    }
}

async fn handle_registration(data: RegistrationData, pool: &PgPool, timestamp: i64) -> Result<(), sqlx::Error> {
    query!(
        "INSERT INTO registrations (user_id, country, device_os, registration_time)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (user_id) DO NOTHING",
        data.user_id,
        data.country,
        data.device_os,
        timestamp
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn handle_session_ping(data: SessionPingData, pool: &PgPool, timestamp: i64) -> Result<(), sqlx::Error> {
    query!(
        "INSERT INTO sessions (user_id, session_start)
         VALUES ($1, $2)",
        data.user_id,
        timestamp
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn handle_match(data: MatchData, pool: &PgPool, timestamp: i64) -> Result<(), sqlx::Error> {
    query!(
        "INSERT INTO matches (match_id, home_user_id, away_user_id, home_goals_scored, away_goals_scored, match_start)
         VALUES ($1, $2, $3, $4, $5, $6)
         ON CONFLICT (match_id) DO NOTHING",
        data.match_id,
        data.home_user_id,
        data.away_user_id,
        data.home_goals_scored,
        data.away_goals_scored,
        timestamp
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok(); // Load environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{}", &database_url);

    let pool = Arc::new(PgPool::connect(&database_url).await?);
    println!("Connected to PostgreSQL!");

    process_events("events.jsonl", pool).await;

    Ok(())
}

