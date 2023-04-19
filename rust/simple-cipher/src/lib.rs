pub fn encode(key: &str, s: &str) -> Option<String> {
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
    unimplemented!("Use {key} to decode {s} using shift cipher")
}

pub fn encode_random(s: &str) -> (String, String) {
    unimplemented!(
        "Generate random key with only a-z chars and encode {s}. Return tuple (key, encoded s)"
    )
}
