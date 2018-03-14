#[macro_use]
extern crate quickcheck;
extern crate hangman;

use hangman::WordSource;
use hangman::choose_secret;

quickcheck! {
    fn property_choosing_file_source_should_select_from_specified_file() -> bool {
        
        let testfile = "tests/words.txt".to_owned();
        let secret = choose_secret(WordSource::FromFile(testfile)).unwrap();
        let words = vec![
            "Arthur".to_owned(),
            "Ford".to_owned(),
            "Trillian".to_owned(),
            "Zaphod".to_owned()];

        words.contains(&secret)
    }

    fn words_containing_non_ascii_letter_characters_should_be_filtered_out() -> bool {
        let testfile = "tests/words.txt".to_owned();
        let secret = choose_secret(WordSource::FromFile(testfile)).unwrap();

        secret != "123"
    }
}
