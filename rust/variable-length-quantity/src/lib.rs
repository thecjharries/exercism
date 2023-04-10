#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result = Vec::new();
    for value in values {
        let mut binary = format!("{:b}", value);
        if 0 != binary.len() % 7 {
            binary = "0".repeat(7 - binary.len() % 7) + &binary;
        }
        for i in (0..binary.len() - 7).step_by(7) {
            let mut byte = String::from("1");
            byte.push_str(&binary[i..i + 7]);
            result.push(u8::from_str_radix(&byte, 2).unwrap());
        }
        let mut byte = String::from("0");
        byte.push_str(&binary[binary.len() - 7..binary.len()]);
        result.push(u8::from_str_radix(&byte, 2).unwrap());
    }
    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        let mut number = String::new();
        let mut byte = format!("{:0>8b}", bytes[index]);
        while byte.starts_with("1") {
            number.push_str(&byte[1..]);
            index += 1;
            if index >= bytes.len() {
                return Err(Error::IncompleteNumber);
            }
            byte = format!("{:0>8b}", bytes[index]);
        }
        number.push_str(&byte[1..]);
        number = number.trim_start_matches('0').to_string();
        if number.is_empty() {
            number = "0".to_string();
        }
        if number.len() > 32 {
            return Err(Error::Overflow);
        }
        result.push(u32::from_str_radix(&number, 2).unwrap());
        index += 1;
    }
    Ok(result)
}
