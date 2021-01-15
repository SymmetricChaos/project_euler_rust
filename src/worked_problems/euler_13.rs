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
    let out = &sum.to_str_radix(10)[1..10];
    return out.parse::<u64>().unwrap();
}
