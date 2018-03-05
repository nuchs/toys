use rand;
use rand::Rng;

pub enum WordSource {
    BuiltIn,
    FromFile(String)
}

pub fn choose_secret(source: WordSource) -> String {
    let words = load_words(source);

    select_random(words)
}

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
