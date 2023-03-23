use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();
    for a in 1..sum - 2 {
        for b in a..sum - 1 {
            let c = match sum.checked_sub(a + b) {
                Some(c) => c,
                None => continue,
            };
            if a * a + b * b == c * c {
                triplets.insert([a, b, c]);
            }
        }
    }
    triplets
}
