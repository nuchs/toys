use self::GameError::*;
use self::GameState::*;

#[derive(Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Won,
    Lost,
}

#[derive(Debug)]
pub enum GameError {
    AlreadyGuessed,
    GameIsOver,
}

pub type GameResult = Result<(), GameError>;

pub struct Game {
    secret: String,
    guesses: Vec<char>,
    remaining_guesses: u32,
}

impl Game {
    pub fn new(secret: String, total_guesses: u32) -> Game {
        Game {
            secret: secret,
            guesses: Vec::new(),
            remaining_guesses: total_guesses,
        }
    }

    pub fn remaining_guesses(&self) -> u32 {
        self.remaining_guesses
    }

    pub fn guesses(&self) -> &[char] {
        &self.guesses
    }

    pub fn secret(&self) -> &str {
        &self.secret
    }

    pub fn state(&self) -> GameState {
        if self.game_is_lost() {
            return Lost;
        } else if self.game_is_won() {
            return Won;
        }

        InProgress
    }

    pub fn make_guess(&mut self, guess: char) -> GameResult {
        self.is_allowed(guess)?;
        self.record(guess);

        Ok(())
    }

    fn is_allowed(&self, guess: char) -> GameResult {
        if self.state() != InProgress {
            return Err(GameIsOver);
        } else if self.guesses.contains(&guess) {
            return Err(AlreadyGuessed);
        }

        Ok(())
    }

    fn record(&mut self, guess: char) {
        self.guesses.push(guess);
        self.guesses.sort();

        if self.is_incorrect(guess) {
            self.remaining_guesses -= 1;
        }
    }

    fn is_incorrect(&self, guess: char) -> bool {
        !self.secret.contains(guess)
    }

    fn game_is_lost(&self) -> bool {
        self.remaining_guesses == 0
    }

    fn game_is_won(&self) -> bool {
        self.secret.chars().all(|c| self.guesses.contains(&c))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initial_game_state_should_be_in_progress() {
        let sut = Game::new("stub".to_owned(), 1);

        assert_eq!(sut.state(), InProgress);
    }

    #[test]
    fn initial_remaining_guesses_should_match_total_guesses() {
        let total_guesses = 1;
        let sut = Game::new("stub".to_owned(), total_guesses);

        assert_eq!(sut.remaining_guesses(), total_guesses);
    }

    #[test]
    fn initially_no_guesses_should_have_been_made() {
        let sut = Game::new("stub".to_owned(), 1);

        assert_eq!(sut.guesses().len(), 0);
    }

    #[test]
    fn the_secret_should_match_the_value_provided_on_construction() {
        let sut = Game::new("secret".to_owned(), 1);

        assert_eq!(sut.secret(), "secret");
    }

    #[test]
    fn an_incorrect_guess_should_decrement_the_remaining_goes() {
        let total_guesses = 2;
        let mut sut = Game::new("secret".to_owned(), total_guesses);

        sut.make_guess('z').unwrap();

        assert_eq!(sut.remaining_guesses(), total_guesses - 1);
    }

    #[test]
    fn a_correct_guess_should_not_decrement_the_remaining_goes() {
        let total_guesses = 2;
        let mut sut = Game::new("secret".to_owned(), total_guesses);

        sut.make_guess('s').unwrap();

        assert_eq!(sut.remaining_guesses(), total_guesses);
    }

    #[test]
    fn guesses_should_be_recorded_in_alphabetical_order() {
        let mut sut = Game::new("secret".to_owned(), 5);

        sut.make_guess('z').unwrap();
        sut.make_guess('a').unwrap();
        sut.make_guess('m').unwrap();

        assert_eq!(sut.guesses(), &['a', 'm', 'z']);
    }

    #[test]
    fn repeating_a_guess_should_have_no_effect() {
        let total_guesses = 5;
        let mut sut = Game::new("secret".to_owned(), total_guesses);

        sut.make_guess('z');
        sut.make_guess('z');
        sut.make_guess('z');
        sut.make_guess('z');

        assert_eq!(sut.guesses(), &['z']);
        assert_eq!(sut.remaining_guesses(), total_guesses - 1);
    }

    #[test]
    fn when_remaining_goes_reaches_zero_the_game_is_lost() {
        let mut sut = Game::new("secret".to_owned(), 1);

        sut.make_guess('z').unwrap();

        assert_eq!(sut.state(), Lost);
    }

    #[test]
    fn the_game_is_won_when_all_the_letters_in_the_secret_are_guessed() {
        let mut sut = Game::new("secret".to_owned(), 1);

        sut.make_guess('s').unwrap();
        sut.make_guess('e').unwrap();
        sut.make_guess('c').unwrap();
        sut.make_guess('r').unwrap();
        sut.make_guess('t').unwrap();

        assert_eq!(sut.state(), Won);
    }

    #[test]
    fn guesses_made_after_the_game_is_lost_are_ignored() {
        let mut sut = Game::new("easy".to_owned(), 1);

        sut.make_guess('z');
        sut.make_guess('e');
        sut.make_guess('a');
        sut.make_guess('s');
        sut.make_guess('y');

        assert_eq!(sut.state(), Lost);
        assert_eq!(sut.guesses(), &['z']);
        assert_eq!(sut.remaining_guesses(), 0);
    }

    #[test]
    fn guesses_made_after_the_game_is_won_are_ignored() {
        let mut sut = Game::new("easy".to_owned(), 1);

        sut.make_guess('e');
        sut.make_guess('a');
        sut.make_guess('s');
        sut.make_guess('y');
        sut.make_guess('z');
        sut.make_guess('z');

        assert_eq!(sut.state(), Won);
        assert_eq!(sut.guesses(), &['a', 'e', 's', 'y']);
        assert_eq!(sut.remaining_guesses(), 1);
    }
}
