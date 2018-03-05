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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stuff() {

    }

}
