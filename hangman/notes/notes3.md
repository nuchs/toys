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

## Pick a number, any number

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

That works and my test passes too. 

## Property testing

I don't like my test, it looks like a good candidates for something
that will fail intermittently e.g. if I had an off by one error in the
code that generates the index into to the word list then the
test would pass unless the RNG happened to produce the one value that
was out of bounds.

One way to address this is property based testing; You specify a
property of the code which and the testing framework
generates a selection of inputs and see's if the property holds for
all of them. This doesn't prove the property always holds but it gives
you more confidence than a single test. 

In Haskell the framework you can use for this is called quick check
and it has been ported to rust by
(https://github.com/BurntSushi/quickcheck)[burntsushi].

I'm only going to need QuickCheck for testing so add it to my list of
dev dependencies

```
[dev-dependencies]
quickcheck = "0.6"
```

And when I build and run the tests QuickCheck gets pulled down. To
make the crate available I pull it in to my lib.rs

```rust
#[cfg(test)] // Only need to include QuickCheck in test builds
#[macro_use] // A crate's macros aren't exported by default,
             // so need this too
extern crate quickcheck;
extern crate rand;
```

Build and huh?

```
error[E0282]: type annotations needed
   --> src/game.rs:118:9
    |
118 |         assert_eq!(sut.guesses(), &[]);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type for `_`
    |
    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error: aborting due to previous error

error: Could not compile `hangman`.
warning: build failed, waiting for other jobs to finish...
error: build failed
```

Well it has to be something to do with the extern statement I just
added. Commenting out the macro import doesn't change matters. What
about moving the extern to the words module where it's used.

```
cargo test 
   Compiling hangman v0.1.0 (file:///home/nuchs/work/toys/hangman)
error[E0468]: an `extern crate` loading macros must be at the crate root
 --> src/words.rs:3:1
  |
3 | extern crate quickcheck;
  | ^^^^^^^^^^^^^^^^^^^^^^^^
```

Not an option either (It does lend weight to putting all the extern
crate statements in the root though).

Here's the problem code:

```rust
#[test]
fn initially_no_guesses_should_have_been_made() {
    let sut = Game::new("stub".to_owned(), 1);

    assert_eq!(sut.guesses(), &[]);  // This line causes the error
}
```

So presumably the compiler is inferring that the type of one of the
parts could either be something from the QuickCheck crate or whatever
it was inferring it to be previously. Since it can't decide which it
errors. The only thing it could possibly be is the empty array
slice. Lets try following the compiler suggestion and adding a type
annotation

```rust
#[test]
fn initially_no_guesses_should_have_been_made() {
    let sut = Game::new("stub".to_owned(), 1);

    assert_eq!(sut.guesses(), &[]: &[char]);
}
```
```
error: type ascription is experimental (see issue #23416)
   --> src/game.rs:118:35
    |
118 |         assert_eq!(sut.guesses(), &[]: &[char]);
    |                                   ^^^^^^^^^^^^

error: aborting due to previous error

error: Could not compile `hangman`.
```

Ok not like that, like this?

```rust
#[test]
fn initially_no_guesses_should_have_been_made() {
let sut = Game::new("stub".to_owned(), 1);
let expected: &[char] = &[];

assert_eq!(sut.guesses(), expected);
}
```

```
Cargo-Process started at Wed Mar  7 07:03:54

cargo test 
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/hangman-6abb3098ec7cb84d
```

Hazza.

Not the trickiest thing in the world to fix but I didn't like the way
that happened. It *felt* like something was secretly brought into scope
without my say so, although it would be more accurate to say that
something was brought into scope without my intent. Either way
surprises are bad but it's something I'll just need to be careful of
in the future.

Back to property testing.

At the simplest level, you specify a function which will test if a
particular property holds and return true if it does and false if it
doesn't. You can then wrap it in the ```quickcheck!``` macro  like so

```rust
#[cfg(test)]
mod test {
    use super::*;

    quickcheck! {
        fn property_choosing_built_in_source_should_select_from_built_in_list() -> bool {
            
            let secret = choose_secret(WordSource::BuiltIn).unwrap();

            built_in_words().contains(&secret)
        }
    }
}
```

When you run cargo test your property function will be executed a
number of times (100 by default) and if any of the runs fail then this
will be reported. There's a lot you can configure about this and you
can have your property function take arguments which will be randomly
generated for you but for my purposes this is enough.

## Reading files

My next step is to see if can get my word list from a file. I'll
be cribbing heavily from the
(https://doc.rust-lang.org/book/second-edition/ch12-02-reading-a-file.html)[rust
book]

Interating with the file system is prone to failure, maybe the file
you're trying to read doesn't exist or maybe we don't have permission
to read it. Given this, it makes sense for the function which reads
the word list from a file to return a ```Result```, specifically an
```io::Result<T>``` which is what the functions used to read files all
return.

Again starting from the outside edge and working our way inwards we
get

```rust
// snip

use std::fs::File;
use std::io;
use std::io::prelude::*;

// snip

pub fn choose_secret(source: WordSource) -> io::Result<String> {
    let words = load_words(source)?;

    Ok(select_random(words))
}

fn load_words(source: WordSource) -> io::Result<Vec<String>> {
    match source {
        WordSource::BuiltIn             => Ok(built_in_words()),
        WordSource::FromFile(file_path) => words_from_file(&file_path)
    }
}

// snip


fn words_from_file(file_path: &str) -> io::Result<Vec<String>> {
    unimplemented!();
}

// snip
```

The expected format for the file is a single word on each line with
unix style line endings. Thus I need to read the contents of the file,
split it on a new line and store it in a Vec.

For simplicities sake I'm going to slurp up the whole file in one go,
realistically this is not a good approach when I don't know how big
the file may be. 

```rust
fn words_from_file(filename: &str) -> io::Result<Vec<String>> {
    let mut contents = String::new();
    let mut f = File::open(filename)?;
    f.read_to_string(&mut contents)?;

    Ok(contents
        .split("\n")
        .collect())
}
```

```
cargo build 
   Compiling hangman v0.1.0 (file:///home/nuchs/work/toys/hangman)
error[E0277]: the trait bound `std::vec::Vec<std::string::String>: std::iter::FromIterator<&str>` is not satisfied
  --> src/words.rs:42:10
   |
42 |         .collect())
   |          ^^^^^^^ a collection of type `std::vec::Vec<std::string::String>` cannot be built from an iterator over elements of type `&str`
   |
   = help: the trait `std::iter::FromIterator<&str>` is not implemented for `std::vec::Vec<std::string::String>`
```

So what's happened here? Looks like split returns references to
substrings from the contents and I'm trying to take ownership of what
they refer to. Naughty, but easily fixed.

```rust
fn words_from_file(filename: &str) -> io::Result<Vec<String>> {
    let mut contents = String::new();
    let mut f = File::open(filename)?;
    f.read_to_string(&mut contents)?;

    Ok(contents
        .split("\n")
        .map(|s| s.to_owned())
        .collect())
}
```

The next issue is that I'm performing no validation on the words
coming in. Lets add a filter that removes any invalid words. Start
with an outline:

```rust
fn words_from_file(filename: &str) -> io::Result<Vec<String>> {
    let mut contents = String::new();
    let mut f = File::open(filename)?;
    f.read_to_string(&mut contents)?;

    Ok(contents
        .split("\n")
        .filter(|w| is_valid_word(w))
        .map(|s| s.to_owned())
        .collect())
}

fn is_valid_word(word: &str) -> bool {
    true
}
```

And then the check; I'm going to be lazy and only accept alphabetic
ASCII characters

```rust
fn is_valid_word(word: &str) -> bool {
    word.chars().all(|c| c.is_ascii_alphabetic())
}
```

This is possibly over the top but to my eyes words_from_file has two
things going on; it has a section detailing how to read a file and a
section detailing how to extract the words from the file content. I'm
going to apply the extract method refactoring twice to pull each of
the sections into their own method. This should also keep the method
all at the same level of abstraction.

```rust
fn words_from_file(filename: &str) -> io::Result<Vec<String>> {
    let mut contents = read_file_contents(filename)?;
    let words = extract_words(contents);

    Ok(words)
}

fn read_file_contents(filename: &str) -> io::Result<String> {
    let mut contents = String::new();
    let mut f = File::open(filename)?;
    f.read_to_string(&mut contents)?;

    Ok(contents)
}

fn extract_words(mut word_list: String) -> Vec<String> {
    word_list
        .split("\n")
        .filter(|w| is_valid_word(w))
        .map(|s| s.to_owned())
        .collect()
}
```

## Integration tests

My rule of thumb when it comes to tests, is that once you need to
start testing with components external to the component you're working
on then you're doing integration testing whether that's with the file
system, the database or another application. 

The classification matters because, I expect my unit tests to be
quick, low level and run often whereas I my inetgration tests will
generally be at a higher level and I can accept them taking longer
becuase they won't be run as often.

Cargo supports integration tests via the tests directory
