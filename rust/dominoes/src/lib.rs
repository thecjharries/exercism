use itertools::Itertools;

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        return Some(vec![]);
    }
    for mut possible_chain in input.into_iter().permutations(input.len()).unique() {
        let result: Vec<(u8, u8)> = possible_chain.iter().map(|&x| *x).collect();
        let first_element = possible_chain.first().copied().unwrap();
        let last_element = possible_chain.last().copied().unwrap();
        if first_element.0 != last_element.1 {
            continue;
        }
        let mut current_element = possible_chain.pop().unwrap();
        while let Some(next_element) = possible_chain.pop() {
            // println!("{:?} - {:?}", current_element, next_element);
            if current_element.0 != next_element.1 {
                break;
            }
            current_element = next_element;
        }
        if possible_chain.is_empty() {
            return Some(result);
        }
    }
    None
}
