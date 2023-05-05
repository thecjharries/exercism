use itertools::Itertools;

pub fn build_all_chains(input: Vec<&(u8, u8)>) -> Vec<Vec<&(u8, u8)>> {
    if 1 == input.len() {
        return vec![
            vec![(input[0].0, input[0].1)],
            vec![(input[0].1, input[0].0)],
        ];
    }
    let mut result = vec![];
    let smaller_chains = build_all_chains(input[1..].to_vec());
    for chain in smaller_chains {
        let mut new_chain = vec![(input[0].0, input[0].1)];
        new_chain.extend(chain);
        result.push(new_chain);
        let mut new_chain = vec![(input[0].1, input[0].0)];
        new_chain.extend(chain);
        result.push(new_chain);
    }
    result
}

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
