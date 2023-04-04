use memoize::memoize;

#[derive(Debug, PartialEq)]
pub struct PascalsTriangle(Vec<Vec<u32>>);

#[memoize]
pub fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[memoize]
pub fn binomial_coefficient(n: u32, k: u32) -> u32 {
    factorial(n) / (factorial(k) * factorial(n - k))
}

impl PascalsTriangle {
    pub fn row(n: u32) -> Vec<u32> {
        (0..=n).map(|k| binomial_coefficient(n, k)).collect()
    }

    pub fn new(row_count: u32) -> Self {
        PascalsTriangle((0..row_count).map(|n| Self::row(n)).collect())
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}
