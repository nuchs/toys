extern crate hangman;

use hangman::Game;
use hangman::GameState;
use hangman::Console;

fn main() {
    let mut game = Game::new();
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
