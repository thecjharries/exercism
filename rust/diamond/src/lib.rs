pub fn get_diamond(c: char) -> Vec<String> {
    if c == 'A' {
        return vec!["A".to_string()];
    }
    let mut result = Vec::new();
    let length = c as u8 - 'A' as u8 + 1;
    for i in 0..length {
        let current = 'A' as u8 + i;
        let mut line = format!(
            "{}{}{}",
            " ".repeat((length - i - 1) as usize),
            (current as char).to_string(),
            " ".repeat((i) as usize)
        );
        let mut reverse = line.clone();
        reverse.pop();
        reverse = reverse.chars().rev().collect();
        line.push_str(&reverse);
        result.push(line);
    }
    for i in (0..length - 1).rev() {
        result.push(result[i as usize].clone());
    }
    result
}
