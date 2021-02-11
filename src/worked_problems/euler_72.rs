// Problem: How many elements would be contained in the set of reduced proper fractions for d ≤ 1,000,000?

/*
*/
use crate::aux_funcs::{totient_sieve};

pub fn euler72() -> u64 {
    let mut out = 0;
    let mut tot = totient_sieve();
    for _ in 0..=999_998 {
        out += tot.next().unwrap()
    }
    out
}

pub fn euler72_example() {
    println!("\nProblem: How many elements would be contained in the set of reduced proper fractions for d ≤ 1,000,000?");
    println!("\n\nThe length of Farey Sequences can be found by the summation of Euler's totient function.");
    let s = "
use crate::aux_funcs::{totient_sieve};

pub fn euler72() -> u64 {
    let mut out = 0;
    let mut tot = totient_sieve();
    for _ in 0..=999_998 {
        length += tot.next().unwrap()
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler72());
}


#[test]
fn test72() {
    assert_eq!(euler72(),303963552391)
}