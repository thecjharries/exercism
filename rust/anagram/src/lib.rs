use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let word = word.to_lowercase();
    let mut word_chars: Vec<char> = word.chars().collect();
    word_chars.sort();
    for possible_anagram in possible_anagrams {
        let mut chars: Vec<char> = possible_anagram.to_lowercase().chars().collect();
        chars.sort();
        if word_chars == chars && word != possible_anagram.to_lowercase() {
            anagrams.insert(*possible_anagram);
        }
    }
    anagrams
}
