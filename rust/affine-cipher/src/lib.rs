/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if 1 != gcd(a, 26) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let mut ciphertext = String::new();
    let plaintext = plaintext
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<String>();
    let mut counter = 0;
    for c in plaintext.chars() {
        if 0 != counter && 0 == counter % 5 {
            ciphertext.push(' ');
        }
        if c.is_alphabetic() {
            let c = c as u8 - b'a';
            let c = (a * c as i32 + b) % 26;
            let c = c as u8 + b'a';
            ciphertext.push(c as char);
            counter += 1;
        } else if c.is_numeric() {
            ciphertext.push(c);
            counter += 1;
        }
    }
    Ok(ciphertext)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if 1 != gcd(a, 26) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    Ok("".to_string())
}
