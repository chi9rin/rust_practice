fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
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
