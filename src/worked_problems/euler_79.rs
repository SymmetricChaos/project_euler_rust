// Problem: Analyse the provided file so as to determine the shortest possible secret passcode of unknown length.

/*
Only 4 and 5 never appear in the file
Thus the passcode must have at least 8 digits
7 only appears in the first position and 0 only appears in the last position

7------0

1 <- 6
1 <- 9
3 <- 1
6 <- 8
1 <- 8
6 <- 9
1 <- 2
2 <- 9
6 <- 2
8 <- 9
3 <- 6
2 <- 8

7 <- 3 <- 1 <- 6 <- 2 <- 8 <- 9 <- 0
*/

use std::fs;
use std::collections::HashSet;
use crate::aux_funcs::{digits_to_int};

fn read_data() -> Vec<Vec<u8>> {
    let s = fs::read_to_string("Euler79Doc.txt").unwrap();
    let rows = s.split("\r\n");
    let mut vec = Vec::new();
    for r in rows {
        let elems = r.chars();
        let v = elems.map(|x| x.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
        vec.push(v);
    }
    vec
}


pub fn euler79() -> u64 {
    let rows = read_data();
    let mut pairs = HashSet::new();
    for r in rows {
        pairs.insert([r[0],r[1]]);
        pairs.insert([r[1],r[2]]);
    }
    let mut chain = vec![];
    let digits = vec![0,1,2,3,6,7,8,9];
    'chain_loop: while chain.len() < 8 {
        'digit_loop: for d in digits.iter() {
            if chain.contains(d) {
                continue
            }
            for p in pairs.iter() {
                if p.contains(d) && p[0] != *d && !chain.contains(&p[0]) {
                    continue 'digit_loop
                }
            }
            chain.push(*d);
            continue 'chain_loop
        }
    }
    digits_to_int(&chain,10)
}

pub fn euler79_example() {
    println!("\nProblem: Analyse the provided file so as to determine the shortest possible secret passcode of unknown length.");
    println!("\n\nThis problem is exceptionally fair and I ended up solving it by hand then tried to translate my method into code. We get all the pairs that appear then we start building a chain of digits. A new digit is accepted to the chain if and only if it is never seen to be preceeded by a digit not yet in the chain. This only workswhen every digit is unique.");
    let s = "
use std::fs;
use std::collections::HashSet;
use crate::aux_funcs::{digits_to_int};

fn read_data() -> Vec<Vec<u8>> {
    let s = fs::read_to_string(\"Euler79Doc.txt\").unwrap();
    let rows = s.split(\"\r\n\");
    let mut vec = Vec::new();
    for r in rows {
        let elems = r.chars();
        let v = elems.map(|x| x.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
        vec.push(v);
    }
    vec
}

pub fn euler79() -> u64 {
    let rows = read_data();
    let mut pairs = HashSet::new();
    for r in rows {
        pairs.insert([r[0],r[1]]);
        pairs.insert([r[1],r[2]]);
    }
    let mut chain = vec![];
    let digits = vec![0,1,2,3,6,7,8,9];
    'chain_loop: while chain.len() < 8 {
        'digit_loop: for d in digits.iter() {
            if chain.contains(d) {
                continue
            }
            for p in pairs.iter() {
                // IF d is in the pair AND the first element not d AND the first element is not yet used
                // THEN try the next digit
                if p.contains(d) && p[0] != *d && !chain.contains(&p[0]) {
                    continue 'digit_loop
                }
            }
            chain.push(*d);
            continue 'chain_loop
        }
    }
    digits_to_int(&chain,10)
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler79());
}


#[test]
fn test79() {
    assert_eq!(euler79(),73162890)
}
