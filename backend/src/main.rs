mod core;
mod contracts;
mod db;
mod handlers;
mod app_state;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::{
    routing::get,
    Router,
};
use axum::http::{HeaderValue, Method, StatusCode};
use axum::routing::post;
use tower_http::cors::CorsLayer;
use crate::app_state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let pool = db::db::create_db_pool().await;

    let state = AppState {
        db_pool: pool,
        rooms: Arc::new(Mutex::new(HashMap::new())),
    };

    let allowed_origin = std::env::var("ALLOWED_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:4200".to_string());
    
    let cors = CorsLayer::new()
        .allow_origin(allowed_origin.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(tower_http::cors::Any);

    let app = Router::new()
        .route("/api/health", get(health_check_handler))
        .nest("/api/chats", Router::new()
            .route("/", post(handlers::create_chat))
            .route("/connect", get(handlers::connect_to_chat))
            .route("/{name}/messages", get(handlers::get_messages))
        )
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn health_check_handler() -> StatusCode {
    StatusCode::OK
}