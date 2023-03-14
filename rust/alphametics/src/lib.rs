use itertools::Itertools;
use std::collections::HashMap;

// Note this brute forces the permutations. There are better ways to do this.
// The tests time out on exercism.
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let characters = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .unique()
        .collect::<Vec<char>>();
    let binding = input.replace("==", "=").replace(" ", "");
    let input_chunks: Vec<String> = binding
        .split(|c| '=' == c || '+' == c)
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();
    let mut chunks = input_chunks.clone();
    for permutation in (0..10).permutations(characters.len()) {
        chunks = input_chunks.clone();
        let mut map = HashMap::new();
        let mut valid = true;
        for (i, c) in characters.iter().enumerate() {
            map.insert(*c, permutation[i]);
            let mut new_chunks = Vec::new();
            for chunk in chunks.iter() {
                let mut new_chunk = chunk
                    .chars()
                    .map(|c| {
                        if let Some(digit) = map.get(&c) {
                            digit.to_string()
                        } else {
                            c.to_string()
                        }
                    })
                    .collect::<String>();
                if new_chunk.starts_with('0') && new_chunk.len() > 1 {
                    valid = false;
                    break;
                }
                new_chunks.push(new_chunk);
            }
            if valid {
                chunks = new_chunks.clone();
            } else {
                break;
            }
        }
        if !valid {
            continue;
        }
        let result = chunks.pop().unwrap().parse::<u64>().unwrap();
        let sum = chunks
            .iter()
            .map(|s| s.parse::<u64>().unwrap())
            .sum::<u64>();
        if result == sum {
            return Some(map);
        }
    }
    None
}
