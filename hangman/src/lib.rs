#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate rand;

pub use game::Game;
pub use game::GameState;
pub use words::WordSource;
pub use words::choose_secret;
pub use render::render;

mod game;
mod render;
mod words;
