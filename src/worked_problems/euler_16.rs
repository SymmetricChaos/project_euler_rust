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