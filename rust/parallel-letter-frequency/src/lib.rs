use std::cmp::min;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input = input.join("");
    let mut data = input.chars();
    let worker_count = min(worker_count, input.len());
    let mut chunk_size = (input.len() / worker_count).max(1);
    if input.len() > chunk_size * worker_count {
        chunk_size += 1;
    }
    let (tx, rx) = mpsc::channel();
    for _ in 0..worker_count {
        let tx = tx.clone();
        let chunk = data.by_ref().take(chunk_size).collect::<Vec<char>>();
        thread::spawn(move || {
            let mut map = HashMap::new();
            for c in chunk {
                if c.is_alphabetic() {
                    *map.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
                }
            }
            tx.send(map).unwrap();
        });
    }
    drop(tx);
    let mut map = HashMap::new();
    for m in rx {
        for (k, v) in m {
            *map.entry(k).or_insert(0) += v;
        }
    }
    map
}
