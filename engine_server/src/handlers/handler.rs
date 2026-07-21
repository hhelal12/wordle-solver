use crate::logic::game;
use crate::logic::models::AppState;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use shared::{GameView, GuessRequest, GuessResponse};
use uuid::Uuid;

/// Handles starting a new game session
pub async fn handle_start_game(State(state): State<AppState>) -> Json<String> {
    let message = game::start_new_game(&state);
    Json(message)
}

/// Handles the read of specfic game
pub async fn handle_read_game(
    State(state): State<AppState>,
    Path(game_id): Path<Uuid>,
) -> Result<Json<GameView>, (StatusCode, String)> {
    match game::read_game(&state, game_id) {
        Ok(view) => Ok(Json(view)),
        Err(err) => Err((StatusCode::NOT_FOUND, err)),
    }
}

/// Handles a word guess attempt
pub async fn handle_guess(
    State(state): State<AppState>,
    Path(game_id): Path<Uuid>, // recive the uuid from url
    Json(payload): Json<GuessRequest>,
) -> Result<Json<GuessResponse>, (StatusCode, String)> {
    match game::guess_word(&state, game_id, &payload.word) {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}
