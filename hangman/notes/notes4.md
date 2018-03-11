# Learning Rust, Project 1: Hangman

## Part 4

With th ebulk of the game logic written, the next stop is the render
module and a look at how to display the game to the user.

The Console struct doesn't look like it makes sense anymore; the Game
struct contans all the state required to render the current state of
the game.  We'll ditch the struct in favour of a render
function. Actually we could probably roll the render_end functionality
into here as well; render_end would have, had to have checked the game
state to decide whether the game is won or lost. Why not have one
function which decides if the game is won, lost or in progress.

```rust
/* ----- main.rs ----- */

// snip
use hangman::choose_secret;
use hangman::render;

fn main() {
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

/* ----- lib.rs ----- */
// snip
pub use words::WordSource;
pub use words::choose_secret;
pub use render::render;

// snip

/* ----- render.rs ----- */
use game::Game;
use game::GameState;

pub fn render(game: &Game) {
    match game.state() {
        GameState::InProgress => (),
        GameState::Won => (),
        GameState::Lost => (),
    }
}
```

Most of the render module is trivial the only bit which has any meat
to it is how to display the partially obscured secret.

I'll get the trivial parts in place first

```rust

```
