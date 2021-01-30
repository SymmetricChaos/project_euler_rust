// Problem: In the first one-thousand expansions of the continued fraction of the square root of two, how many fractions contain a numerator with more digits than the denominator?

/*

*/

use num::bigint::BigInt;
use num::rational::BigRational;
use num::traits::Zero;

fn count_digits(n: BigInt) -> u64 {
    let mut ctr = 1;
    let mut n = n;
    n /= 10;
    while n != BigInt::zero() {
        ctr += 1;
        n /= 10;
    }
    ctr
}

fn sqrt_frac_step(pair: BigRational) -> BigRational {
    BigRational::new(pair.numer() + pair.denom() + pair.denom(), pair.numer() + pair.denom())
}

pub fn euler57() -> u64 {
    let mut ctr = 0;
    let mut ratio = BigRational::new(BigInt::from(3),BigInt::from(2));
    for _ in 0..1000 {
        ratio = sqrt_frac_step(ratio).reduced();
        if count_digits(ratio.numer().clone()) > count_digits(ratio.denom().clone()) {
            ctr += 1;
        }
    }
    ctr
}

pub fn euler57_example() {
    println!("\nProblem: In the first one-thousand expansions of the continued fraction of the square root of two, how many fractions contain a numerator with more digits than the denominator?");
    println!("\n\nAgain a big integer library is useful. Making a digit by digit calculation of the greatest common denominator is possible, of course, but I had enough trouble making errors with addition and multiplication.");
    let s = "
use num::bigint::BigInt;
use num::rational::BigRational;
use num::traits::Zero;

fn count_digits(n: BigInt) -> u64 {
    let mut ctr = 1;
    let mut n = n;
    n /= 10;
    while n != BigInt::zero() {
        ctr += 1;
        n /= 10;
    }
    ctr
}

fn sqrt_frac_step(pair: BigRational) -> BigRational {
    BigRational::new(pair.numer() + pair.denom() + pair.denom(), pair.numer() + pair.denom())
}

pub fn euler57() -> u64 {
    let mut ctr = 0;
    let mut ratio = BigRational::new(BigInt::from(3),BigInt::from(2));
    for _ in 0..1000 {
        ratio = sqrt_frac_step(ratio).reduced();
        if count_digits(ratio.numer().clone()) > count_digits(ratio.denom().clone()) {
            ctr += 1;
        }
    }
    ctr
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler57());
}

#[test]
fn test57() {
    assert_eq!(euler57(),153)
}