pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum: u32 = 0;
    let mut n = num;
    let mut digits = 0;
    while n > 0 {
        n /= 10;
        digits += 1;
    }
    n = num;
    while n > 0 {
        sum = match sum.overflowing_add((n % 10).pow(digits)) {
            (sum, false) => sum,
            (_, true) => return false,
        };
        n /= 10;
    }
    sum == num
}
