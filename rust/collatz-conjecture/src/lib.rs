pub fn collatz(n: u64) -> Option<u64> {
    let mut n = n;
    let mut steps = 0;
    loop {
        if n == 1 {
            return Some(steps);
        }
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = match n.checked_mul(3) {
                Some(n) => match n.checked_add(1) {
                    Some(n) => n,
                    None => return None,
                },
                None => return None,
            };
        }
        steps += 1;
        if steps > 1000 {
            return None;
        }
    }
}
