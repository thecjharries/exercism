use rand::distributions::Uniform;
use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    if 0 == key.chars().filter(|c| c.is_ascii_lowercase()).count() {
        return None;
    }
    let mut result = String::new();
    let mut key_iter = key.chars().cycle();
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            let key_char = key_iter.next().unwrap();
            if key_char.is_ascii_lowercase() {
                result.push(((c as u8 + key_char as u8 - b'a' - b'a') % 26 + b'a') as char)
            } else {
                return None;
            }
        } else {
            result.push(c);
        }
    }
    Some(result)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if 0 == key.chars().filter(|c| c.is_ascii_lowercase()).count() {
        return None;
    }
    let mut result = String::new();
    let mut key_iter = key.chars().cycle();
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            let key_char = key_iter.next().unwrap();
            if key_char.is_ascii_lowercase() {
                result.push(((c as u8 + 26 - key_char as u8 + b'a' - b'a') % 26 + b'a') as char)
            } else {
                return None;
            }
        } else {
            result.push(c);
        }
    }
    Some(result)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key: String = rand::thread_rng()
        .sample_iter(Uniform::from(b'a'..=b'z'))
        .take(100)
        .map(|c| c as char)
        .collect();
    (key.clone(), encode(&key, s).unwrap())
}
