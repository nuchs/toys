use clap::{App, Arg};

use hangman::{Config, WordSource};

pub fn parse_command_line() -> Config {
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
