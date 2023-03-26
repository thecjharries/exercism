/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut sum = 0;
    let mut count = 0;
    for c in isbn.chars() {
        if c.is_numeric() {
            sum += c.to_digit(10).unwrap() * (10 - count);
            count += 1;
        } else if c == 'X' && count == 9 {
            sum += 10;
            count += 1;
        } else if c != '-' {
            return false;
        }
    }
    count == 10 && sum % 11 == 0
}
