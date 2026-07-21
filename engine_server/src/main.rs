mod handlers;
mod logic;

use crate::logic::models::{AppState, GameEngineState};
use axum::{
    Router,
    routing::{get, post},
}; // Make sure 'get' is imported
use std::sync::{Arc, RwLock};

#[tokio::main]
async fn main() {
    println!("Server starting...");

    // Global state
    let shared_state: AppState = Arc::new(RwLock::new(GameEngineState::new()));

    // Routes
    let app = Router::new()
        .route("/start", post(handlers::handler::handle_start_game))
        .route("/read/{id}", get(handlers::handler::handle_read_game))
        .route("/game/{id}/guess", post(handlers::handler::handle_guess))
        .with_state(shared_state);

    // Listener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
