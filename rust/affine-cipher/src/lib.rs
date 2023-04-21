/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if let (1, _, _) = extended_gcd(a, 26) {
    } else {
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

pub fn modular_inverse(a: i32, m: i32) -> Result<i32, AffineCipherError> {
    let (g, x, _) = extended_gcd(a, m);
    if 1 == g {
        Ok((x % m + m) % m)
    } else {
        Err(AffineCipherError::NotCoprime(a))
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if let (1, _, _) = extended_gcd(a, 26) {
    } else {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let mut plaintext = String::new();
    let ciphertext = ciphertext
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<String>();
    let a_inv = modular_inverse(a, 26)?;
    for c in ciphertext.chars() {
        if c.is_alphabetic() {
            let c = c as u8 - b'a';
            let c = ((a_inv * (c as i32 - b)) % 26 + 26) % 26;
            println!("c: {}", c);
            let c = c as u8 + b'a';
            plaintext.push(c as char);
        } else if c.is_numeric() {
            plaintext.push(c);
        }
    }
    Ok(plaintext)
}
