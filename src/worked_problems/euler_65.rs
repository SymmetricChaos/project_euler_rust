// Problem: Find the sum of digits in the numerator of the 100th convergent of the continued fraction for e.
/*

*/

use num::integer::gcd;
use num::traits::{Zero,One};
use num::bigint::BigUint;
use crate::aux_funcs::{int_to_digits};

fn sum_of_u8(digits: &Vec<u8>) -> u64 {
    let mut sum = 0u64;
    for d in digits.iter() {
        sum += *d as u64
    }
    sum
}

fn convergent(coef: &Vec<BigUint>) -> (BigUint,BigUint) {
    let (mut n0, mut d0) = (BigUint::zero(),BigUint::one());
    let (mut n1, mut d1) = (BigUint::one(),BigUint::zero());

    for c in coef.iter() {
        let nt = n1.clone();
        n1 = c * n1 + n0;
        n0 = nt;
        let dt = d1.clone();
        d1 = c * d1 + d0;
        d0 = dt;
        let g = gcd(n1.clone(),d1.clone());
        n1 = &n1/&g;
        d1 = &d1/&g;

    }
    (n1,d1)
}

pub fn euler65() -> u64 {
    let mut coef = vec![BigUint::from(2u8)];
    let mut ctr = 2u8;
    let mut third = 1u8;
    while coef.len() < 100 {
        if ctr == 0 {
            coef.push(BigUint::from(third*2));
            third += 1
        } else {
            coef.push(BigUint::one())
        }
        ctr = (ctr+1) % 3;
    }
    let (n1, _) = convergent(&coef);
    let v = int_to_digits(n1,BigUint::from(10u8));
    sum_of_u8(&v)
}

pub fn euler65_example() {
    println!("\nProblem: Find the sum of digits in the numerator of the 100th convergent of the continued fraction for e.");
    println!("\n\nConvergents can be calculate iteratively without much trouble other than wreslting with the borrow checker when using BigUint.");
    let s = "
use num::integer::gcd;
use num::traits::{Zero,One};
use num::bigint::BigUint;
use crate::aux_funcs::{int_to_digits};

fn sum_of_u8(digits: &Vec<u8>) -> u64 {
    let mut sum = 0u64;
    for d in digits.iter() {
        sum += *d as u64
    }
    sum
}

fn convergent(coef: &Vec<BigUint>) -> (BigUint,BigUint) {
    let (mut n0, mut d0) = (BigUint::zero(),BigUint::one());
    let (mut n1, mut d1) = (BigUint::one(),BigUint::zero());

    for c in coef.iter() {
        let nt = n1.clone();
        n1 = c * n1 + n0;
        n0 = nt;
        let dt = d1.clone();
        d1 = c * d1 + d0;
        d0 = dt;

        let g = gcd(n1.clone(),d1.clone());
        n1 = &n1/&g;
        d1 = &d1/&g;

    }

    (n1,d1)
}

pub fn euler65() -> u64 {
    let mut coef = vec![BigUint::from(2u8)];
    let mut ctr = 2u8;
    let mut third = 1u8;
    while coef.len() < 100 {
        if ctr == 0 {
            coef.push(BigUint::from(third*2));
            third += 1
        } else {
            coef.push(BigUint::one())
        }
        ctr = (ctr+1) % 3;
    }

    let (n1, _) = convergent(&coef);
    let v = int_to_digits(n1,BigUint::from(10u8));
    sum_of_u8(&v)
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler65());
}


#[test]
fn test65() {
    assert_eq!(euler65(),272)
}
