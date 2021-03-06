// Find the sum of all 0 to 9 pandigital numbers with this property.

use itertools::Itertools;
use crate::aux_funcs::{digits_to_int};

pub fn euler43() -> u64 {
    let mut out = 0u64;

    // All the 0 to 9 pandigital numbers
    let perms = (0..10u8).permutations(10).into_iter();

    for p in perms {
        if p[0] == 0 {
            continue
        }
        if !(digits_to_int(&p[1..4].to_vec(),10) % 2 == 0) {
            continue
        }
        if !(digits_to_int(&p[2..5].to_vec(),10) % 3 == 0) {
            continue
        }
        if !(digits_to_int(&p[3..6].to_vec(),10) % 5 == 0) {
            continue
        }
        if !(digits_to_int(&p[4..7].to_vec(),10) % 7 == 0) {
            continue
        }
        if !(digits_to_int(&p[5..8].to_vec(),10) % 11 == 0) {
            continue
        }
        if !(digits_to_int(&p[6..9].to_vec(),10) % 13 == 0) {
            continue
        }
        if !(digits_to_int(&p[7..10].to_vec(),10) % 17 == 0) {
            continue
        }
        out += digits_to_int(&p,10)
    }
    out
}

pub fn euler43_example() {
    println!("\nProblem: Find the sum of all 0 to 9 pandigital numbers with this property.");
    println!("\n\nOnce again permutations help as a way to generate pandigital numbers. From there they are simple to check.");
    let s = "
use itertools::Itertools;
use crate::aux_funcs::{digits_to_int};

pub fn euler43() -> u64 {
    let mut out = 0;

    // All the 0 to 9 pandigital numbers
    let perms = (0..10u64).permutations(10).into_iter();

    for p in perms {
        if p[0] == 0 {
            continue
        }
        if !(digits_to_int(&p[1..4].to_vec(),10) % 2 == 0) {
            continue
        }
        if !(digits_to_int(&p[2..5].to_vec(),10) % 3 == 0) {
            continue
        }
        if !(digits_to_int(&p[3..6].to_vec(),10) % 5 == 0) {
            continue
        }
        if !(digits_to_int(&p[4..7].to_vec(),10) % 7 == 0) {
            continue
        }
        if !(digits_to_int(&p[5..8].to_vec(),10) % 11 == 0) {
            continue
        }
        if !(digits_to_int(&p[6..9].to_vec(),10) % 13 == 0) {
            continue
        }
        if !(digits_to_int(&p[7..10].to_vec(),10) % 17 == 0) {
            continue
        }
        out += digits_to_int(&p,10)
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler43());
}

#[test]
fn test43() {
    assert_eq!(euler43(),16695334890)
}