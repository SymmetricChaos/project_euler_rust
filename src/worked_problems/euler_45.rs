// What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?

use crate::aux_funcs::{is_prime};

fn prime_diff(n: u64, v: &Vec<u64>) -> bool {
    for s in v {
        if s < &n {
            if is_prime(n-s) {
                return true
            }
        }
    }
    return false
}

pub fn euler45() -> u64 {
    let mut odd = 1;
    let mut n = 1;
    let mut two_square = vec![2];

    let out = loop {
        odd += 2;
        if is_prime(odd) {
            continue
        }
        if odd > *two_square.last().unwrap() {
            two_square.push(2*n*n);
            n += 1;
        }
        if !prime_diff(odd,&two_square) {
            break odd;
        }
    };
    out
}

pub fn euler45_example() {
    println!("\nProblem: What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?");
    println!("\n\nIn this case checking for primality is less work than checking is a number is twice a square. So we'll generate twice squares and check if the difference is prime.");
    let s = "
use crate::aux_funcs::{is_prime};

fn prime_diff(n: u64, v: &Vec<u64>) -> bool {
    for s in v {
        if s < &n {
            if is_prime(n-s) {
                return true
            }
        }
    }
    return false
}

pub fn euler45() -> u64 {
    let mut odd = 1;
    let mut n = 1;
    let mut two_square = vec![2];

    let out = loop {
        odd += 2;
        if is_prime(odd) {
            continue
        }
        if odd > *two_square.last().unwrap() {
            two_square.push(2*n*n);
            n += 1;
        }
        if !prime_diff(odd,&two_square) {
            break odd;
        }
    };
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler45());
}

#[test]
fn test45() {
    assert_eq!(euler45(),5777)
}