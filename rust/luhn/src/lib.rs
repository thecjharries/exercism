/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(" ", "");
    if code.len() <= 1 {
        return false;
    }
    let mut sum = 0;
    for (i, c) in code.chars().rev().enumerate() {
        if !c.is_digit(10) {
            return false;
        }
        let mut digit = c.to_digit(10).unwrap();
        if i % 2 == 1 {
            digit *= 2;
            if digit > 9 {
                digit -= 9;
            }
        }
        sum += digit;
    }
    sum % 10 == 0
}
