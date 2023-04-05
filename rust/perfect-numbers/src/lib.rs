#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn factorize(num: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    for i in 1..num {
        if num % i == 0 {
            factors.push(i);
        }
    }
    factors
}

pub fn classify(num: u64) -> Option<Classification> {
    unimplemented!("classify {num}");
}
