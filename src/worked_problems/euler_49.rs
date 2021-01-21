// The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330, is unusual in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit numbers are permutations of one another. What is the number produced by concatenating the terms of the one other 4-digit increasing sequence like this?

/*
Facts we can deduce: 
At least two digits must change at each step (otherwise it doesn't produce a permutation)
The step size must be even
The step size must be less than 4500 since 1000 + 4500 + 4500 = 10000
Thus there are less 2250 possible step sizes to check for each eligible prime

*/
use crate::aux_funcs::{prime_sieve,int_to_digits};

fn is_permutation(a: u64, b: u64) -> bool {
    let mut x = int_to_digits(a,10);
    let mut y = int_to_digits(b,10);
    x.sort();
    y.sort();
    format!("{:?}",x) == format!("{:?}",y)
}
 
fn prime_permutation_arithmetic_sequences(n: u64, primes: &Vec<u64>) -> u64 {
    let mut e = 0;
    let mut out: u64 = 0;
    loop {
        e += 2;
        if 10_000 - n - e < e {
            break
        }
        if primes.contains(&(n+e)) && primes.contains(&(n+e+e)) {
            if is_permutation(n,n+e) && is_permutation(n,n+e+e) {
                let s = &format!("{}{}{}",n, n+e, n+e+e);
                out = s.parse().unwrap();
                break 
            }
        }
    }
    out.clone()
}

pub fn euler49() -> u64 {
    let mut pset = prime_sieve();
    let mut primes = Vec::new();
    loop {
        let cur = pset.next().unwrap();
        if cur > 10000 {
            break
        }
        if cur > 999 {
            primes.push(cur);
        }
    }
    let mut out: u64 = 0;
    for p in &primes {
        if *p != 1487 {
            let s = prime_permutation_arithmetic_sequences(*p,&primes);
            if s != 0 {
                out = s;
                break
            }
        }
    };
    out
}

pub fn euler49_example() {
    println!("\nProblem: The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330, is unusual in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit numbers are permutations of one another. What is the number produced by concatenating the terms of the one other 4-digit increasing sequence like this?");
    println!("\n\nThere is probably a way to narrow this down but I couldn't come up with one that made a difference. Instead simply observe that there are less than 2500 possibly arithmetic progressions to check as the step must be even and cannot be greater than 4500. We also don't have to do any primality testing as we are interested in only 4-digit primes and will need a list anyway so primality checking can be done by lookup.");
    let s = "
use crate::aux_funcs::{prime_sieve,int_to_digits};

fn is_permutation(a: u64, b: u64) -> bool {
    let mut x = int_to_digits(a,10);
    let mut y = int_to_digits(b,10);
    x.sort();
    y.sort();
    format!(\"{:?}\",x) == format!(\"{:?}\",y)
}
    
fn prime_permutation_arithmetic_sequences(n: u64, primes: &Vec<u64>) -> u64 {
    let mut e = 0;
    let mut out: u64 = 0;
    loop {
        e += 2;
        if 10_000 - n - e < e {
            break
        }
        if primes.contains(&(n+e)) && primes.contains(&(n+e+e)) {
            if is_permutation(n,n+e) && is_permutation(n,n+e+e) {
                let s = &format!(\"{}{}{}\",n, n+e, n+e+e);
                out = s.parse().unwrap();
                break 
            }
        }
    }
    out.clone()
}

pub fn euler49() -> u64 {
    let mut pset = prime_sieve();
    let mut primes = Vec::new();
    loop {
        let cur = pset.next().unwrap();
        if cur > 10000 {
            break
        }
        if cur > 999 {
            primes.push(cur);
        }
    }
    let mut out: u64 = 0;
    for p in &primes {
        if *p != 1487 {
            let s = prime_permutation_arithmetic_sequences(*p,&primes);
            if s != 0 {
                out = s;
                break
            }
        }
    };
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler49());
}

#[test]
fn test48() {
    assert_eq!(euler49(),296962999629)
}