pub fn encode(source: &str) -> String {
    let mut index = 0;
    let mut result = String::new();
    while index < source.len() {
        let mut count = 1;
        let mut ch = source.chars().nth(index).unwrap();
        while index + 1 < source.len() && source.chars().nth(index + 1).unwrap() == ch {
            count += 1;
            index += 1;
        }
        if count == 1 {
            result.push(ch);
        } else {
            result.push_str(&count.to_string());
            result.push(ch);
        }
        index += 1;
    }
    result
}

pub fn decode(source: &str) -> String {
    unimplemented!("Return the run-length decoding of {source}.");
}
