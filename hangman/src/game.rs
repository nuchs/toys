#[derive(Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Won,
    Lost
}

pub struct Game {
    secret: String,
    guesses: Vec<char>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            secret: "bacon".to_string(),
            guesses: Vec::new(),
        }
    }

    pub fn state(&self) -> GameState {
        GameState::Won
    }

    pub fn make_guess(&mut self, guess: char) {
    }
}
