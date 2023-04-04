use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut set = HashSet::new();
    for c in sentence.to_lowercase().chars() {
        if b'a' <= c as u8 && c as u8 <= b'z' {
            set.insert(c);
        }
    }
    set.len() == 26
}
