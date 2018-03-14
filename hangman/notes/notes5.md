# Learning Rust, Project 1: Hangman

## Part 5

There's only a couple of things left to do now, there are a couple of
configuration options I'd like to be able toset from the command line
and we need a way to get a user's guess

## User Input

I'll start first with getting the user's guess. This isn't going to be
anything particularly complicated but I do want to do some basic
validation on what they enter.

```rust
/* ----- main.rs ----- */

fn get_guess() -> char {
    loop {
        print!("Please enter your guess: ");

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if let Ok(guess) = parse(line) {
            return guess;
        }
    }
}

fn parse(line: String) -> Result<char, ()> {
    if line.len() != 1 {
        println!("Guesses should only contain one letter");
        return Err(());
    }

    let guess = line.chars().next().unwrap();

    if !guess.is_ascii_alphabetic() {
        println!("Only ASCII letters are supported");
        return Err(());
    }

    Ok(guess)
}
```

I feel like I've missed a trick here, I've been building this program a
block at a time and deferring assmebling them until the end. Now I've
reached that point things don't line up quite right.

This is a classic issue and one of the reasons people will advise to
build thin vertical slices of functionality and gradually build them
up. It forces early integration of the parts of the system and reveals
issues like these early when they are more straight forward to deal with.

In this case the program is very simple and the issue isn't hard to
fix. I haven't got a consistent way of handling bad guesses, this is
made a little tricky in that sometimes the fact that a guess is bad is
detected by the main event loop and sometimes by the library.

I'm not going to deal with this now; it's not breaking anything and
right now I'd just like to get the program running end to
end. It is a target for refactoring later and a reminder for my next
project about what appraoch to take when designing something. 

## Does it work?

The game should be playable now so lets give it a shot

```
            _ _ _ _ _ _ _
Guesses   : 
Remaining : 7
e
Please enter your guess: Guesses should only contain one letter
```

A few issues are immediately apparant

1.
The secret line could do with a label
2.
The validation of the user guess seems to be counting the carriage
return when checking the user has't entered too many characters at once.
3.
The request for the guess isn't printed until after you enter it

The first two points are trivial, the last point sounds like the std
out isn't being flushed.

Deal with the trivial stuff first
```rust
/* ----- render.rs ----- */

// in get_guess we can trim whitespace from the line before parsing it
    if let Ok(guess) = parse(line.trim()) {
        return guess;
    }
    
// When rendering the game state the secret line now has a label
    format!(
        r#"
Secret    : {}
Guesses   : {}
Remaining : {}
"#,
        obscured_secret(game),
        display_guesses(game.guesses()),
        game.remaining_guesses()
    )
```

Run the program again

```
# Last go of a winning game
Please enter your guess: 
Secret    : _ a c o n
Guesses   : a, c, k, n, o
Remaining : 6
b
Please enter your guess: Well done you guessed the secret (bacon)
Now all of your hopes and dreams will come true%   

# Last go of a losing game
Please enter your guess: 
Secret    : e _ _
Guesses   : e, q, r, t, u, w, y
Remaining : 1
i
Please enter your guess: You failed to guess the secret (egg)
Never mind, we can't all be winners.
Now if you could just join this queue for the fabulous Ark Ship B...%  
```

Getting better, there are some spacing issues and for reason a %
character at the end of the game complete messages. Quickly fix them
by adding some blank lines to the message templates.

```rust
Please enter your guess: 
Secret    : _ a c o n
Guesses   : a, c, e, k, n, o
Remaining : 5

b
Please enter your guess: Well done you guessed the secret (bacon)
Now all of your hopes and dreams will come true
```

