pub fn abbreviate(phrase: &str) -> String {
    let exploded = phrase.split(|c: char| !c.is_alphabetic() && '\'' != c);
    let mut result = String::new();
    for word in exploded {
        if "" == word {
            continue;
        }
        let mut possible_addition = String::new();
        let mut chars = word.chars();
        possible_addition.push(chars.next().unwrap().to_uppercase().next().unwrap());
        for c in chars {
            if c.is_uppercase() {
                possible_addition.push(c);
            }
        }
        if possible_addition == "" {
            continue;
        }
        if possible_addition == word {
            result.push(possible_addition.chars().next().unwrap());
        } else {
            result.push_str(&possible_addition);
        }
    }
    result
}
