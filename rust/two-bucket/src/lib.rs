#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

pub fn gcd(a: u8, b: u8) -> u8 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if goal > capacity_1 && goal > capacity_2 {
        return None;
    }
    if 0 != goal % gcd(capacity_1, capacity_2) {
        return None;
    }
    None
}
