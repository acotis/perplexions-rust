
use std::sync::LazyLock;

static WORDS: LazyLock<Vec<String>> = LazyLock::new(|| {
    let mut words = 
        include_str!("words.txt")
            .to_ascii_uppercase()
            .lines()
            .map(str::to_owned)
            .collect::<Vec<String>>();
    words.sort();
    words
});

pub fn is_valid_word(word: String) -> bool {
    WORDS.binary_search(&word).is_ok()
}

pub fn remove_last_word_tried() { /* do nothing */ }
pub fn add_last_word_tried() { /* do nothing */ }

pub fn initialize() {
    LazyLock::force(&WORDS);
}

fn parse_levels(level_data: &'static str) -> impl Iterator<Item=String> {
    level_data
        .split("——————————")
        .map(str::to_ascii_uppercase)
        .map(|level|
            level.lines()
                 .map(|line| line.split("#").nth(0).unwrap())
                 .map(str::trim_end)
                 .collect::<Vec<_>>()
                 .join("\n")
        )
        .filter(|split| split.trim() != "")
}

pub fn levels() -> impl Iterator<Item=String> {
    parse_levels(include_str!("levels.txt"))
}

