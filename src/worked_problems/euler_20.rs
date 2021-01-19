// Find the sum of the digits in the number 100 factorial

use num::bigint::BigInt;

fn big_factorial(n: u32) -> BigInt {
    let mut out = BigInt::from(1);
    for p in 2..n+1 {
        out *= p;
    }
    out
}

pub fn euler20() -> u64 {
    let s = big_factorial(100).to_str_radix(10);
    let mut out = 0u64;
    for ch in s.chars() {
        out += ch.to_digit(10).unwrap() as u64;
    }
    out
}

pub fn euler20_example() {
    println!("\nProblem: Find the sum of the digits in the number 100 factorial.");
    println!("\n\nOnce again the big integer library makes this straightforward.");
    let s = "
use num::bigint::BigInt;

fn big_factorial(n: u32) -> BigInt {
    let mut out = BigInt::from(1);
    for p in 2..n+1 {
        out *= p;
    }
    out
}

pub fn euler20() -> u64 {
    let s = big_factorial(100).to_str_radix(10);
    let mut out = 0u64;
    for ch in s.chars() {
        out += ch.to_digit(10).unwrap() as u64;
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler20());
}

#[test]
fn test20() {
    assert_eq!(euler20(),648)
}