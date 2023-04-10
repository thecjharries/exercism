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
            if i + 7 >= binary.len() {
                byte = format!("{:0>8}", byte);
            }
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
    unimplemented!("Convert the list of bytes {bytes:?} to a list of numbers")
}
