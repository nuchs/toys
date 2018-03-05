# Learning Rust, Project 1: Hangman

## Part 2 testing times

Now that the skeleton of the program is place, the first thing I want
to try and flesh out is the game logic; primarily because it's easy to
test (no messy interactions with things like the file system or random
numbers).

Rust has support for tests via cargo. You can run ```cargo test``` and
it will find all the functions in your crates tagged as tests and
execute them.. Any functions which panic or timeout, fail; all others
pass.

There are some conventions around how to organise your unit tests
In the module bein gtested you create a test module and mark it to
only be compiled when running tests e.g.


```rust
mod my_module {

    // Module is only compiled when the 'test' symbol is passed to 
    // compiler. Cargo does this for you when you run cargo test. 
    // Technically this isn't needed but if it were missing your tests
    // would get compiled into the released binary
    #[cfg(test)] 
    mod test { // Calling the module test is just convention, you
               // could call it bacon and it would still work

        use super::*; // brings my_modules members into scope for
                      // testing. Since test is a child of my_module
                      // it will be able to access the private members 
                      // of my_module should you wish to test those
                      // directly. 
    
        // This attribute marks this as a test function, it will be
        // run and the result recorded in the test results.
        #[test]
        fn a_test_function() {
            // passes test
        }

        #[test]
        fn another_test_function() {
            // A function which panics counts as a fail
            panic!("All the telephone sanitiser is on Ark ship B!");
        }
    }
}
```

## Hello tests

Start with something extrememly simple, that the game shoud start in
the ```InProgress``` state. First write a failing test:

```rust
/* ----- game.rs ----- */

#[cfg(test)]
mod test {
    use super::*; 
    
    #[test]
    fn initial_game_state_should_be_in_progress() {
        let sut = Game::new("stub".to_owned());

        assert_eq!(sut.state, GameState::InProgress);
    }
}
```

Now run ```cargo test``` to make sure it fails.

```
running 1 test
test game::test::initial_game_state_should_be_in_progress ... FAILED

failures:

---- game::test::initial_game_state_should_be_in_progress stdout ----
	thread 'game::test::initial_game_state_should_be_in_progress' panicked at 'not yet implemented', src/game.rs:19:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    game::test::initial_game_state_should_be_in_progress

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

And finally write some code to make it pass

```rust
/* ----- game.rs ----- */

pub struct Game {
    state: GameState
}

impl Game {
    pub fn new(secret: String) -> Game {
        Game {
            state : GameState::InProgress
        }
    }

    pub fn state(&self) -> GameState {
        self.state
    }
```

Attempt to build... and the world ends

```
error[E0507]: cannot move out of borrowed content
  --> src/game.rs:20:9
   |
20 |         self.state
   |         ^^^^ cannot move out of borrowed content
```

My first run in with the borrow checker and thankfully it's fairly
obvious where I've gone wrong; the state getter only has a reference
to the Game object and I'm trying to move ownership of the state
member out of the function. A bit of a brain wrong on my part, in my
head I still think of enums as being an integer and hence a copy of it
would be made for returning. I think I'm basically mixing up my mental
model for C with my one for rust. Lets fix that:

```rust
/* ----- game.rs ----- */

// snip

    pub fn state(&self) -> &GameState {
        &self.state
    }

// snip

    #[test]
    fn initial_game_state_should_be_in_progress() {
        let sut = Game::new("stub".to_owned());

        assert_eq!(*sut.state(), GameState::InProgress);
    }

/* ----- main.rs ----- */

// snip

