use serde::{Deserialize, Serialize};
use shared::GameSession;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub type AppState = Arc<RwLock<GameEngineState>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameEngineState {
    pub games: HashMap<Uuid, GameSession>,
    pub word_list: Vec<String>,
}

impl GameEngineState {
    pub fn new() -> Self {
        let words_raw = include_str!("../../../data/valid-wordle-words.txt");
        let word_list: Vec<String> = words_raw
            .lines()
            .map(|s| s.trim().to_uppercase())
            .filter(|s| !s.is_empty())
            .collect();

        Self {
            games: HashMap::new(),
            word_list,
        }
    }
}
