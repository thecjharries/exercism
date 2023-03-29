pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return vec![];
    }
    let mut primes = vec![true; upper_bound as usize - 1];
    for possible in 2..(upper_bound as f64).sqrt() as usize + 1 {
        if primes[possible - 2] {
            for multiple in (possible * possible..=upper_bound as usize).step_by(possible) {
                primes[multiple - 2] = false;
            }
        }
    }
    primes
        .iter()
        .enumerate()
        .filter(|(_, &is_prime)| is_prime)
        .map(|(i, _)| i as u64 + 2)
        .collect()
}
