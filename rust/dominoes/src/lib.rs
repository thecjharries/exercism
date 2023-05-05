use itertools::Itertools;

pub fn build_all_chains(input: Vec<&(u8, u8)>) -> Vec<Vec<(u8, u8)>> {
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
        new_chain.extend(chain.clone());
        result.push(new_chain);
        let mut new_chain = vec![(input[0].1, input[0].0)];
        new_chain.extend(chain.clone());
        result.push(new_chain);
    }
    result
}

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        return Some(vec![]);
    }
    for permutation in input.into_iter().permutations(input.len()).unique() {
        let mut possible_chains = build_all_chains(permutation.to_vec());
        for chain in possible_chains.iter_mut() {
            let result = chain.clone();
            if chain[0].0 != chain[chain.len() - 1].1 {
                continue;
            }
            let mut current_element = chain.pop().unwrap();
            let mut possible = true;
            while let Some(previous_element) = chain.pop() {
                if current_element.0 != previous_element.1 {
                    possible = false;
                    break;
                }
                current_element = previous_element;
            }
            if possible {
                return Some(result);
            }
        }
    }
    None
}
