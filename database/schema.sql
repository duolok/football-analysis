CREATE TABLE registrations (
    user_id TEXT PRIMARY KEY,
    country TEXT NOT NULL,
    device_os TEXT NOT NULL,
    registration_time BIGINT NOT NULL
);

CREATE TABLE sessions (
    session_id SERIAL PRIMARY KEY,
    user_id TEXT NOT NULL,
    session_start BIGINT,
    session_end BIGINT
);

CREATE TABLE matches (
    match_id TEXT PRIMARY KEY,
    home_user_id TEXT NOT NULL,
    away_user_id TEXT NOT NULL,
    home_goals_scored INTEGER,
    away_goals_scored INTEGER,
    match_start BIGINT NOT NULL,
    match_end BIGINT NOT NULL
);
