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
    FromFile(String)
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

The intention of the ```WordSource``` enum is that the user can
specify different places to load the words from. To make stuff a bit
simpler later when I want to start testing the program, I'd like to
able to use a short list of built in words.

```rust
fn load_words(source: WordSource) -> Vec<String> {
    match source {
        WordSource::BuiltIn             => built_in_words(),
        WordSource::FromFile(file_path) => unimplemented!()
    }
}

fn built_in_words() -> Vec<String> {
    vec![
        "bacon".to_owned(),
        "egg".to_owned(),
        "sausage".to_owned(),
        "klingon".to_owned(),
    ]
}
```

Now I have my word list I need to be able to choose one at random. For
this I'm going to crib from the first exercise in the
[https://doc.rust-lang.org/book/second-edition/ch02-00-guessing-game-tutorial.html](rust
book) on how to generate a random number. 

Random numbers are provided by the rand crate; I can include this
crate in my build by adding it to the Cargo.Toml file and cargo will
automatically download it and any transitive dependencies I now
have. The first thing I need to know is what version should I be
using, a quick visit to [https://crates.io/crates/rand](crates.io)
shows that the latest version (at time of writing) is 0.4.2.

```toml
// snip

[dependencies]
rand = "0.4.2"
```

And now when I build cargo pulls in the rand create. To make it
available in the words module I add the following to the top of the
file

```rust
// This makes the cotents of this crate available in this file
extern crate rand;

// This trait needs to be in scope to allow me to call one of
// it's methods (gen_range)
use rand::Rng
```

Quick check to see if I can build and 

```
cargo build 
   Compiling hangman v0.1.0 (file:///home/nuchs/work/toys/hangman)
error[E0432]: unresolved import `rand`
 --> src/words.rs:3:5
  |
3 | use rand::Rng;
  |     ^^^^ Did you mean `self::rand`?

warning: unused import: `rand::Rng`
 --> src/words.rs:3:5
  |
3 | use rand::Rng;
  |     ^^^^^^^^^
  |
  = note: #[warn(unused_imports)] on by default

error: aborting due to previous error
```

Did I mean ```self::rand```? I don't know lets try that

```rust
/* ----- words.rs ----- */
use self::rand::Rng;
```

And try building again

```
cargo build 
   Compiling hangman v0.1.0 (file:///home/nuchs/work/toys/hangman)

// Blah Blah, unused stuff warnings, Blah Blah

Cargo-Process finished at Mon Mar  5 20:21:52
```

Ok. Well full marks for fixing my problem but I've no idea what just
happend. Time to consult stack overflow ... and
[https://stackoverflow.com/a/33950291/3740219](this) seems to answer
my exact question. Basically the use statement takes an absolute path
from the root of the crate (lib.rs) and since there is no rand module
declared there I get the error. If I've understood this correctly then
there should be three possible solutions:

1.
Move the extern statement into lib.rs

2.
Use the full path to the rand module i.e. ```words::rand::Rng```

3.
Follow the compiler suggestion and prefix the argument to the use
statement with ```self::```, which I think is syntactic sugar for no
2.

I know no 3. works, lets try the other two, starting with number 2.

```rust
/* ----- words.rs ----- */
extern crate rand;

use words::rand::Rng;

// snip
```

That builds fine, now no 1.

```rust
/* ----- words.rs ----- */
use rand::Rng;

// snip

/* ----- lib.rs ----- */
extern crate rand;

// snip
```

That works too. I think I understand how this works now but it doesn't
feel like a terribly intuative choice. Given the inclredibly helpful
compiler error message, I would be willing to bet I'm not the only
person thrown by this. 

I now have three ways to do the same thing, which one to use? Option 3
seems like it's going to be better than option 2 in all circumstances,
it will almost always require less typing and (once you understand how
things work) it more clearly indicates what's going on. I have a
slight preference for option 1 though, it's a little less typing, it
makes more sense if you need to use the external crate in more than
one module and it gives us one place to look for all the extern crate
declarations.

Now that, that's sorted lets try and pick a word at random

```rust
fn select_random(words: Vec<String>) -> String {
    let index = rand::thread_rng().gen_range(0, words.len());

    words[index]
}
```

Kick of the build

```rust
cargo build 
   Compiling hangman v0.1.0 (file:///home/nuchs/work/toys/hangman)
error[E0433]: failed to resolve. Use of undeclared type or module `rand`
  --> src/words.rs:32:17
   |
32 |     let index = rand::thread_rng().gen_range(0, words.len());
   |                 ^^^^ Use of undeclared type or module `rand`
```

Doh! So much for me understanding this. Actually the error does make
sense, I've not introduced rand into this module with a use statemet
so the compiler doesn't know what I'm talking about. Easy enough to
fix:

```rust
use rand;
use rand::Rng;

// snip
```

And all now I have a new error

```
error[E0507]: cannot move out of indexed content
  --> src/words.rs:34:5
   |
34 |     words[index]
   |     ^^^^^^^^^^^^ cannot move out of indexed content

error: aborting due to previous error
```

I didn't expect that, the select_random function has taken ownership
of the words vector so I had assumed I could remove part of it at the
cost of losing the rest. I guess not, lets see if there is a method on
Vec that can help me.

[https://doc.rust-lang.org/std/vec/struct.Vec.html#method.remove](Remove)
nearly does what I want but it fills the whole caused by the removed
element afterwards. Given that I plan to ditch the Vec, this would be
nugatory. Perhaps the simplest thing would just to be to clone the string.

```rust
fn select_random(words: Vec<String>) -> String {
    let index = rand::thread_rng().gen_range(0, words.len());

    words[index].clone()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn the_secret_should_be_chosen_from_the_built_in_list_when_specified() {
        let secret = choose_secret(WordSource::BuiltIn);

        assert!(built_in_words().contains(&secret));
    }

}
```

That works and my test passes too. Now to see if I can get my word
list from a file.




I don't like my tests, they look like good candidates for things that
will work for some (or even most) of the time but might occasionally
fail e.g. if I had an off by one error in the code that generates the
random index into to the word list then the test would pass unless the
RNG happened to produce the one bad value. 

One way to address propety based testing where your test specifies a
property of the code which should hold and the testing framework
generates a selection of inputs and see's if the property is true for
all of them. This doesn't prove the property is true but it gives you
more confidence than a single test and the set of test values
generated can be tailored to ensure they contain all the boundary
conditions.

In Haskell the framework you can use for this is called quick check
and it has been ported to rust


