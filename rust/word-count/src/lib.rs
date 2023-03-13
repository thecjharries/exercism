use std::collections::HashMap;

const IGNORED_PUNCTUATION: [char; 14] = [
    '!', '&', '@', '$', '%', '^', '&', ':', ',', '.', '-', '_', ';', '"',
];

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut exploded = words
        .split(|c: char| c.is_whitespace() || IGNORED_PUNCTUATION.contains(&c))
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut lower_word = s.to_lowercase();
            lower_word = match lower_word.strip_suffix("'") {
                Some(s) => s.to_string(),
                None => lower_word,
            };
            lower_word = match lower_word.strip_prefix("'") {
                Some(s) => s.to_string(),
                None => lower_word,
            };
            lower_word
        })
        .collect::<Vec<String>>();
    exploded.sort();
    exploded.iter().fold(HashMap::new(), |mut acc, s| {
        *acc.entry(s.to_string()).or_insert(0) += 1;
        acc
    })
}
