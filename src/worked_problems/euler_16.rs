// What is the sum of the digits of the number 2^1000?

use num::bigint::BigInt;

pub fn euler16() -> u64 {
    let mut n = BigInt::from(2);
    for _ in 1..1000 {
        n *= 2;
    }
    let s = n.to_str_radix(10);
    let mut out = 0u64;
    for ch in s.chars() {
        out += ch.to_digit(10).unwrap() as u64;
    }
    out
}

pub fn euler16_example() -> u64 {
    println!("\nProblem: What is the sum of the digits of the number 2^1000?");
    println!("\n\nThe BigInt library ones again makes this fairly easy.");
    let s = "
use num::bigint::BigInt;

pub fn euler16() -> u64 {
    let mut n = BigInt::from(2);
    for _ in 1..1000 {
        n *= 2;
    }
    let s = n.to_str_radix(10);
    let mut out = 0u64;
    for ch in s.chars() {
        out += ch.to_digit(10).unwrap() as u64;
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler16());
    0u64
}

#[test]
fn test16() {
    assert_eq!(euler16(),1366)
}