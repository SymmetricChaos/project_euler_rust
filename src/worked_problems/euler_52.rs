// Problem: Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits.

/*
We have to search relatively few numbers. 10/6 = 1.666...
So we only need to search 10-16, 100-166, 1000-1666 and so on
*/

use crate::aux_funcs::{int_to_digits,vec_identical};

pub fn euler52() -> u64 {
    let mut pow_ten = 10;
    let mut sixes = 6;
    let out = 'outer: loop {
        'naturals: for i in pow_ten..pow_ten+sixes {
            let mut v = int_to_digits(i,10);
            v.sort();
            for m in 2..7 {
                let mut c = int_to_digits(i*m,10);
                c.sort();
                if !vec_identical(&v,&c) {
                    continue 'naturals
                }
            }
            break 'outer i
        }
        pow_ten *= 10;
        sixes = sixes*10 + 6;
    };
    out
}

pub fn euler52_example() {
    println!("\nProblem: Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits.");
    println!("\n\nWe have to search relatively few numbers. 10/6 = 1.666... So we only need to search 10-16, 100-166, 1000-1666 and so on.");
    let s = "
use crate::aux_funcs::{int_to_digits,vec_identical};

pub fn euler52() -> u64 {
    let mut pow_ten = 10;
    let mut sixes = 6;
    let out = 'outer: loop {
        'naturals: for i in pow_ten..pow_ten+sixes {
            let mut v = int_to_digits(i,10);
            v.sort();
            for m in 2..7 {
                let mut c = int_to_digits(i*m,10);
                c.sort();
                if !vec_identical(&v,&c) {
                    continue 'naturals
                }
            }
            break 'outer i
        }
        pow_ten *= 10;
        sixes = sixes*10 + 6;
    };
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler52());
}

#[test]
fn test52() {
    assert_eq!(euler52(),142857)
}