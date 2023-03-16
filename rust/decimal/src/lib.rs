use num_bigint::BigInt;
use std::ops::{Add, Mul, Sub};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, PartialEq)]
pub struct Decimal {
    negative: bool,
    mantissa: BigInt,
    exponent: i32,
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<std::cmp::Ordering> {
        if self.negative && !other.negative {
            return Some(std::cmp::Ordering::Less);
        } else if !self.negative && other.negative {
            return Some(std::cmp::Ordering::Greater);
        }
        let mut self_mantissa = self.mantissa.clone();
        let mut other_mantissa = other.mantissa.clone();
        let mut self_exponent = self.exponent;
        let mut other_exponent = other.exponent;
        while self_exponent > other_exponent {
            self_mantissa *= 10;
            self_exponent -= 1;
        }
        while other_exponent > self_exponent {
            other_mantissa *= 10;
            other_exponent -= 1;
        }
        if self.negative {
            Some(other_mantissa.cmp(&self_mantissa))
        } else {
            Some(self_mantissa.cmp(&other_mantissa))
        }
    }
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(self, other: Self) -> Self {
        if 0 == self.exponent && BigInt::from(0) == self.mantissa {
            return other;
        } else if 0 == other.exponent && BigInt::from(0) == other.mantissa {
            return self;
        }
        let mut self_mantissa = self.mantissa.clone();
        let mut other_mantissa = other.mantissa.clone();
        let mut self_exponent = self.exponent;
        let mut other_exponent = other.exponent;
        while self_exponent > other_exponent {
            self_mantissa *= 10;
            self_exponent -= 1;
        }
        while other_exponent > self_exponent {
            other_mantissa *= 10;
            other_exponent -= 1;
        }
        println!(
            "{} {} {} {}",
            self_mantissa, other_mantissa, self_exponent, other_exponent
        );
        let mut negative = if self > other {
            self.negative
        } else {
            other.negative
        };
        let mut mantissa = self_mantissa;
        let mut exponent = 0;
        if self.negative {
            if other.negative {
                mantissa += other_mantissa;
                exponent = self.exponent;
            } else {
                match mantissa.cmp(&other_mantissa) {
                    std::cmp::Ordering::Less => {
                        mantissa = other_mantissa - mantissa;
                        exponent = other.exponent;
                    }
                    std::cmp::Ordering::Equal => {
                        mantissa = BigInt::from(0);
                        exponent = 0;
                    }
                    std::cmp::Ordering::Greater => {
                        mantissa -= other_mantissa;
                        exponent = self.exponent;
                    }
                }
            }
        } else {
            if other.negative {
                match mantissa.cmp(&other_mantissa) {
                    std::cmp::Ordering::Less => {
                        mantissa = other_mantissa - mantissa;
                        exponent = other.exponent;
                    }
                    std::cmp::Ordering::Equal => {
                        mantissa = BigInt::from(0);
                        exponent = 0;
                    }
                    std::cmp::Ordering::Greater => {
                        mantissa -= other_mantissa;
                        exponent = self.exponent;
                    }
                }
            } else {
                mantissa += other_mantissa;
                exponent = self.exponent;
            }
        }
        while mantissa.clone() % 10 == BigInt::from(0) && exponent > 0 {
            mantissa /= 10;
            exponent -= 1;
        }
        println!("{} {} {}", mantissa, exponent, negative);
        Decimal {
            negative,
            mantissa,
            exponent,
        }
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, other: Self) -> Self {
        Decimal {
            negative: self.negative ^ other.negative,
            mantissa: self.mantissa * other.mantissa,
            exponent: self.exponent + other.exponent,
        }
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    fn sub(self, other: Self) -> Self {
        let mut other = other;
        other.negative = !other.negative;
        self + other
    }
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let mut negative = false;
        let mut mantissa = BigInt::from(0);
        let mut exponent = 0;
        let mut decimal_point = false;
        let mut decimal_point_offset = 0;
        for c in input.chars() {
            match c {
                '-' => {
                    if negative {
                        return None;
                    }
                    negative = true;
                }
                '.' => {
                    if decimal_point {
                        return None;
                    }
                    decimal_point = true;
                }
                '0'..='9' => {
                    if decimal_point {
                        decimal_point_offset += 1;
                    }
                    mantissa = mantissa * 10 + BigInt::from(c.to_digit(10).unwrap());
                }
                _ => return None,
            }
        }
        exponent = decimal_point_offset;
        while mantissa.clone() % 10 == BigInt::from(0) && exponent > 0 {
            mantissa /= 10;
            exponent -= 1;
        }
        Some(Decimal {
            negative,
            mantissa,
            exponent,
        })
    }
}
