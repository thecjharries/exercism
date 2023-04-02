/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let forward_digits = value.to_string();
        let reverse_digits = forward_digits.chars().rev().collect::<String>();
        if forward_digits == reverse_digits {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_palindrome = None;
    let mut max_palindrome = None;
    for i in min..=max {
        for j in i..=max {
            let product = i * j;
            if let Some(palindrome) = Palindrome::new(product) {
                if min_palindrome.map_or(true, |min| palindrome < min) {
                    min_palindrome = Some(palindrome);
                }
                if max_palindrome.map_or(true, |max| palindrome > max) {
                    max_palindrome = Some(palindrome);
                }
            }
        }
    }
    min_palindrome.zip(max_palindrome)
}
