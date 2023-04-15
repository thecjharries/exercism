pub fn encode(n: u64) -> String {
    unimplemented!("Say {n} in English.");
}

format(n: u64, divisor: u64, order: &str) -> String {
    format!("{} {} {}", encode(n / divisor), order, encode(n % divisor))
}
