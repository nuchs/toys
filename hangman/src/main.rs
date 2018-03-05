extern crate hangman;

use hangman::Game;
use hangman::GameState;
use hangman::Console;

fn main() {
    let secret = hangman::choose_secret();
    let mut game = Game::new(secret, 7);
    let view = Console::new();

    while game.state() == GameState::InProgress {
        view.render(&game);
        let guess = get_guess();
        game.make_guess(guess);
    }

    view.render_end(&game);
}

fn get_guess() -> char {
    unimplemented!();
}
