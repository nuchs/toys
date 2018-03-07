extern crate hangman;

use hangman::Game;
use hangman::GameState;
use hangman::Console;
use hangman::WordSource;

fn main() {
    let secret = hangman::choose_secret(WordSource::BuiltIn).unwrap();
    let mut game = Game::new(secret, 7);
    let view = Console::new();

    while game.state() == GameState::InProgress {
        view.render(&game);
        let guess = get_guess();
        game.make_guess(guess).unwrap();
    }

    view.render_end(&game);
}

fn get_guess() -> char {
    unimplemented!();
}
