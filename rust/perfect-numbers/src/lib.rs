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
    if 0 == num {
        return None;
    }
    let factors = factorize(num);
    let sum: u64 = factors.iter().sum();
    match sum.cmp(&num) {
        std::cmp::Ordering::Less => Some(Classification::Deficient),
        std::cmp::Ordering::Equal => Some(Classification::Perfect),
        std::cmp::Ordering::Greater => Some(Classification::Abundant),
    }
}
