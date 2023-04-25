use itertools::Itertools;

pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(_max_weight: u32, _items: &[Item]) -> u32 {
    let items = _items
        .iter()
        .filter(|item| item.weight <= _max_weight)
        .collect_vec();
    let powerset = items.iter().powerset();
    let mut max_value = 0;
    for subset in powerset {
        let total_weight: u32 = subset.iter().map(|item| item.weight).sum();
        if total_weight <= _max_weight {
            let total_value: u32 = subset.iter().map(|item| item.value).sum();
            if total_value > max_value {
                max_value = total_value;
            }
        }
    }
    max_value
}
