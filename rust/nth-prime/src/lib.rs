pub fn is_prime(i: u32) -> bool {
    if i < 2 {
        return false;
    }
    if i == 2 {
        return true;
    }
    if i % 2 == 0 {
        return false;
    }
    let mut j = 3;
    while j * j <= i {
        if i % j == 0 {
            return false;
        }
        j += 2;
    }
    true
}

pub fn nth(n: u32) -> u32 {
    if 0 == n {
        return 2;
    }
    let mut count = 1;
    let mut i = 3;
    while count <= n {
        if is_prime(i) {
            count += 1;
        }
        i += 2;
    }
    i - 2
}
