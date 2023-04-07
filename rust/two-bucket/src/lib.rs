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

#[cfg(not(tarpaulin_include))]
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
    if capacity_1 == goal {
        let moves = if start_bucket == &Bucket::One { 1 } else { 2 };
        let other_bucket = if start_bucket == &Bucket::One {
            0
        } else {
            capacity_2
        };
        return Some(BucketStats {
            moves,
            goal_bucket: Bucket::One,
            other_bucket,
        });
    }
    if capacity_2 == goal {
        let moves = if start_bucket == &Bucket::Two { 1 } else { 2 };
        let other_bucket = if start_bucket == &Bucket::Two {
            0
        } else {
            capacity_1
        };
        return Some(BucketStats {
            moves,
            goal_bucket: Bucket::Two,
            other_bucket,
        });
    }
    let mut bucket_1 = 0;
    let mut bucket_2 = 0;
    let mut moves = 0;
    let goal_bucket;
    let other_bucket;
    loop {
        if start_bucket == &Bucket::One {
            if bucket_1 == 0 {
                bucket_1 = capacity_1;
                moves += 1;
            } else if bucket_2 == capacity_2 {
                bucket_2 = 0;
                moves += 1;
            } else {
                let diff = capacity_2 - bucket_2;
                if diff >= bucket_1 {
                    bucket_2 += bucket_1;
                    bucket_1 = 0;
                } else {
                    bucket_1 -= diff;
                    bucket_2 = capacity_2;
                }
                moves += 1;
            }
        } else {
            if bucket_2 == 0 {
                bucket_2 = capacity_2;
                moves += 1;
            } else if bucket_1 == capacity_1 {
                bucket_1 = 0;
                moves += 1;
            } else {
                let diff = capacity_1 - bucket_1;
                if diff >= bucket_2 {
                    bucket_1 += bucket_2;
                    bucket_2 = 0;
                } else {
                    bucket_2 -= diff;
                    bucket_1 = capacity_1;
                }
                moves += 1;
            }
        }
        if bucket_1 == goal {
            goal_bucket = Bucket::One;
            other_bucket = bucket_2;
            break;
        }
        if bucket_2 == goal {
            goal_bucket = Bucket::Two;
            other_bucket = bucket_1;
            break;
        }
    }
    Some(BucketStats {
        moves,
        goal_bucket,
        other_bucket,
    })
}
