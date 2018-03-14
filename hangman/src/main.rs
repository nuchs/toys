extern crate clap;
extern crate hangman;

mod args;

use std::io;
use std::io::prelude::*; 

use args::parse_command_line;
use hangman::{GameState, render, start_game};

fn main() {
    let config = parse_command_line();
    let mut game = start_game(config);

    print!("{}", render(&game));

    while game.state() == GameState::InProgress {
        let guess = get_guess();
        game.make_guess(guess).unwrap();
        print!("{}", render(&game));
    }
}

fn get_guess() -> char {
    loop {
        print!("Please enter your guess: ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if let Ok(guess) = parse(line.trim()) {
            return guess;
        }
    }
}

fn parse(line: &str) -> Result<char, ()> {
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
