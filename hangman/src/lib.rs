pub struct Game {
    secret: String,
    guesses: Vec<char>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            secret: "bacon".to_string(),
            guesses: Vec::new(),
        }
    }
}
