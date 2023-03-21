pub fn series(digits: &str, len: usize) -> Vec<String> {
    if 0 == len {
        return vec!["".to_string(); digits.len() + 1];
    }
    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|w| w.iter().collect())
        .collect()
}
