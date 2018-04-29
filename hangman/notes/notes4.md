# Learning Rust, Project 1: Hangman

## Part 4

With the bulk of the game logic written, the next stop is the render
module and a look at how to display the game to the user.

The Console struct doesn't look like it makes sense anymore; the Game
struct contains all the state required to render the current state of
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

    print!("{}", render(&game));

    while game.state() == GameState::InProgress {
        let guess = get_guess();
        game.make_guess(guess).unwrap();
        print!("{}", render(&game));
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

pub fn render(game: &Game) -> String {
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
// snip
pub fn render(game: &Game) -> String {
    match game.state() {
        GameState::InProgress => game_in_progress_message(game),
        GameState::Won => game_won_message(game),
        GameState::Lost => game_lost_message(game),
    }
}

fn game_won_message(game: &Game) -> String {
    format!(r#"Well done you guessed the secret ({})
Now all of your hopes and dreams will come true"#,
    game.secret())
}

fn game_lost_message(game: &Game) -> String {
    format!(r#"You failed to guess the secret ({})
Never mind, we can't all be winners.
Now if you could just join this queue for the fabulous Ark Ship B..."#,
    game.secret())
}

fn game_in_progress_message(game: &Game) -> String {
   format!(r#"
            {}
Guesses   : {}
Remaining : {}
"#,
           obscured_secret(game),
           display_guesses(game.guesses()),
           game.remaining_guesses()) 
}

fn obscured_secret(game: &Game) -> String {
    unimplemented!();
}

fn display_guesses(guesses: &[char]) -> String {
    let mut display = String::new();

    for g in guesses {
        display.push(*g);
        display.push(',');
        display.push(' ');
    }

    display.pop();
    display.pop();

    display
}
```

There's probably a better way to build a string out of characters but
I'm on a timer (the baby is going to wake up soon) and this is a nice
and simple way of doing it.

Finally I need to generate a string from the secret which only
displays those characters that have been guessed.

```rust
fn obscured_secret(game: &Game) -> String {
    let mut os = String::new();

    for c in game.secret().chars() {
        if game.guesses().contains(&c) {
            os.push(c); 
        } else {
            os.push('_');
        }

        os.push(' ');
    }

    os.pop();
    os
}

// snip

#[test]
fn the_obscured_secret_should_initially_be_all_blanks() {
    let game = Game::new("secret".to_owned(), 1);

    assert_eq!(obscured_secret(&game), "_ _ _ _ _ _");
}

#[test]
fn a_correct_guess_should_reveal_all_matching_parts_of_the_obscured_secret() {
    let mut game = Game::new("secret".to_owned(), 1);

    game.make_guess('e');

    assert_eq!(obscured_secret(&game), "_ e _ _ e _");
}

#[test]
fn an_incorrect_guess_should_not_reveal_any_parts_of_the_obscured_secret() {
    let mut game = Game::new("secret".to_owned(), 2);

    game.make_guess('f');

    assert_eq!(obscured_secret(&game), "_ _ _ _ _ _");
}
```

Genrally my preference is only to test the public interface to
things. From my experience the interior of a class/struct changes far
more often than the exterior which means that if you do test the
interior that your tests will have to be updated more often making
them more of a burden. A related point is that in a lot of cases as
long as the external behaviour of the type is correct then its not
terribly important how we got there (not always though). In which Case
testing the interior of a type is of questionable value.

With regards to the obscured secret as I could parse the data out of
the status update but this seems unnecessarily burdensome given I have
the string directly available. So I took the easy root and tested a
private function.

```rust
#[test]
fn the_obscured_secret_should_initially_be_all_blanks() {
    let game = Game::new("secret".to_owned(), 1);

    assert_eq!(obscured_secret(&game), "_ _ _ _ _ _");
}

#[test]
fn a_correct_guess_should_reveal_all_matching_parts_of_the_obscured_secret() {
    let mut game = Game::new("secret".to_owned(), 1);

    game.make_guess('e');

    assert_eq!(obscured_secret(&game), "_ e _ _ e _");
}

#[test]
fn an_incorrect_guess_should_not_reveal_any_parts_of_the_obscured_secret() {
    let mut game = Game::new("secret".to_owned(), 2);

    game.make_guess('f');

    assert_eq!(obscured_secret(&game), "_ _ _ _ _ _");
}
```

As with the display_guesses function I've gone for the obvious implementation

```rust
fn obscured_secret(game: &Game) -> String {
    let mut os = String::new();

    for c in game.secret().chars() {
        if game.guesses().contains(&c) {
            os.push(c);
        } else {
            os.push('_');
        }

        os.push(' ');
    }

    os.pop();
    os
}
```

And that's pretty much it; this was quite a simple module all that's
left now is to link everything together.
