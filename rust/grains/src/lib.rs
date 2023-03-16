use memoize::memoize;

#[memoize(Capacity: 64)]
pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    if s == 1 {
        return 1;
    }
    2 * square(s - 1)
}

pub fn total() -> u64 {
    (1..=64).map(|s| square(s)).sum()
}
