use crate::logic::models::AppState;
use rand::Rng;
use shared::{GameSession, GameView, GuessResponse, LetterStatus};
use uuid::Uuid;

/// Creates a new game session and stores it in the state
pub fn start_new_game(state: &AppState) -> String {
    let game_id = Uuid::new_v4();

    let secret_word = {
        let guard = state.read().unwrap();
        if guard.word_list.is_empty() {
            "APPLE".to_string()
        } else {
            let mut rng = rand::thread_rng();
            let idx = rng.gen_range(0..guard.word_list.len());
            guard.word_list[idx].clone()
        }
    };

    let session = GameSession {
        game_id: game_id.to_string(),
        secret_word,
        guesses: Vec::new(),
    };

    let mut guard = state.write().unwrap();
    guard.games.insert(game_id, session);

    format!("Game started! with id: {}", game_id)
}

// read the game data like guess

pub fn read_game(state: &AppState, game_id: Uuid) -> Result<GameView, String> {
    let guard = state.read().unwrap();

    if let Some(session) = guard.games.get(&game_id) {
        Ok(GameView {
            game_id: session.game_id.clone(),
            guesses: session.guesses.clone(),
        })
    } else {
        Err(format!("Game with ID {} not found", game_id))
    }
}

// for the attempt the guess
pub fn guess_word(state: &AppState, game_id: Uuid, word: &str) -> Result<GuessResponse, String> {
    let mut guard = state.write().unwrap();

    let session = match guard.games.get_mut(&game_id) {
        Some(s) => s,
        None => return Err(format!("Game with ID {} not found", game_id)),
    };

    let upper_word = word.to_uppercase();
    if upper_word.len() != 5 {
        return Err("Word must be 5 letters".to_string());
    }

    if session.guesses.len() >= 6 {
        return Err("Game over! Maximum 6 guesses reached.".to_string());
    }

    // Save the guess
    session.guesses.push(upper_word.clone());

    let feedback = evaluate_guess(&session.secret_word, &upper_word);
    let is_win = upper_word == session.secret_word;
    let is_loss = !is_win && session.guesses.len() == 6;

    // Reveal the secret word if they won or lost
    let secret_word = if is_win || is_loss {
        Some(session.secret_word.clone())
    } else {
        None
    };

    // If the game is over (win or loss), remove the session instance from state
    if is_win || is_loss {
        guard.games.remove(&game_id);
    }

    Ok(GuessResponse {
        feedback,
        is_win,
        is_loss,
        secret_word,
    })
}

// validate the charcters
pub fn evaluate_guess(secret: &str, guess: &str) -> Vec<LetterStatus> {
    let secret_chars: Vec<char> = secret.chars().collect();
    let guess_chars: Vec<char> = guess.chars().collect();

    let mut feedback = vec![LetterStatus::Gray; 5];
    let mut secret_matched = [false; 5];

    // Pass 1: Find Greens (Exact matches)
    for i in 0..5 {
        if guess_chars[i] == secret_chars[i] {
            feedback[i] = LetterStatus::Green;
            secret_matched[i] = true;
        }
    }

    // Pass 2: Find Yellows (Wrong position) and Grays
    for i in 0..5 {
        if feedback[i] == LetterStatus::Green {
            continue; // Skip already matched greens
        }

        let mut found_yellow = false;
        for j in 0..5 {
            if !secret_matched[j] && guess_chars[i] == secret_chars[j] {
                found_yellow = true;
                secret_matched[j] = true; // Mark as used
                break;
            }
        }

        if found_yellow {
            feedback[i] = LetterStatus::Yellow;
        } else {
            feedback[i] = LetterStatus::Gray;
        }
    }

    feedback
}
