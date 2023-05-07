pub fn calculate_price(groups: Vec<Vec<u32>>) -> u32 {
    let mut total = 0;
    for group in groups {
        match group.len() {
            1 => total += 800,
            2 => total += 1520,
            3 => total += 2160,
            4 => total += 2560,
            5 => total += 3000,
            _ => total += 0,
        }
    }
    total
}

pub fn lowest_price(books: &[u32]) -> u32 {
    unimplemented!("Find the lowest price of the bookbasket with books {books:?}")
}
