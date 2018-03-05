
/home/nuchs/work/toys/hangman/src/game.rs,369
pub enum GameState {GameState2,28
pub struct Game {Game8,86
    pub fn new(secret: String) -> Game {new12,119
    pub fn state(&self) -> GameState {state16,183
    pub fn make_guess(&mut self, guess: char) {make_guess20,252
mod test {test25,322
    fn a_test_function() {a_test_function29,369
    fn another_test_function() {another_test_function33,415

/home/nuchs/work/toys/hangman/src/render.rs,188
pub struct Console;Console3,17
    pub fn new() -> Console {new6,53
    pub fn render(&self, game: &Game) {render10,109
    pub fn render_end(&self, game: &Game) {render_end13,156

/home/nuchs/work/toys/hangman/src/main.rs,64
fn main() {main7,89
fn get_guess() -> char {get_guess21,395

/home/nuchs/work/toys/hangman/src/my_module/mod.rs,0

/home/nuchs/work/toys/hangman/src/words.rs,53
pub fn choose_secret() -> String {choose_secret1,0

/home/nuchs/work/toys/hangman/src/lib.rs,0
/home/nuchs/.rusty-tags/cache/rand-6479602757255575142.emacs,include
