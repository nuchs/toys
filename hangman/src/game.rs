#[derive(Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Won,
    Lost
}

pub struct Game {
}

impl Game {
    pub fn new() -> Game {
        Game {}
    }

    pub fn state(&self) -> GameState {
        GameState::Won
    }

    pub fn make_guess(&mut self, guess: char) {
    }
}
