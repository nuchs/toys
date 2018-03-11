extern crate hangman;

use hangman::Game;
use hangman::GameState;
use hangman::WordSource;
use hangman::choose_secret;
use hangman::render;

fn main() {
    // when doing clap, use extract method on these two lines
    // and move them into lib.rs
    let secret = choose_secret(WordSource::BuiltIn).unwrap();
    let mut game = Game::new(secret, 7);

    render(&game);

    while game.state() == GameState::InProgress {
        let guess = get_guess();
        game.make_guess(guess).unwrap();
        render(&game);
    }
}

fn get_guess() -> char {
    unimplemented!();
}

