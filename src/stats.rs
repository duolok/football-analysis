use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Row};
use std::error::Error;

use crate::models::{UserStatsRequest, UserLevelStats, GameStatsRequest, GameLevelStats};

fn map_user_stats_row(row: &Row) -> UserLevelStats {
    UserLevelStats {
        country: row.get("country"),
        registration_datetime: row.get("registration_datetime"),
        days_since_last_login: row.get("days_since_last_login"),
        session_count: row.get("session_count"),
        game_time: row.get("game_time"),
        match_points: row.get("match_points"),
        match_time_percentage: row.get("match_time_percentage"),
    }
}

fn map_game_stats_row(row: &Row) -> GameLevelStats {
    GameLevelStats {
        daily_active_users: row.get("daily_active_users"),
        session_count: row.get("session_count"),
        avg_sessions: row.get("avg_sessions"),
        top_users: row.get("top_users"),
    }
}

pub async fn get_user_stats(
    request: &UserStatsRequest,
    client: &tokio_postgres::Client,
) -> Result<UserLevelStats, Box<dyn std::error::Error>> {
    let query = r#"
        WITH session_data AS (
            SELECT
                user_id,
                MIN(session_start) AS session_start,
                MAX(session_start) AS session_end
            FROM sessions
            WHERE user_id = $1
            GROUP BY user_id, session_id
        ),
        match_data AS (
            SELECT
                CASE WHEN home_user_id = $1 THEN home_goals_scored > away_goals_scored ELSE away_goals_scored > home_goals_scored END AS is_win,
                CASE WHEN home_user_id = $1 THEN home_goals_scored = away_goals_scored ELSE away_goals_scored = home_goals_scored END AS is_draw,
                home_user_id,
                away_user_id,
                match_start
            FROM matches
            WHERE home_user_id = $1 OR away_user_id = $1
        )
        SELECT
            r.country,
            to_char(r.registration_time, 'YYYY-MM-DD HH24:MI:SS') AS registration_datetime,
            COALESCE(
                DATE_PART('day', NOW() - MAX(sd.session_end)),
                0
            ) AS days_since_last_login,
            COUNT(sd.session_start) AS session_count,
            COALESCE(SUM(EXTRACT(EPOCH FROM (sd.session_end - sd.session_start))), 0) AS game_time,
            (
                SELECT
                    COALESCE(SUM(CASE WHEN md.is_win THEN 3 WHEN md.is_draw THEN 1 ELSE 0 END), 0)
                FROM match_data md
            ) AS match_points,
            ROUND(
                COALESCE(SUM(EXTRACT(EPOCH FROM (CASE WHEN md.is_win OR md.is_draw THEN AGE(md.match_start) ELSE NULL END))), 0)::FLOAT /
                COALESCE(SUM(EXTRACT(EPOCH FROM (sd.session_end - sd.session_start))), 1)::FLOAT * 100, 2
            ) AS match_time_percentage
        FROM registrations r
        LEFT JOIN session_data sd ON r.user_id = sd.user_id
        WHERE r.user_id = $1
        GROUP BY r.country, r.registration_time;
    "#;

    let row = client.query_one(query, &[&request.user_id]).await?;

    Ok(UserLevelStats {
        country: row.get("country"),
        registration_datetime: row.get("registration_datetime"),
        days_since_last_login: row.get("days_since_last_login"),
        session_count: row.get("session_count"),
        game_time: row.get("game_time"),
        match_points: row.get("match_points"),
        match_time_percentage: row.get("match_time_percentage"),
    })
}

pub async fn get_game_stats(
    request: &GameStatsRequest,
    client: &Client,
) -> Result<GameLevelStats, Box<dyn Error>> {
    let query = r#"
        SELECT
            COUNT(DISTINCT sessions.user_id) AS daily_active_users,
            COUNT(sessions.id) AS session_count,
            ROUND(AVG(sessions_per_user.session_count), 2) AS avg_sessions,
            ARRAY(
                SELECT user_id
                FROM (
                    SELECT user_id,
                        SUM(CASE WHEN home_user_id = user_id AND home_goals_scored > away_goals_scored THEN 3
                                 WHEN home_user_id = user_id AND home_goals_scored = away_goals_scored THEN 1
                                 WHEN away_user_id = user_id AND away_goals_scored > home_goals_scored THEN 3
                                 WHEN away_user_id = user_id AND away_goals_scored = home_goals_scored THEN 1 ELSE 0 END) AS points
                    FROM matches
                    GROUP BY user_id
                ) AS user_points
                ORDER BY points DESC LIMIT 1
            ) AS top_users
        FROM sessions
        JOIN (
            SELECT user_id, COUNT(*) AS session_count
            FROM sessions
            GROUP BY user_id
        ) AS sessions_per_user ON sessions.user_id = sessions_per_user.user_id
        WHERE ($1::DATE IS NULL OR DATE(sessions.start_time) = $1)
    "#;

    let row = client.query_one(query, &[&request.date]).await?;
    Ok(map_game_stats_row(&row))
}
