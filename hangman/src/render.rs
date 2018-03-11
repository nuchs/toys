use game::Game;
use game::GameState;

/// The render function generates a description of the current state
/// of its [Game](struct.Game.html) argument. If the game is in progress it will display
/// an obscured version of the secret, with only those values that have
/// been guessed being displayed.
/// # Examples
/// ```
/// # use hangman::Game;
/// # use hangman::GameState;
/// # use hangman::render;
/// let mut game = Game::new("secret".to_owned(), 1);
///
/// game.make_guess('e');
/// game.make_guess('t');
///
/// assert!(render(&game).contains("_ e _ _ e t"));
/// ```
pub fn render(game: &Game) -> String {
    match game.state() {
        GameState::InProgress => game_in_progress_message(game),
        GameState::Won => game_won_message(game),
        GameState::Lost => game_lost_message(game),
    }
}

fn game_won_message(game: &Game) -> String {
    format!(
        r#"Well done you guessed the secret ({})
Now all of your hopes and dreams will come true"#,
        game.secret()
    )
}

fn game_lost_message(game: &Game) -> String {
    format!(
        r#"You failed to guess the secret ({})
Never mind, we can't all be winners.
Now if you could just join this queue for the fabulous Ark Ship B..."#,
        game.secret()
    )
}

fn game_in_progress_message(game: &Game) -> String {
    format!(
        r#"
            {}
Guesses   : {}
Remaining : {}
"#,
        obscured_secret(game),
        display_guesses(game.guesses()),
        game.remaining_guesses()
    )
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn game_won_message_should_be_displayed_if_the_player_wins() {
        let mut game = Game::new("a".to_owned(), 1);
        game.make_guess('a');

        assert!(render(&game).contains("you guessed the secret"));
    }

    #[test]
    fn game_lost_message_should_be_displayed_if_the_player_loses() {
        let mut game = Game::new("a".to_owned(), 1);
        game.make_guess('z');

        assert!(render(&game).contains("You failed to guess the secret"));
    }

    #[test]
    fn game_in_progress_message_should_be_displayed_if_the_game_isnt_over() {
        let game = Game::new("a".to_owned(), 1);

        assert!(render(&game).contains("Remaining"));
    }

    #[test]
    fn the_obscured_secret_should_initally_be_all_blanks() {
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
}
