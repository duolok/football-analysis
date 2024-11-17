use football_analysis::handlers;
use dotenv::dotenv;
use sqlx::PgPool;
use std::sync::Arc;
use std::env;

const JSONL_PATH: &str = "events.jsonl";

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok(); // load env variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = Arc::new(PgPool::connect(&database_url).await?);
    println!("Connected to PostgreSQL!");

    handlers::process_events(JSONL_PATH, pool).await;

    Ok(())
}
