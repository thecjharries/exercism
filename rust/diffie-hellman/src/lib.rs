pub fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut base = base as u128;
    let mut exp = exp as u128;
    let modulus = modulus as u128;
    let mut result = 1 as u128;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp = exp >> 1;
        base = (base * base) % modulus;
    }
    result as u64
}

pub fn private_key(p: u64) -> u64 {
    p / 2
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_exp(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_exp(b_pub, a, p)
}
