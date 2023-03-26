use std::str::FromStr;
use std::env;
use env::args as args;

fn main(){
    let mut numbers = Vec::new();
    for arg in args.skip(1) {
        numbers.push(u64::from_str(&arg)
            .expect("error parsing argument"));
    }
    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ... ");
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    println!("The greates common divisor of {:?} is {}".
            numbers, d)
}

