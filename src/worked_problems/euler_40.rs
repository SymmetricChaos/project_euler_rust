/*
If dn represents the nth digit of the fractional part of Campernowne's constant, find the value of the following expression.

d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000
*/

use crate::aux_funcs::{int_to_digits};

pub fn euler40() -> u64 {
    let mut out = 1u64;
    let mut index = 10;
    let mut n = 2;
    let mut pos = 1;
    loop {
        let s = n.to_string();
        pos += s.len();
        if pos >= index {
            // If we step past the index we need get the digits and go backward
            let mut digits = int_to_digits(n,10);
            digits.reverse();
            out *= digits[pos-index] as u64;
            index *= 10;
            if index > 1000000 {
                break
            }
        }
        n += 1
    }
    out
}

pub fn euler40_example() {
    println!("\nIf d_n represents the nth digit of the fractional part of Campernowne's constant, find the value of the following expression: d_1 × d_10 × d_100 × d_1000 × d_10000 × d_100000 × d_1000000");
    println!("\n\nUsing the length of the integers as a step size we can make our way through Campernowne's constant and step back whenever we step past one of the indicies we're looking for.");
    let s = "
use crate::aux_funcs::{int_to_digits};

pub fn euler40() -> u64 {
    let mut out = 1;
    let mut index = 10;
    let mut n = 2;
    let mut pos = 1;
    loop {
        let s = n.to_string();
        pos += s.len();
        if pos >= index {
            // If we step past the index we need get the digits and go backward
            let mut digits = int_to_digits(n,10);
            digits.reverse();
            out *= digits[pos-index];
            index *= 10;
            if index > 1000000 {
                break
            }
        }
        n += 1
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler40());
}

#[test]
fn test40() {
    assert_eq!(euler40(),210)
}