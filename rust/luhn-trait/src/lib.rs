pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let mut sum = 0;
        let mut binding = self.to_string();
        binding.retain(|c| c.is_alphanumeric());
        let mut chars = binding.chars().rev();
        let mut i = 0;
        while let Some(c) = chars.next() {
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
            i += 1;
        }
        sum % 10 == 0
    }
}
