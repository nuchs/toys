# Learning Rust, Project 1: Hangman

## Part 1 

One of the exercises that I found most useful in "Haskell programming
from first principles" was where I was required to build a command
line hangman game. I think the appeal was that although it was a
trivial problem there was enough meat to it that you were required to
use a reasonable amount of the language.

Rust is just alien enough I don't have a feel for what the best way to
structure things are and hangman feels like a good level of problem for
puzzling some of that out.

## High level plan 

I'm going to start off with the advice from the [rust book]
(https://doc.rust-lang.org/book/second-edition/ch12-03-improving-error-handling-and-modularity.html)
and have the application handle the setup and user interaction while
delegating the remaining behaviour to a library.

As an initial stab I'm going to separate out the display code from the
game logic, that way if I want to add a fancier interface later it
should be fairly simple. For a similar reason I'm going to put the
word selection into its own module; I can start with a hard coded list
of words and then move on to selecting a random word from a file
later.

Starting off my project structure looks like this:

```
$ tree
.
├── Cargo.toml
└── src
    ├── lib.rs
    └── main.rs

```

## The main event loop

When starting something new, I like following the advice in [growing
object orientated software guided by
tests](https://www.amazon.co.uk/Growing-Object-Oriented-Software-Guided-Signature/dp/0321503627)
where I start at the top level and specify the interface I want to
have and then gradually fill in the details behind it.

In this case it's fairly straight forward. I want to display the game
state, have the user make a guess and then update the game
state. Rinse and repeat until the game ends. 

```rust
/* ----- main.rs ----- */
extern crate hangman;

use hangman::Game;
use hangman::GameState;
use hangman::Console;

fn main() {
    let secret = hangman::choose_secret();
    let mut game = Game::new(secret);
    let view = Console::new();

    while game.state() == GameState::InProgress {
        view.render(&game);
        let guess = get_guess();
        game.make_guess(guess);
    }

    view.render_end(&game);
}
```

I can then stub all of that out.

```rust
/* ----- lib.rs ----- */
#[derive(Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Won,
    Lost,
}

pub struct Game {}

impl Game {
    pub fn new(secret: String) -> Game {
        Game {}
    }

    pub fn state(&self) -> GameState {
        unimplemented!();
    }

    pub fn make_guess(&mut self, guess: char) {}
}

pub struct Console;

impl Console {
    pub fn new() -> Console {
        Console {}
    }

    pub fn render(&self, game: &Game) {}

    pub fn render_end(&self, game: &Game) {}
}

pub choose_secret() -> String {
    unimplemented!();
}

/* ----- main.rs ----- */
fn get_guess() -> char {
    unimplemented!();
}
```

And check it all builds

```
-*- mode: cargo-process; default-directory: "~/work/toys/hangman/" -*-
Cargo-Process started at Mon Feb 26 09:49:31

cargo build 
   Compiling hangman v0.1.0 (file:///home/nuchs/work/toys/hangman)
warning: unused variable: `secret`
  --> src/game.rs:12:16
   |
12 |     pub fn new(secret: String) -> Game {
   |                ^^^^^^
   |
   = note: #[warn(unused_variables)] on by default
   = note: to avoid this warning, consider using `_secret` instead

warning: unused variable: `guess`
  --> src/lib.rs:19:34
   |
19 |     pub fn make_guess(&mut self, guess: char) {}
   |                                  ^^^^^
   |
   = note: #[warn(unused_variables)] on by default
   = note: to avoid this warning, consider using `_guess` instead

warning: unused variable: `game`
  --> src/lib.rs:29:26
   |
29 |     pub fn render(&self, game: &Game) {}
   |                          ^^^^
   |
   = note: to avoid this warning, consider using `_game` instead

warning: unused variable: `game`
  --> src/lib.rs:31:30
   |
31 |     pub fn render_end(&self, game: &Game) {}
   |                              ^^^^
   |
   = note: to avoid this warning, consider using `_game` instead

    Finished dev [unoptimized + debuginfo] target(s) in 0.67 secs

Cargo-Process finished at Mon Feb 26 09:49:32
```

Rust is a bit grumpy with me for declaring a load of stuff that I'm
not using yet but it all compiles.

The next step is to split the code up into modules, in the normal
course of things this would probably be premature but rust's module
system is just different enough from what I'm used to that I want to
play with it a little bit.

## Rust module system

My mental model for how the module system works (and it may be wrong)
is that the modules are arranged in a set of trees. There is one tree
for each crate in the project and the root of the tree is in the file
which contains the entry point for the crate (main.rs for an
application, lib.rs for libraries). The nodes in the tree are all
modules. A module will be a leaf node if it does not contain any
module declarations.

Modules are declared using the ```mod``` keyword e.g. ```mod module_name``` within a
module.

The definitions of the modules can be in one of three locations, they
can be in the same file, directly following their declaration e.g.

```rust
mod my_module {
    // module definition
}
```

They can be in another file which is in the same directory and at the
same level as the file containing the module declaration. The name of
the file containing the definition must be the same as the module name
e.g.

```
$ ls src
lib.rs
my_module.rs
```

```rust
/* ----- lib.rs ----- */
mod my_module;

/* ----- my_module.rs ----- */

// content of my_module
```

One thing to note here is that we do not need to repeat the
declaration of my_module within the my_module file.

Finally a definition can be in another directory which is within the
same directory as the file containing the declaration. The name of the
directory must be the same as the module name and it must contain a
file called mod.rs which contains the definition e.g.

```
$ tree src
src
├── lib.rs
└── my_module/
    └── mod.rs
```

```rust
/* ----- lib.rs ----- */
mod my_module;

/* ----- my_module/mod.rs ----- */

// content of my_module
}
```

Again, note that we do not need to re-declare my_module within the
mod.rs file. 

Modules defined using the first two modes can only contain same file
declarations of modules. Modules defined using a mod.rs file can
contain modules defined using any of the modes. The reason for this is
to avoid ambiguity e.g.

```
$ tree src
src
├── lib.rs
└── my_other_module.rs
```

```rust
/* ----- lib.rs ----- */
// my_module is declared and defined inline
mod my_module { 

    // This is a sub module of my_module, it is only declared here
    // its definition must be in another file
    mod my_other_module;
}

// This is a sub module of the root module, it is only declared here
// its definition must be in another file
mod my_other_module;
```

The question is, does my_other_module.rs define the module declared in
the top level scope or the one in the my_module scope? 

## Carving into modules

As I noted at the start of the post I have a rough idea of how I want
to separate the concerns for this project; game logic, display code,
and word selection. I will create a module for each of these. First I
create the files

```
$ tree
.
├── Cargo.toml
└── src
    ├── game.rs
    ├── lib.rs
    ├── main.rs
    ├── render.rs
    └── words.rs
```

Next I move the skeleton code I've already written into the
appropriate location and add the necessary definitions to lib.rs

```rust
/* ----- lib.rs ----- */
mod game;
mod render;
mod words;

/* ----- game.rs ----- */
#[derive(Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Won,
    Lost
}

pub struct Game {
}

impl Game {
    pub fn new(secret: String) -> Game {
        Game {}
    }

    pub fn state(&self) -> GameState {
        GameState::Won
    }

    pub fn make_guess(&mut self, guess: char) {
    }
}

/* ----- render.rs ----- */
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

/* ----- words.rs ----- */
pub fn choose_secret() -> String {
    unimplemented!();
}

```

And check that it builds... 

```
Cargo-Process started at Mon Feb 26 10:37:57

cargo build 
   Compiling hangman v0.1.0 (file:///home/nuchs/work/toys/hangman)
error[E0412]: cannot find type `Game` in this scope
 --> src/render.rs:8:33
  |
8 |     pub fn render(&self, game: &Game) {
  |                                 ^^^^ not found in this scope
help: possible candidate is found in another module, you can import it into scope
  |
1 | use game::Game;
  |

error[E0412]: cannot find type `Game` in this scope
  --> src/render.rs:11:37
   |
11 |     pub fn render_end(&self, game: &Game) {
   |                                     ^^^^ not found in this scope
help: possible candidate is found in another module, you can import it into scope
   |
1  | use game::Game;
   |

error: aborting due to 2 previous errors

```

Whoops, now I've split the code into separate modules the render
implementation no longer has access to the ```Game``` struct. I'm
going to have the same problem with main.rs. So I need to add the
appropriate use statements.

```rust
/* ----- render.rs ----- */
use game::Game;

  // snip

/* ----- main.rs ----- */
extern crate hangman;

use hangman::game::Game;
use hangman::game::GameState;
use hangman::render::Console;

  // snip
```

And build again

```
cargo build 
   Compiling hangman v0.1.0 (file:///home/nuchs/work/toys/hangman)
error[E0603]: module `game` is private
 --> src/main.rs:3:5
  |
3 | use hangman::game::Game;
  |     ^^^^^^^^^^^^^^^^^^^

error[E0603]: module `game` is private
 --> src/main.rs:4:5
  |
4 | use hangman::game::GameState;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^

error[E0603]: module `render` is private
 --> src/main.rs:5:5
  |
5 | use hangman::render::Console;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^

error[E0603]: module `words` is private
 --> src/main.rs:8:18
  |
8 |     let secret = hangman::words::choose_secret();
  |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 4 previous errors

error: Could not compile `hangman`.

To learn more, run the command again with --verbose.

Cargo-Process exited abnormally with code 101 at Mon Feb 26 10:44:10
```

OK, rust doesn't like me trying to peer into the guts of a private
module. For now lets make them public.

```rust
/* ----- lib.rs ----- */
pub mod game;
pub mod render;
pub mod words;
```

And go again

```
-*- mode: cargo-process; default-directory: "~/work/toys/hangman/" -*-
Cargo-Process started at Mon Feb 26 10:46:35

cargo build 
   Compiling hangman v0.1.0 (file:///home/nuchs/work/toys/hangman)
warning: unused variable: `secret`
  --> src/game.rs:12:16
   |
12 |     pub fn new(secret: String) -> Game {
   |                ^^^^^^
   |
   = note: #[warn(unused_variables)] on by default
   = note: to avoid this warning, consider using `_secret` instead

warning: unused variable: `guess`
  --> src/game.rs:20:34
   |
20 |     pub fn make_guess(&mut self, guess: char) {
   |                                  ^^^^^
   |
   = note: #[warn(unused_variables)] on by default
   = note: to avoid this warning, consider using `_guess` instead

warning: unused variable: `game`
  --> src/render.rs:10:26
   |
10 |     pub fn render(&self, game: &Game) {
   |                          ^^^^
   |
   = note: to avoid this warning, consider using `_game` instead

warning: unused variable: `game`
  --> src/render.rs:13:30
   |
13 |     pub fn render_end(&self, game: &Game) {
   |                              ^^^^
   |
   = note: to avoid this warning, consider using `_game` instead

    Finished dev [unoptimized + debuginfo] target(s) in 0.67 secs

Cargo-Process finished at Mon Feb 26 10:46:36
```

And we're back to compiling.

## Access levels

I had a little 'aha' moment while I was doing this. I wasn't happy
with making the game and render modules public, what if I wanted to
make something public within the crate but I didn't want it to be
externally available. 

For example maybe I'd like to add something to the game module to make
rendering easier. I would need to make this item public to allow the render
module to access it. 

```rust
/* ----- game.rs ----- */

// This has not been declared public and so is not accessible
// outside of the module
struct CratePrivateThing;
 
/* ----- render.rs ----- */

// This will fail to build with the error:
// error[E0603]: struct `CratePrivateThing` is private
pub fn render(&self, game: &Game) {
    use game::CratePrivateThing;
    let _x = CratePrivateThing {};
}
```

As thing stand though, if I make it public, then anything which can
access the game module can access it.

```rust
/* ----- game.rs ----- */

// Adding pub on the front makes everything build
pub struct CratePrivateThing;
 
/* ----- Meanwhile in main.rs ... ----- */

pub fn make_mess_of_module_guts() {
    use hangman::game::CratePrivateThing;
    let _x = CratePrivateThing {}; // This wasn't supposed to be
                                   // accessible here
}
```

Effectively I would like a way to share stuff between modules within a
crate but hide from modules outside of the crate, a crate private mode
if you will (this is basically the equivalent of 'package private' in
Java or 'internal' in C#).

As it turns out there is a fairly straight forward way to do this, a
module can re-export members of its children to create a facade for
the module

```rust
// libs.rs
pub use game::Game;
pub use game::GameState;
pub use render::Console;
pub use words::choose_secret;

mod game;
mod render;
mod words;

// main.rs
extern crate hangman;

use hangman::Game;
use hangman::GameState;
use hangman::Console;
```

Now the crate is not exposing the game and render modules, meaning
users of the crate cannot access their content regardless of the
content's access level but the parts of the module they do need have
been exposed.

Check it builds

```
-*- mode: cargo-process; default-directory: "~/work/toys/hangman/" -*-
Cargo-Process started at Mon Feb 26 10:58:35

cargo build 
   Compiling hangman v0.1.0 (file:///home/nuchs/work/toys/hangman)
warning: unused variable: `secret`
  --> src/game.rs:12:16
   |
12 |     pub fn new(secret: String) -> Game {
   |                ^^^^^^
   |
   = note: #[warn(unused_variables)] on by default
   = note: to avoid this warning, consider using `_secret` instead

warning: unused variable: `guess`
  --> src/game.rs:20:34
   |
20 |     pub fn make_guess(&mut self, guess: char) {
   |                                  ^^^^^
   |
   = note: to avoid this warning, consider using `_guess` instead

warning: unused variable: `game`
  --> src/render.rs:10:26
   |
10 |     pub fn render(&self, game: &Game) {
   |                          ^^^^
   |
   = note: to avoid this warning, consider using `_game` instead

warning: unused variable: `game`
  --> src/render.rs:13:30
   |
13 |     pub fn render_end(&self, game: &Game) {
   |                              ^^^^
   |
   = note: to avoid this warning, consider using `_game` instead

    Finished dev [unoptimized + debuginfo] target(s) in 0.67 secs

Cargo-Process finished at Mon Feb 26 10:58:36
```

Excellent, everything compiles. That's probably a good place to stop
(not least because the baby has just woken up)
