#[cfg(not(tarpaulin_include))]
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
    let mut groups: Vec<Vec<u32>> = Vec::new();
    for book in books {
        let mut lowest_price = u32::MAX;
        let mut lowest_price_group = usize::MAX;
        for (i, _group) in groups.iter().enumerate() {
            let mut new_groups = groups.clone();
            if new_groups[i].contains(book) {
                continue;
            }
            new_groups[i].push(*book);
            let new_price = calculate_price(new_groups);
            if new_price < lowest_price {
                lowest_price = new_price;
                lowest_price_group = i;
            }
        }
        let mut new_groups = groups.clone();
        new_groups.push(vec![*book]);
        let new_price = calculate_price(new_groups);
        if new_price < lowest_price {
            lowest_price_group = groups.len();
        }
        if lowest_price_group < groups.len() {
            groups[lowest_price_group].push(*book);
        } else {
            groups.push(vec![*book]);
        }
    }
    calculate_price(groups)
}
