# Learning Rust, Project 1: Hangman

## Part 3

The next section I'm going to focus on is the selection of the secret
for the game. There are two points of interest that I intend to
address in this module are file io and using external crates.

## Module outline

As before I sketch out a high level outline of what I want the module
to look like.

```rust
/* ----- words.rs ----- */
pub enum WordSource {
    BuiltIn,
    FromFile
}

pub fn choose_secret(source: WordSource) -> String {
    let words = load_words(source);

    select_random(words)
}

fn load_words(source: WordSource) -> Vec<String> {
    unimplemented!();
}

fn select_random(words: Vec<String>) -> String {
    unimplemented!();
}

/* ----- main.rs ----- */
// snip

use hangman::WordSource;

fn main() {
    let secret = hangman::choose_secret(WordSource::BuiltIn);
    let mut game = Game::new(secret, 7);

// snip

/* ----- lib.rs ----- */
// snip
pub use words::WordSource;
// snip
```

next up , using a crate from crates.io reading files
configure what value to use

FIrts use the rand crate

[dev-dependencies]
quickcheck = "0.6"
