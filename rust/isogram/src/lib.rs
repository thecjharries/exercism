use std::collections::BTreeSet;

pub fn check(candidate: &str) -> bool {
    let mut set = BTreeSet::new();
    for c in candidate.chars() {
        if c.is_alphabetic() {
            if set.contains(&c.to_lowercase().next().unwrap()) {
                return false;
            }
            set.insert(c.to_lowercase().next().unwrap());
        }
    }
    true
}
