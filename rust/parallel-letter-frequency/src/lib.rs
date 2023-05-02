use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let frequencies: Rc<Mutex<HashMap<char, usize>>> = Rc::new(Mutex::new(HashMap::new()));
    let mut handles = Vec::new();
    for text in input.iter() {
        let frequencies = Rc::clone(&frequencies);
        let handle = thread::spawn(move || {
            let mut map = frequencies.lock().unwrap();
            for c in text.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
                let counter = map.entry(c).or_insert(0);
                *counter += 1;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let mut map = frequencies.lock().unwrap();
    let mut result = HashMap::new();
    for (k, v) in map.drain() {
        result.insert(k, v);
    }
    result
}
