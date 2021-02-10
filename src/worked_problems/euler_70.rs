// Problem: Find the value of n, 1 < n < 10^7, for which φ(n) is a permutation of n and the ratio n/φ(n) produces a minimum.
/*
A number (other than 1) cannot be a permutation of its totient if all its digits are odd since totients are even.
A prime, p, cannot be a permutation of its totient since φ(p) = p-1
*/

use crate::aux_funcs::{int_to_digits, vec_identical, totient_sieve};

pub fn euler70() -> u64 {
    let mut tot = totient_sieve();
    let mut smallest = f64::INFINITY;
    let mut record = 0;
    for n in 2..10_000_000u64 {
        let t = tot.next().unwrap();
        let num: f64 = n as f64;
        let den: f64 = t as f64;
        if num/den < smallest {
            let mut d1 = int_to_digits(n,10);
            let mut d2 = int_to_digits(t,10);
            d1.sort();
            d2.sort();
            if vec_identical(&d1,&d2) {
                smallest = num/den;
                record = n;
            }
        }
    }
    record
}

pub fn euler70_example() {
    println!("\nProblem: Find the value of n, 1 < n < 10^7, for which φ(n) is a permutation of n and the ratio n/φ(n) produces a minimum.");
    println!("\n\nThis is possible by brute force using the totient sieve from Problem 69 but I can't think of a good way to narrow it down to be more efficient. For instance primes never qualify but primes are fairly sparse. Similarly numbers with all odd digits never qualify but these are even less common.");
    let s = "
use crate::aux_funcs::{int_to_digits, vec_identical, totient_sieve};

pub fn euler70() -> u64 {
    let mut tot = totient_sieve();
    let mut smallest = f64::INFINITY;
    let mut record = 0;
    for n in 2..10_000_000u64 {
        let t = tot.next().unwrap();
        let num: f64 = n as f64;
        let den: f64 = t as f64;
        if num/den < smallest {
            let mut d1 = int_to_digits(n,10);
            let mut d2 = int_to_digits(t,10);
            d1.sort();
            d2.sort();
            if vec_identical(&d1,&d2) {
                smallest = num/den;
                record = n;
            }
        }
    }
    record
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler70());
}


#[test]
fn test70() {
    assert_eq!(euler70(),8319823)
}