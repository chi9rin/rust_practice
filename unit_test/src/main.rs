fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0, "m and n should be > 0");
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}


fn main() {
    let m = 55;
    let n = 30;
    println!("GCD of {} and {} is {}", m, n, gcd(m, n));
}


#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
               3 * 11);
}


#[test]
#[should_panic(expected = "m and n should be > 0")]
fn test_gcd_zero() {
    assert_eq!(gcd(0, 1), 0)
}
