use memoize::memoize;

pub struct PascalsTriangle;

impl PascalsTriangle {
    #[memoize]
    pub factorial(n: u32) -> u32 {
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

    pub fn new(row_count: u32) -> Self {
        unimplemented!("create Pascal's triangle with {row_count} rows");
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        unimplemented!();
    }
}
