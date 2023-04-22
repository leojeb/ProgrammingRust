
fn main() {
    println!("Hello, world!");
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t: u64 = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    {
        println!("求得最大公约数为:{n}");
        n
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15) ,1)
}
