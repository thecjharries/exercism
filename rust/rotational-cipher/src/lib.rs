pub fn rotate(input: &str, key: i8) -> String {
    let key = key % 26;
    let mut result = String::new();
    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            let mut new_c = c as u8;
            if c.is_ascii_lowercase() {
                new_c = (((new_c - b'a') as i8 + key) as u8) % 26 + b'a';
            } else {
                new_c = (((new_c - b'A') as i8 + key) as u8) % 26 + b'A';
            }
            result.push(new_c as char);
        } else {
            result.push(c);
        }
    }
    result
}
