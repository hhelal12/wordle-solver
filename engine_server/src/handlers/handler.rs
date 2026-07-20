use crate::logic::game;
use axum::{Json, debug_handler};
use shared::{GuessRequest, GuessResponse};

pub async fn handle_start_game() -> Json<String> {
    let message = game::start_new_game();
    Json(message)
}

#[debug_handler]
pub async fn handle_guess(Json(payload): Json<GuessRequest>) -> Json<GuessResponse> {
    println!("Received guess: {}", payload.word);

    Json(GuessResponse {
        feedback: Vec::new(),
        is_win: false,
    })
}
