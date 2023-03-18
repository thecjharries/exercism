pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = Vec::new();
    for factor in factors {
        if *factor == 0 {
            continue;
        }
        let mut multiple = *factor;
        while multiple < limit {
            multiples.push(multiple);
            multiple += factor;
        }
    }
    multiples.sort();
    multiples.dedup();
    multiples.iter().sum()
}
