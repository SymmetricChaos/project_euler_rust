// Find the sum of all numbers which are equal to the sum of the factorial of their digits.

/*
We need to establish an upper limit.

If a qualifying number contains a 9 it must also be exactly 7 digits and less than 2540160,
since that is 9! * 7. Any larger number will have to be larger than its factorial digit sum.
*/

use crate::aux_funcs::{int_to_digits};

fn digit_factorial_sum(n: u64) -> u64 {
    let factorials = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    let digits = int_to_digits(n,10);
    let mut out = 0;
    for d in digits {
        out += factorials[d as usize];
    }
    out
}

pub fn euler34() -> u64 {
    let mut out = 0;
    for n in 10..2540160 {
        if digit_factorial_sum(n) == n {
            out += n;
        }
    }
    out
}

pub fn euler34_example() {
    println!("\nProblem: Find the sum of all numbers which are equal to the sum of the factorial of their digits.");
    println!("\n\nAs in some other problems we need to establish our own upper bound. A loose one can be found by observing that the 9 * 7! is 2540160 which is less than 9999999 and the thereafter all numbers are greater than the sum of the factorials of their digits. The int_to_digits function used in some other problems is imported from a file of useful auxilliary functions.");
    let s = "
use crate::aux_funcs::{int_to_digits};

fn digit_factorial_sum(n: u64) -> u64 {
    let factorials = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    let digits = int_to_digits(n,10);
    let mut out = 0;
    for d in digits {
        out += factorials[d as usize];
    }
    out
}

pub fn euler34() -> u64 {
    let mut out = 0;
    for n in 10..2540160 {
        if digit_factorial_sum(n) == n {
            out += n;
        }
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler34());
}

#[test]
fn test34() {
    assert_eq!(euler34(),40730)
}