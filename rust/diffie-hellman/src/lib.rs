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
