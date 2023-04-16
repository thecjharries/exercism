pub struct Luhn(Vec<char>);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if self.0.len() <= 1 {
            return false;
        }
        let mut sum = 0;
        for (i, &c) in self.0.iter().rev().enumerate() {
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
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<'a> From<&'a str> for Luhn {
    fn from(input: &'a str) -> Self {
        Luhn(input.chars().filter(|&c| c.is_alphanumeric()).collect())
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn::from(input.to_string())
    }
}
