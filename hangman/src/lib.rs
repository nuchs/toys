#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate rand;

pub use game::Game;
pub use game::GameState;
pub use render::Console;
pub use words::WordSource;
pub use words::choose_secret;

mod game;
mod render;
mod words;
