// Problem: What is the side length of the square spiral for which the ratio of primes along both diagonals first falls below 10%?

/*

*/

use crate::aux_funcs::{is_prime};

pub fn euler58() -> u64 {

    let mut step = 0;
    let mut pos = 1;

    let mut prime_ctr = 0;
    let mut total_ctr = 1;

    loop {
        step += 2;
        total_ctr += 4;
        for _ in 0..4 {
            pos += step;
            if is_prime(pos) {
                prime_ctr += 1
            }
        }
        if prime_ctr*10 < total_ctr {
            break
        }
    }
    step+1
}

pub fn euler58_example() {
    println!("\nProblem: What is the side length of the square spiral for which the ratio of primes along both diagonals first falls below 10%?");
    println!("\n\nAs in problem 28 there's no need to build the whole spiral in memory when the side lengths are so predictable.");
    let s = "
use crate::aux_funcs::{is_prime};

pub fn euler58() -> u64 {

    let mut step = 0;
    let mut pos = 1;

    let mut prime_ctr = 0;
    let mut total_ctr = 1;

    loop {
        step += 2;
        total_ctr += 4;
        for _ in 0..4 {
            pos += step;
            if is_prime(pos) {
                prime_ctr += 1
            }
        }
        if prime_ctr*10 < total_ctr {
            break
        }
    }
    step+1
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler58());
}

#[test]
fn test58() {
    assert_eq!(euler58(),26241)
}