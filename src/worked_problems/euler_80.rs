// Problem: For the first one hundred natural numbers, find the total of the digital sums of the first one hundred decimal digits for all the irrational square roots.

/*

*/

use num::traits::{Zero,One};
use num::bigint::BigUint;
use crate::aux_funcs::{sum_digits};

fn next_digit(p: &BigUint, c: &BigUint) -> u8 {
    let mut i = BigUint::zero();
    let twenty = BigUint::from(20u32);
    let mut out = 9;
    for t in 0..10 {
        if &i*(&i+&twenty*p) > *c {
            out = t-1;
            break
        }
        i += BigUint::one();
    }
    out
}

// Not a general solution, fails for n > 100
fn sqrt_digit_sum(n: u64) -> u64 {
    let mut digits = vec![];
    let hundred = BigUint::from(100u32);
    let twenty = BigUint::from(20u32);
    let ten = BigUint::from(10u32);
    let zero = BigUint::zero();
    let mut r = zero.clone();
    let mut p = zero.clone();
    let mut d = BigUint::from(n);

    while digits.len() < 100 {
        let c = &hundred*r+d;
        let x = next_digit(&p,&c);
        digits.push(x);
        let bigx = &BigUint::from(x);
        r = c - bigx*(bigx+&twenty*&p);
        p = &ten*&p+bigx;
        d = BigUint::zero();
    }
    sum_digits(&digits)
}

pub fn euler80() -> u64 {
    let mut out = 0;
    let squares = [4,9,16,25,36,49,64,81];
    for n in 2..100 {
        if squares.contains(&n) {
            continue
        } else {
            out += sqrt_digit_sum(n)
        }
    }
    out
}

pub fn euler80_example() {
    println!("\nProblem: For the first one hundred natural numbers, find the total of the digital sums of the first one hundred decimal digits for all the irrational square roots.");
    println!("\n\nUsing floating point square roots will not work in this case. Fortunately there are digit by digit methods for calculating square roots. We have to use BigUint to deal with the very large values generated.");
    let s = "
use num::traits::{Zero,One};
use num::bigint::BigUint;
use crate::aux_funcs::{sum_digits};

fn next_digit(p: &BigUint, c: &BigUint) -> u8 {
    let mut i = BigUint::zero();
    let twenty = BigUint::from(20u32);
    let mut out = 9;
    for t in 0..10 {
        if &i*(&i+&twenty*p) > *c {
            out = t-1;
            break
        }
        i += BigUint::one();
    }
    out
}

// Not a general solution, fails for n > 100
fn sqrt_digit_sum(n: u64) -> u64 {
    let mut digits = vec![];
    let hundred = BigUint::from(100u32);
    let twenty = BigUint::from(20u32);
    let ten = BigUint::from(10u32);
    let zero = BigUint::zero();
    let mut r = zero.clone();
    let mut p = zero.clone();
    let mut d = BigUint::from(n);

    while digits.len() < 100 {
        let c = &hundred*r+d;
        let x = next_digit(&p,&c);
        digits.push(x);
        let bigx = &BigUint::from(x);
        r = c - bigx*(bigx+&twenty*&p);
        p = &ten*&p+bigx;
        d = BigUint::zero();
    }
    sum_digits(&digits)
}

pub fn euler80() -> u64 {
    let mut out = 0;
    let squares = [4,9,16,25,36,49,64,81];
    for n in 2..100 {
        if squares.contains(&n) {
            continue
        } else {
            out += sqrt_digit_sum(n)
        }
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler80());
}


#[test]
fn test80() {
    assert_eq!(euler80(),40886)
}
