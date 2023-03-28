fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' => true,
        _ => false,
    }
}

pub fn translate(input: &str) -> String {
    let exploded = input.split_whitespace().collect::<Vec<&str>>();
    if 1 < exploded.len() {
        let mut result = String::new();
        for word in exploded {
            result.push_str(&translate(word));
            result.push(' ');
        }
        return result.trim().to_string();
    }
    if input.starts_with("xr") || input.starts_with("yt") {
        return format!("{}ay", input);
    }
    if input.starts_with("y") {
        return format!("{}yay", &input[1..]);
    }
    let mut is_q = false;
    let mut index = 0;
    for (i, c) in input.chars().enumerate() {
        if 'q' == c {
            is_q = true;
        }
        if is_vowel(c) {
            if i == 0 {
                return format!("{}ay", input);
            }
            if is_q && 'u' == c {
                return format!("{}{}ay", &input[i + 1..], &input[..i + 1]);
            }
            index = i;
            break;
        }
    }
    format!("{}{}ay", &input[index..], &input[..index])
}
