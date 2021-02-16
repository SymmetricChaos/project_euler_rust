// Problem: For the first one hundred natural numbers, find the total of the digital sums of the first one hundred decimal digits for all the irrational square roots.

/*
Consider 
*/

use num::traits::{Zero,One};
use num::bigint::BigUint;
use crate::aux_funcs::{int_to_digits, sum_digits};
use num::integer::sqrt;

fn next_digit(p: &BigUint, c: &BigUint) -> u8 {
    let mut i = BigUint::zero();
    let twenty = BigUint::from(20u32);
    let mut out = 9;
    for t in 0..10 {
        if i.clone()*(i.clone()+twenty.clone()*p) > *c {
            out = t-1;
            break
        }
        i += BigUint::one();
    }
    out
}

fn sqrt_digit_sum(n: u64) -> u64 {
    let mut digits = int_to_digits(n,10);
    let hundred = BigUint::from(100u32);
    let twenty = BigUint::from(20u32);
    let ten = BigUint::from(10u32);
    let zero = BigUint::zero();
    let mut r = zero.clone();
    let mut p = zero.clone();
    let mut d = BigUint::from(n);

    while digits.len() < 100 {
        let c = hundred.clone()*r+d;
        let x = next_digit(&p,&c);
        digits.push(x);
        let bigx = BigUint::from(x);
        r = c.clone() - bigx.clone()*(bigx.clone()+twenty.clone()*p.clone());
        p = ten.clone()*p+bigx.clone();
        d = zero.clone();
    }
    sum_digits(&digits)
}

pub fn euler80() -> u64 {
    let mut out = 0;
    for n in 0..100 {
        if sqrt(n)*sqrt(n) == n {
            continue
        }
        out += sqrt_digit_sum(n)
    }
    out
}

pub fn euler80_example() {
    println!("\nProblem: For the first one hundred natural numbers, find the total of the digital sums of the first one hundred decimal digits for all the irrational square roots.");
    println!("\n\nUsing floating point square roots will not work in this case. Fortunately there are digit by digit methods for calculating square roots.");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler80());
}


#[test]
fn test80() {
    assert_eq!(euler80(),40886)
}
