// Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.

use crate::aux_funcs::{int_to_digits, digits_to_int};

fn is_palindrome(n: u64)-> bool {
    let mut digits = int_to_digits(n,10);
    let mut bits = int_to_digits(n,2);
    digits.reverse();
    bits.reverse();
    if n == digits_to_int(&digits,10) && n == digits_to_int(&bits,2) {
        return true
    }
    false
}

pub fn euler36() -> u64 {
    let mut out = 0;
    for n in 0..1_000_000 {
        if is_palindrome(n) {
            out += n;
        }
    }
    out
}

#[test]
fn test36() {
    assert_eq!(euler36(),872187)
}