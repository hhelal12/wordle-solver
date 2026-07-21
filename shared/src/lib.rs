use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuessRequest {
    pub word: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum LetterStatus {
    Green,
    Yellow,
    Gray,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuessResponse {
    pub feedback: Vec<LetterStatus>,
    pub is_win: bool,
    pub is_loss: bool,
    pub secret_word: Option<String>, // Only reveal when game ends
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameSession {
    pub game_id: String,
    pub secret_word: String,
    pub guesses: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameView {
    pub game_id: String,
    pub guesses: Vec<String>,
}
