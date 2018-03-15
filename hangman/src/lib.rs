//! The hangman crate is used to play a simple game of hangman. It
//! consists of three parts.
//! * game : The game module contaisn the Game type which tracks
//!          the state of a game and contains all the game logic
//! * words : This module is resposible for loading a list of
//!           words and selecting one of them as the secret for
//!           the game
//! * render : The render module is responsible for generating
//!            descriptions of the game.
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate rand;

use std::io;

pub use game::Game;
pub use game::GameState;
pub use words::WordSource;
pub use words::choose_secret;
pub use render::render;

mod game;
mod render;
mod words;

pub struct Config {
    pub total_guesses: u32,
    pub word_source: WordSource
}

pub fn start_game(config: Config) -> io::Result<Game> {
    let secret = choose_secret(config.word_source)?;
    Ok(Game::new(secret, config.total_guesses))
}
