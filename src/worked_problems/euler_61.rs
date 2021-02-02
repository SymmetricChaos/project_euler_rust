// Problem: Find the sum of the only ordered set of six cyclic 4-digit numbers for which each polygonal type: triangle, square, pentagonal, hexagonal, heptagonal, and octagonal, is represented by a different number in the set.
/*

*/

use crate::aux_funcs::{is_prime,prime_sieve};
use std::collections::HashMap;
use itertools::Itertools;

fn count_digits(n: u16) -> u8 {
    let mut ctr = 1;
    let mut n = n;
    n /= 10;
    while n != 0 {
        ctr += 1;
        n /= 10;
    }
    ctr
}

fn figurate(s: u16, n: u16) -> u16 {
    if s == 3 {
        return n*(n+1) / 2
    }
    ( n*n*(s-2)-n*(s-4) ) / 2
}

fn all_four_digit_figurate(s: u16) -> Vec<u16> {
    let mut ctr = 0;
    let mut out = Vec::<u16>::new();
    loop {
        ctr += 1;
        let f = figurate(s,ctr);
        let c = count_digits(f);
        if c > 4 {
            break
        }
        if c == 4 {
            out.push(f)
        }
    }
    out
}

pub fn euler61() -> u64 {
    let tri = all_four_digit_figurate(3);
    let sqr = all_four_digit_figurate(4);
    let pen = all_four_digit_figurate(5);
    let hex = all_four_digit_figurate(6);
    let sep = all_four_digit_figurate(7);
    let oct = all_four_digit_figurate(8);
    println!("{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",tri,sqr,pen,hex,sep,oct);

    0u64
}

pub fn euler61_example() {
    println!("\nProblem: Find the sum of the only ordered set of six cyclic 4-digit numbers for which each polygonal type: triangle, square, pentagonal, hexagonal, heptagonal, and octagonal, is represented by a different number in the set.");
    println!("\n\n");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler61());
}


#[test]
fn test61() {
    assert_eq!(euler61(),28684)
}
