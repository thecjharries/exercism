use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub struct Roman {
    pub decimal: u32,
    pub numeral: String,
}

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        write!(_f, "{}", self.numeral)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut numeral = String::new();
        let decimal = num;
        let mut num = num;
        while num > 0 {
            if num >= 1000 {
                numeral.push_str("M");
                num -= 1000;
            } else if num >= 900 {
                numeral.push_str("CM");
                num -= 900;
            } else if num >= 500 {
                numeral.push_str("D");
                num -= 500;
            } else if num >= 400 {
                numeral.push_str("CD");
                num -= 400;
            } else if num >= 100 {
                numeral.push_str("C");
                num -= 100;
            } else if num >= 90 {
                numeral.push_str("XC");
                num -= 90;
            } else if num >= 50 {
                numeral.push_str("L");
                num -= 50;
            } else if num >= 40 {
                numeral.push_str("XL");
                num -= 40;
            } else if num >= 10 {
                numeral.push_str("X");
                num -= 10;
            } else if num >= 9 {
                numeral.push_str("IX");
                num -= 9;
            } else if num >= 5 {
                numeral.push_str("V");
                num -= 5;
            } else if num >= 4 {
                numeral.push_str("IV");
                num -= 4;
            } else if num >= 1 {
                numeral.push_str("I");
                num -= 1;
            }
        }
        Roman { decimal, numeral }
    }
}
