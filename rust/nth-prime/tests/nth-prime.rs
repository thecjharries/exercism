use nth_prime as np;

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]
fn test_big_prime() {
    assert_eq!(np::nth(10_000), 104_743);
}

#[test]
fn test_is_prime() {
    assert!(!np::is_prime(1));
    assert!(np::is_prime(2));
    assert!(np::is_prime(3));
    assert!(np::is_prime(5));
    assert!(!np::is_prime(8));
}
