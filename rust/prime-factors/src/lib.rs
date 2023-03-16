pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = Vec::new();
    let mut n = n;
    let mut i = 2;
    while i <= n {
        if n % i == 0 {
            prime_factors.push(i);
            n /= i;
        } else {
            i += 1;
        }
    }
    prime_factors
}
