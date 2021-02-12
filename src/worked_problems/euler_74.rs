// Problem: How many chains of numbers to the sum of the factorials of their digits, with a starting number below one million, contain exactly sixty non-repeating terms?

/*
*/

use std::convert::TryFrom;
use std::collections::HashMap;
use crate::aux_funcs::{int_to_digits};


// An actual factorial() function is needlessly inefficient here, instead we use a static array to map each digit to its factorial
static FACTORIAL: [u64; 10] = [1,1,2,6,24,120,720,5040,40320,362880];

fn step(n: u64) -> u64 {
    let digits = int_to_digits(n,10);
    let mut out = 0u64;
    for d in digits {
        out += FACTORIAL[usize::try_from(d).unwrap()]
    }
    out
}

fn chain(n: u64, length_map: &HashMap<u64,u8>) -> u8 {
    let mut terms = vec![n];
    let mut t = n;
    loop {
        t = step(t);
        if length_map.contains_key(&t) {
            return terms.len() as u8 + length_map[&t]
        }
        if terms.contains(&t) {
            break
        }
        terms.push(t)
    }
    terms.len() as u8
}

pub fn euler74() -> u64 {
    let mut lengths = HashMap::<u64,u8>::new();
    let mut out = 0;
    for n in 1..1_000_000 {
        let l = chain(n,&lengths);
        if l == 60 {
            out += 1
        }
        lengths.insert(n,l);
    }
    out
}


pub fn euler74_example() {
    println!("\nProblem: How many chains of numbers to the sum of the factorials of their digits, with a starting number below one million, contain exactly sixty non-repeating terms?");
    println!("\n\nThe same memoization technique that can be used for any chain like this applies. For speed we use a static array rather than calculating factorials repeatedly.");
    let s = "
use std::convert::TryFrom;
use std::collections::HashMap;
use crate::aux_funcs::{int_to_digits};

static FACTORIAL: [u64; 10] = [1,1,2,6,24,120,720,5040,40320,362880];

fn step(n: u64) -> u64 {
    let digits = int_to_digits(n,10);
    let mut out = 0u64;
    for d in digits {
        out += FACTORIAL[usize::try_from(d).unwrap()]
    }
    out
}

fn chain(n: u64, length_map: &HashMap<u64,u8>) -> u8 {
    let mut terms = vec![n];
    let mut t = n;
    loop {
        t = step(t);
        if length_map.contains_key(&t) {
            return terms.len() as u8 + length_map[&t]
        }
        if terms.contains(&t) {
            break
        }
        terms.push(t)
    }
    terms.len() as u8
}

pub fn euler74() -> u64 {
    let mut lengths = HashMap::<u64,u8>::new();
    let mut out = 0;
    for n in 1..1_000_000 {
        let l = chain(n,&lengths);
        if l == 60 {
            out += 1
        }
        lengths.insert(n,l);
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler74());
}


#[test]
fn test74() {
    assert_eq!(euler74(),402)
}