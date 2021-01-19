// Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.

use std::fs;
use num::bigint::BigInt;

pub fn euler13() -> u64 {
    let s = fs::read_to_string("Euler13Doc.txt").unwrap();
    let nums = s.split("\r\n");
    let mut sum = BigInt::from(0);
    for n in nums {
        sum += BigInt::parse_bytes(n.as_bytes(),10).unwrap();
    }
    let out = &sum.to_str_radix(10)[0..10];
    out.parse::<u64>().unwrap()
}

pub fn euler13_example() {
    println!("\nProblem: Work out the first ten digits of the sum of the provided one-hundred 50-digit numbers.");
    println!("\n\nThere's probably a clever way to do this but I chose to use a BigInt library to solve it directly.");
    let s = "
use std::fs;
use num::bigint::BigInt;

pub fn euler13() -> u64 {
    let s = fs::read_to_string(\"Euler13Doc.txt\").unwrap();
    let nums = s.split(\"\r\n\");
    let mut sum = BigInt::from(0);
    for n in nums {
        sum += BigInt::parse_bytes(n.as_bytes(),10).unwrap();
    }
    let out = &sum.to_str_radix(10)[0..10];
    out.parse::<u64>().unwrap()
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler13());
}

#[test]
fn test13() {
    assert_eq!(euler13(),5537376230)
}