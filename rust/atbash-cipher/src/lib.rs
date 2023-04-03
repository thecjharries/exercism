/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| {
            if c.is_ascii_digit() {
                c
            } else {
                (b'a' + b'z' - c.to_ascii_lowercase() as u8) as char
            }
        })
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| {
            if c.is_ascii_digit() {
                c
            } else {
                (b'a' + b'z' - c.to_ascii_lowercase() as u8) as char
            }
        })
        .collect()
}
