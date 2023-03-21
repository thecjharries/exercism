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
    unimplemented!("Calculate public key using prime numbers {p} and {g}, and private key {a}")
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    unimplemented!(
        "Calculate secret key using prime number {p}, public key {b_pub}, and private key {a}"
    )
}
