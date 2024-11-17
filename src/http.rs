use actix_web::{web, HttpResponse, Responder};
use tokio_postgres::Client;
use std::sync::Arc;

use crate::stats::{get_user_stats, get_game_stats};
use crate::models::{UserStatsRequest, GameStatsRequest};

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user-stats")
            .route(web::get().to(get_user_stats_handler)) 
    )
    .service(
        web::resource("/game-stats")
            .route(web::get().to(get_game_stats_handler)) 
    );
}

pub async fn get_user_stats_handler(
    query: web::Query<UserStatsRequest>,
    client: web::Data<Arc<Client>>,
) -> impl Responder {

    match get_user_stats(&query.into_inner(), &client).await {
        Ok(stats) => {
            HttpResponse::Ok().json(stats) 
        }
        Err(err) => {
            HttpResponse::InternalServerError().body("Failed to fetch user stats")
        }
    }
}

pub async fn get_game_stats_handler(
    query: web::Query<GameStatsRequest>,
    client: web::Data<Arc<Client>>,
) -> impl Responder {

    match get_game_stats(&query.into_inner(), &client).await {
        Ok(stats) => {
            HttpResponse::Ok().json(stats) 
        }
        Err(err) => {
            HttpResponse::InternalServerError().body("Failed to fetch game stats")
        }
    }
}

