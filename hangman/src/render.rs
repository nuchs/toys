use game::Game;
use game::GameState;

pub fn render(game: &Game) {
    let display = match game.state() {
        GameState::InProgress => game_in_progress_message(game),
        GameState::Won => game_won_message(game),
        GameState::Lost => game_lost_message(game),
    };

    print!("{}", display);
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
