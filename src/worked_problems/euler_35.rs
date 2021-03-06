// How many circular primes are there below one million?

use crate::aux_funcs::{is_prime, int_to_digits, digits_to_int, prime_sieve};

fn cycle_digits(n: u64) -> u64 {
    let mut digits = int_to_digits(n,10);
    let d = digits.pop().unwrap();
    digits.insert(0,d);
    digits_to_int(&digits,10)
}


pub fn euler35() -> u64 {
    let mut p = prime_sieve();
    let mut out = 2; // include 2 and 5 which are otherwise excluded
    let disallowed_digits = [0,2,4,5,6,8];
    'outer: loop {
        let cur = p.next().unwrap();
        if cur > 1_000_000 {
            break;
        }
        let digits = int_to_digits(cur,10);
        for d in digits.iter() {
            if disallowed_digits.contains(d) {
                continue 'outer
            }
        }
        let mut temp = cur;
        for _ in 0..digits.len() {
            temp = cycle_digits(temp);
            if !is_prime(temp) {
                continue 'outer
            }
        }
        out += 1;
    }
    out
}

pub fn euler35_example() {
    println!("\nProblem: How many circular primes are there below one million?");
    println!("\n\nWe import several previously established functions to make this possible. For speed we keep an array of disallowed digits that allow us to discard candidates that will inevitably fail to be circular because they contain an even digit or a 5 digit.");
    let s = "
use crate::aux_funcs::{is_prime, int_to_digits, digits_to_int, prime_sieve};

fn cycle_digits(n: u64) -> u64 {
    let mut digits = int_to_digits(n,10);
    let d = digits.pop().unwrap();
    digits.insert(0,d);
    digits_to_int(&digits,10)
}

pub fn euler35() -> u64 {
    let mut p = prime_sieve();
    let mut out = 2; // include 2 and 5 which are otherwise excluded
    let disallowed_digits = [0,2,4,5,6,8];
    'outer: loop {
        let cur = p.next().unwrap();
        if cur > 1_000_000 {
            break;
        }
        let digits = int_to_digits(cur,10);
        for d in digits.iter() {
            if disallowed_digits.contains(d) {
                continue 'outer
            }
        }
        let mut temp = cur;
        for _ in 0..digits.len() {
            temp = cycle_digits(temp);
            if !is_prime(temp) {
                continue 'outer
            }
        }
        out += 1;
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler35());
}

#[test]
fn test35() {
    assert_eq!(euler35(),55)
}