Unexpectedly that fixed the weird trailing characters and it looks
better too. Onto the flushing issue; a quick google, and
[yes,](https://github.com/rust-lang/rust/issues/23818) it seems that
the ```print!``` macro doesn't flush the std out. I'll steal the work
around from the issue

```rust
fn get_guess() -> char {
    loop {
        print!("Please enter your guess: ");
        io::stdout().flush().unwrap();
        
// snip
```

Run it once again and...

```
Secret    : _ _ _
Guesses   : 
Remaining : 7

Please enter your guess: e

Secret    : e _ _
Guesses   : e
Remaining : 7

Please enter your guess: g
Well done you guessed the secret (egg)
Now all of your hopes and dreams will come true
```

Hazza! Everything is working the way I wanted :)

## Command line options

There are two command line options I want to support

*
-g number
Which sets how many wrong guesses you can have
*
-f filename
Which sets the file to select the secret from

I'm going to wrap these up in a configuration struct and use that to
initialise the game. As part of this I'm going to wrap up the multiple
calls it takes to currently takes to start the game into a single
entry point for the hangman library

```rust
/* ----- lib.rs ----- */
pub struct Config {
    pub total_guesses: u32,
    pub word_source: WordSource
}

pub fn start_game(config: Config) -> Game {
    let secret = choose_secret(config.word_source).unwrap();
    Game::new(secret, config.total_guesses)
}

/* ----- main.rs ----- */
use hangman::Config;
use hangman::GameState;
use hangman::WordSource;
use hangman::render;
use hangman::start_game;

fn main() {
    let config = parse_command_line();
    let mut game = start_game(config);

    print!("{}", render(&game));
    
// snip

fn parse_command_line() -> Config {
    Config { total_guesses: 7, word_source: WordSource::BuiltIn }
}
```


For this I'm going to use the CLAP (Command Line Argument Parser)
crate. I'll add it as a dependency to my Cargo.toml and then rebuild
hangman so that cargo will pull the crate down.


```toml
[dependencies]
rand = "0.4.2"
clap = "^2"
```

CLAP's API is geuninely a joy to use, it uses the
[builder pattern](https://en.wikipedia.org/wiki/Builder_pattern) to
create a specification of how the command line args should be parsed
which is incredibly intuative. I wrote the follwoign which worked
first time

```rust
/* ----- arg.rs ----- */

use clap::{App, Arg};

use hangman::{Config, WordSource};

fn parse_command_line() -> Config {
    let matches = App::new("Hangman")
        .version("0.1")
        .author("nuchs <sjbrown@live.co.uk>")
        .about("Plays a game of hangman")
        .arg(Arg::with_name("Guesses")
             .short("g")
             .long("guesses")
             .help("Number of wrong guesses a player can make before losing the game")
             .takes_value(true)
             .value_name("GUESSES"))
        .arg(Arg::with_name("Word File")
             .short("f")
             .long("file")
             .help("File to load secret word from")
             .takes_value(true)
             .value_name("FILE"))
        .get_matches();
        
    Config {
        total_guesses: parse_total_guesses(matches.value_of("Guesses")),
        word_source: parse_word_file(matches.value_of("Word File"))
    }
}

fn parse_total_guesses(total_guesses: Option<&str>) -> u32 {
    match total_guesses {
        Some(guesses) => guesses.parse().unwrap(),
        None => 7
    }
}

fn parse_word_file(word_file: Option<&str>) -> WordSource {
    match word_file {
        Some(file) => WordSource::FromFile(file.to_owned()),
        None => WordSource::BuiltIn
    }
}
```

There's not too much to say about this as spec reads pretty much
exactly like is was written in the code. The only minor fly in the
ointment is that CLAP is stringly typed meaning I have to convert the
arguments to the values I need outside of the framework (Although I
can see from the website that this is a planned change for v3 of the library).

What's really quite neat is that it automatically generates the help
and version messages for you based on your specification.

```
$ hangman -h
Hangman 0.1
nuchs <sjbrown@live.co.uk>
Plays a game of hangman

USAGE:
    hangman [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -g, --guesses <GUESSES>    Number of wrong guesses a player can make before losing the game
    -f, --file <FILE>          File to load secret word from
    
$ hangman -V
Hangman 0.1
```

I honestlt can't remember another time when it's been this easy to get
the argument parsing working on a program.

## The proverbial fan