    // 
    while *game.state() == GameState::InProgress {

// snip
```

Run the tests again

```
running 1 test
test game::test::initial_game_state_should_be_in_progress ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/hangman-f82a1c65e2128868

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests hangman

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Whoop, all is well. Ok now that I've got the basic test loop setup
lets look at what the game logic should be.

1. A users guess should be recorded
2. When a user makes an incorrect guess it should decrement their
   remaining goes
3. If the user has no remaining goes the game state should change to
   lost
4. When a user makes a correct guess the remaining goes count should
   not decrement
5. If the user guesses all the letters in the secret the game state
   should change to won
6. Repeating a guess should have no effect.
7. Requesting the collection of previous guesses should return them
   sorted into alphabetical order (as defined by UK English, i18n
   would be overkill for this exercise).
   
## Initial states

I'll repeat the trick from earlier of specifying the interface I want
to have (this time in the tests) and then fill out the implementation
afterwards. For the first tests I'll check that the initial state of a
game object is correct

```rust
#[test]
fn initial_remaining_guesses_should_match_total_guesses() {
    let total_guesses = 1;
    let sut = Game::new("stub".to_owned(), total_guesses);

    assert_eq!(sut.remaining_guesses(), total_guesses);
}

#[test]
fn initially_no_guesses_should_have_been_made() {
    let sut = Game::new("stub".to_owned(), 1);

    assert_eq!(sut.guesses(), &[]);
}

#[test]
fn the_secret_should_match_the_value_provided_on_construction() {
    let sut = Game::new("secret".to_owned(), 1);

    assert_eq!(sut.secret(), "secret");
}
```

And the implementation

```rust
pub struct Game {
    state: GameState,
    secret: String,
    guesses: Vec<char>,
    remaining_guesses: u32
}

impl Game {
    // snip
    pub fn remaining_guesses(&self) -> u32 {
        self.remaining_guesses
    }

    pub fn guesses(&self) -> &[char] {
        self.guesses.as_ref()
    }

    pub fn secret(&self) -> &str {
        &self.secret
    }
    // snip
}
```

At this stage I just want to do the simplest thing that works. Later I
can refactor if needs be. For example I could have calculated
remaining_guesses from the first two fields but it feels simpler just
to have a counter. Similarly I chose Vec, not because it's the most
efficient data structure for the job but because I can get it working
with minimal cognitive load.

The key thing is that by encapsulating the representation of the
data it should be straight forward to change it a later date 
(and the tests will show me if I break anything).

## Properties

One of the questons that occurred to me while writing this post was
"Do getters and setters make sense in rust". Nowadays I mostly work in
C# and making use of properties when I want to expose state is almost
like a reflex but what makes sense in one language doesn't necessarily
in another.

Typically I use properties for one of the following reasons

1. To give me more fine grained control of access to a classes' state
2. To add validation logic to setters.
3. To encapsulate state so that I can change its representation later
4. To add lazy instantiation and caching
5. To protect an invariant of the type

All of that still makes sense, but in cases where I don't need to
do one of these things, is it beneficial to do so? Rust doesn't have
the language support for properties that C# does so the cost in terms
of code verbosity/readabilty is more significant.

In cases where the struct member was only being exposed within a scope
which I controlled (basically up to, but not including the crate
border), then I think not bothering to use a porepty as wa wrapper
might make sense. As soon as it becomes part of the public interface I
owuld be a lot more wary of exposing the struct member.

In the specific case of this exercise, properties make sense. I havea
type invariant that the remaining_guesses should equal it's initial
value minus the number of elements in guesses that are not in
secret. Allowing any of these members to be modified directly rather
than via the make_guess method would open up th epossibiity of the
invariant being violated.

## Remaining logic

I'm going to skip forward a little bit now, partly because not too
much of interest happened but mostly because I got carried away witht
the coding and forgot to keep any notes(note to self: don't forget to
check-in regularly).

The point I've got to is that the game logic is implemented and
passing the tests but it could do with a bit of a tidy up. The main
point of interest is the make_guess method.

```rust
pub fn make_guess(&mut self, guess: char) {
    if self.state != InProgress {
        return;
    } else if self.guesses.contains(&guess) {
        return;
    }

    self.guesses.push(guess);
    self.guesses.sort();

    if self.is_incorrect(guess) {
        self.remaining_guesses -= 1;
    }
    
    if self.guesses == 0 {
        self.state = GameState::Lost;
    } else if self.secret.chars().all(|c| self.guesses.contains(&c)) {
        self.state = GameState::Won;
    }
}
```

The checks at the beginning of the method cause a user's guess to be
ignored if the game is over or they've already mae the guess. I'm not
happy about the way that this happens silently, given that this is a
library it feels like it should be signalled tgo the caller and they
should be able to make a decision about what to do.

In rust the way to do do this seems to be via error codes,
specifically the Result type (which seems to be equivalent to Either
type in Haskell). The Result type is an enuemration and can return one
of two possibilities Ok along with a value if everything went well or
Err and an error code if it didn't e.g.

```rust
fn divide (n: float64, d: float64) -> Result<float64, String> {
    if d == 0 {
        return Err("Divide by zero error".to_owned());
    }
    
    Ok(n / d)
}

fn do_the_maths() {
    match divide (10, 2) {
        Ok(value) => do_something(value),
        Err(msg) => eprintln!("{}", msg);
    }
}
```

At first glance it seems quite cumbersome having to match on the
return value but it has a number of advantages, it makes it clear that
the called function can error and it forces you to deal with potential
errors (even if you choose to do nothing on an error, that's at least
a choice you conciously made). There are also a load of heplful
functions implemented for the Result type to make it significanty more
ergonomic to use.

Getting back to the code in hand, there are two possibe reasons the
user can fail to make a guess so lets record those using an error code enumeration
and use that as the error type for the Result. There's no actual
value to return for the happy path so we can just use unit and a type
alias will help cut down on the noise.

```rust
#[derive(Debug)]
pub enum GameError {
    AlreadyGuessed,
    GameIsOver
}

pub type GameResult = Result<(), GameError>;

pub fn make_guess(&mut self, guess: char) -> GameResult {
    if self.state != InProgress {
        return Err(GameError::GameIsOver);
    } else if self.guesses.contains(&guess) {
        return Err(GameError::AlreadyGuessed);
    }

    self.guesses.push(guess);
    self.guesses.sort();

    if self.is_incorrect(guess) {
        self.remaining_guesses -= 1;
    }
    
    if self.guesses == 0 {
        self.state = GameState::Lost;
    } else if self.secret.chars().all(|c| self.guesses.contains(&c)) {
        self.state = GameState::Won;
    }
    
    Ok(())
}
```

The tests are still passing so all is well.

When reading this method I don't really need to see the validation
logic to understand the method's purpose, I just need to know that
it's being done. Given that, I can apply the 'extract method'
refactoring.

```rust
pub fn make_guess(&mut self, guess: char) -> GameResult {

    if let Err(msg) = self.is_allowed(guess) {
        return Err(msg);
    }

    self.guesses.push(guess);
    self.guesses.sort();

    // snip
}

fn is_allowed(&self, guess: char) -> GameResult {
    if self.state != InProgress {
        return Err(GameIsOver);
    } else if self.guesses.contains(&guess) {
        return Err(AlreadyGuessed);
    }

    Ok(())
}
```

The pattern of either propogating an error from a method call or
carrying on if it suceeds is so common that some syntactic suger has
been added to make this more frictionless, meaning we can reduce
make_guess to:


```rust
pub fn make_guess(&mut self, guess: char) -> GameResult {

    self.is_allowed(guess)?;

    self.guesses.push(guess);
    self.guesses.sort();

    // snip
}
```

Another thing that is bothering me is the Game struct. Any game can be
reproduced at any point in the game if you know the secret,
what guesses the user has made up to that point and the total number
of guesses allowed. This means that the state member of the struct is
redundent and what we are doing is effectively caching it. This might
make sense if it were expensive to calculate the state but this is
hangman and it feels like this member is not pulling its weight.

The first step to getting rid of it is to self encapsulate it

```rust

/* ----- game.rs ----- */
fn is_allowed(&self, guess: char) -> GameResult {
    if self.state() != GameState::InProgress {

// snip


/* ----- main.rs ----- */
while game.state() == GameState::InProgress {

// snip
```

It still builds and the tests still pass. Next move the logic to
calculate its value into the getter

```rust
pub fn state(&self) -> GameState {
    if self.remaining_guesses == 0 {
        return GameState::Lost;
    } else if self.secret.chars().all(|c| self.guesses.contains(&c)) {
        return GameState::Won;
    }

    GameState::InProgress
}

pub fn make_guess(&mut self, guess: char) -> GameResult {
    self.is_allowed(guess)?;

    self.guesses.push(guess);
    self.guesses.sort();

    if self.is_incorrect(guess) {
        self.remaining_guesses -= 1;
    }
    
    Ok(())
}
```

The tests are still green, so now I can do the final step and remove
the state member;

```rust
pub struct Game {
    secret: String,
    guesses: Vec<char>,
    remaining_guesses: u32,
}
```

Looking over the code I see number of small things to tidy up.

1.
make_guess looks like it contains code which is at different levels of
abstraction; the first part indicates the guess is being validated
without giving the details while the second part gives the details of
how to record a guess. Using the extract method refacoring again
should sort that

```rust
pub fn make_guess(&mut self, guess: char) -> GameResult {
    self.is_allowed(guess)?;
    self.record(guess);

    Ok(())
}

fn record(&mut self, guess: char) {
    self.guesses.push(guess);
    self.guesses.sort();

    if self.is_incorrect(guess) {
        self.remaining_guesses -= 1;
    }
}
```

In the method to calculate state it's not immediately apparant what
the condition to check if the player has won is doing. The decompose
conditional refactoring pulls the condition out into a method with a
descriptive name making it easier to understand. For symmetry I repeat
this for the check to see if the player has lost

```rust
pub fn state(&self) -> GameState {
    if self.game_is_lost() {
        return Lost;
    } else if self.game_is_won() {
        return Won;
    }

    GameState::InProgress
}

fn game_is_lost(&self) -> bool {
    self.remaining_guesses == 0
}

fn game_is_won(&self) -> bool {
    self.secret.chars().all(|c| self.guesses.contains(&c))
}
```

Finally there's no danger of the names from the GameState and GameError enums
colliding with something else so I bring them into scope for this
file. This lets me avoid having to fully qualify them, reducing the
noise in the file a little.

The code now looks like this

```rust
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
    GameIsOver
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

        assert_eq!(sut.guesses(), &[]);
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
```

I've probably committed hundreds of rustic, mortal sins here but at
least at this stage they're not obvious to me (ignorence is
bliss). This may all change once I start trying to plug the variosu
components together, but that's for another time.
