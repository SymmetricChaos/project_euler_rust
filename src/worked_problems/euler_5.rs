// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
// This one is trivial to solve by hand using prime factorization

use crate::aux_funcs::{prime_factorization};
use std::collections::HashMap;
use std::cmp;
use num::traits::pow;

pub fn euler5() -> u64 {
    let mut prime_factors = HashMap::<u64,u64>::new();
    for n in 1..21 {
        let c = prime_factorization(n);
        for (p,e) in c {
            if prime_factors.contains_key(&p) {
                prime_factors.insert(p,cmp::max(e,prime_factors[&p]));
            } else {
                prime_factors.insert(p,e);
            }
        }
    }
    let mut out: u64 = 1;
    for (p,e) in prime_factors.iter() {
        out *= pow(*p,*e as usize)
    }
    out
}

pub fn euler5_example() -> u64 {
    println!("\nWhat is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?");
    println!("\nThis can easily be solved by hand by inspection of the prime factors of the integers in question. Using this method we get 2*3*2*5*7*2*3*11*13*2*17*19 = 232792560");
    println!("\nHowever it is worth considered how to solve a question like this in general. Here we use a simple axiliary function that produces a HashMap with keys primes and values the exponent they are to be raised to.");
    let s = "
use crate::aux_funcs::{prime_factorization};
use std::collections::HashMap;
use std::cmp;
use num::traits::pow;

pub fn euler5() -> u64 {
    let mut prime_factors = HashMap::<u64,u64>::new();
    for n in 1..21 {
        let c = prime_factorization(n);
        for (p,e) in c {
            if prime_factors.contains_key(&p) {
                prime_factors.insert(p,cmp::max(e,prime_factors[&p]));
            } else {
                prime_factors.insert(p,e);
            }
        }
    }
    let mut out: u64 = 1;
    for (p,e) in prime_factors.iter() {
        out *= pow(*p,*e as usize)
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler5());
    0u64
}
