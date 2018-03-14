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
            "arthur".to_owned(),
            "ford".to_owned(),
            "trillian".to_owned(),
            "zaphod".to_owned()];

        println!("{}", secret);
        words.contains(&secret)
    }

    fn property_non_ascii_words_should_be_filtered_out() -> bool {
        let testfile = "tests/words.txt".to_owned();
        let secret = choose_secret(WordSource::FromFile(testfile)).unwrap();

        secret != "123"
    }
}
