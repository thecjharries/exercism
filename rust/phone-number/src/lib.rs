pub fn number(user_number: &str) -> Option<String> {
    let mut number = user_number
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>();
    if number.len() == 11 && number.starts_with('1') {
        number.remove(0);
    }
    if 10 == number.len() {
        let area_code = &number[0..3];
        let exchange_code = &number[3..6];
        if !area_code.starts_with('0') && !area_code.starts_with('1') {
            if !exchange_code.starts_with('0') && !exchange_code.starts_with('1') {
                return Some(number);
            }
        }
    }
    None
}
