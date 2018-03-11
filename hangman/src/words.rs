use rand;
use rand::Rng;

use std::fs::File;
use std::io;
use std::io::prelude::*;

pub enum WordSource {
    BuiltIn,
    FromFile(String),
}

pub fn choose_secret(source: WordSource) -> io::Result<String> {
    let words = load_words(source)?;

    Ok(select_random(words))
}

fn load_words(source: WordSource) -> io::Result<Vec<String>> {
    match source {
        WordSource::BuiltIn => Ok(built_in_words()),
        WordSource::FromFile(filename) => words_from_file(&filename),
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

fn words_from_file(filename: &str) -> io::Result<Vec<String>> {
    let contents = read_file_contents(filename)?;
    let words = extract_words(contents);

    Ok(words)
}

fn read_file_contents(filename: &str) -> io::Result<String> {
    let mut contents = String::new();
    let mut f = File::open(filename)?;
    f.read_to_string(&mut contents)?;

    Ok(contents)
}

fn extract_words(word_list: String) -> Vec<String> {
    word_list
        .split("\n")
        .filter(|w| is_valid_word(w))
        .map(|s| s.to_owned())
        .collect()
}

fn is_valid_word(word: &str) -> bool {
    word.len() > 0 && word.chars().all(|c| c.is_ascii_alphabetic()) 
}

fn select_random(words: Vec<String>) -> String {
    let index = rand::thread_rng().gen_range(0, words.len());

    words[index].clone()
}

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
