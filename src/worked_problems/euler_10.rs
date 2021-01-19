// Find the sum of all the primes below two million.

use crate::aux_funcs::{prime_sieve};

pub fn euler10() -> u64 {
    let pset = prime_sieve();
    let mut out = 0;
    for p in pset {
        if p > 2_000_000 {
            break;
        }
        out += p;
    }
    out
}

pub fn euler10_example() -> u64 {
    println!("\nProblem: Find the sum of all the primes below two million.");
    println!("\n\nThe same sieving method as used for Problem 7 allows us to generate primes.");
    let s = "
pub fn euler10() -> u64 {
    let pset = prime_sieve();
    let mut out = 0;
    for p in pset {
        if p > 2_000_000 {
            break;
        }
        out += p;
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler10());
    0u64
}

#[test]
fn test10() {
    assert_eq!(euler10(),142913828922)
}