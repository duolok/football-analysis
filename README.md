# football-analysis

This project processes and analyzes data generated by a football game. It includes an API to retrieve user-level and game-level statistics and requires a PostgreSQL database for data storage.

**Note:** This is an unfinished project. The server implementation is currently missing and needs to be completed.

---

## Getting Started

### Prerequisites

Ensure you have the following installed:

- Docker
- Rust (with `cargo`)

---

### Setup Instructions

#### Step 1: Start the PostgreSQL Database

Use Docker Compose to set up and start the PostgreSQL database. Run the following command in your project directory:

```bash
docker-compose up -v
```

#### Step 2: Init Tables
Run this script to init tables: 

```bash
./init_tables.sh
```

#### Step 3: Process Events

To process the events from the input dataset (e.g., `events.jsonl`) and populate the database, run:

```bash
cargo run --bin process_events
```

#### Step 3: Running the server

```bash
cargo run --bin server
```

### API Endpoints
#### 1. Get User-Level Statistics
Endpoint: /user-stats
Method: GET

Query Parameters:
- user_id (required): The unique identifier of the user
- date (optional): A specific date for which to retrieve statistics (format: YYYY-MM-DD)
Example:

```bash
curl "http://127.0.0.1:8080/user-stats?user_id=12345"
```
#### 2. Get Game-Level Statistics
Endpoint: /game-stats
Method: GET

Query Parameters:
- date (optional): A specific date for which to retrieve statistics (format: YYYY-MM-DD)
Example:

``` bash
curl "http://127.0.0.1:8080/game-stats?date=2024-11-14"
```

## How it works

My idea was to make 3(4) binaries for this project:
- process_events - which would process all the data and make it clean for users to query
- server - which you would run in the background ( used to send http requests: curl, Postman, etc...)
- cli - client for easier server management
- tui (which is not implemented) - tui app that would pretty much do the exact same thing as cli but in a nice terminal interface specifically made for this




# Football Manager Data Analysis API

This project processes and analyzes data generated by a football manager game. It includes an API to retrieve user-level and game-level statistics, and it requires a PostgreSQL database for data storage.

## Getting Started

### Prerequisites

Ensure you have the following installed:
- [Docker](https://www.docker.com/)
- [Rust](https://www.rust-lang.org/) (with `cargo`)

---

### Setup Instructions

#### Step 1: Start the PostgreSQL Database

Use Docker Compose to set up and start the PostgreSQL database. Run the following command in your project directory:

```bash
docker-compose up -v

