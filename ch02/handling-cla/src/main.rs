use std::str::FromStr;
use std::env;

fn main() {
    //notice we don't declare a type for numbers
    //because it'll be used as an input to gcd(), rust infers u64.
    //we could declare it explicitly as: let mut numbers = Vec::<u64>::new();
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        //we import FromStr trait, u64 implements it, we can use it here.
        //we use .expect() here to result the value of the Result.
        //Result returns either Ok(v) or Err(e)
        numbers.push(u64::from_str(&arg).expect("Error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }
    
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}",
                numbers, d);
}

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

//add unit testing.
#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15), 1);
    assert_eq!(gcd(2*3*5*11*17,
        3*7*11*13*19),
        3 * 11);
}
