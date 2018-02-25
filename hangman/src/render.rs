use game::Game;

pub struct Console;

impl Console {
    pub fn new() -> Console {
        Console {}
    }

    pub fn render(&self, game: &Game) {
    }

    pub fn render_end(&self, game: &Game) {
    }
}
