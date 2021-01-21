// Which prime, below one-million, can be written as the sum of the most consecutive primes?

/*
The full text of the problem gives us the hint that the sum has at least 21 primes
The chain has an even number of terms if and only if it starts at 2, otherwise the sum
    would be even
We can also be sure that the primes are "small" otherwise the chain could not be long
Specifically the chain cannot start with a number greater than 48,000 since the sum would
    have to be greater than one million
*/

use crate::aux_funcs::{prime_sieve,is_prime,PrimeSieve};

fn chain_from_two(primes: &Vec<u64>) -> (u64,u64) {
    let mut num: u64 = 0;
    let mut length = 0;
    let mut cur = 0;
    let mut ctr = 0;
    loop {
        cur += primes[ctr] + primes[ctr+1];
        ctr += 2;
        if cur >= 1_000_000 {
            break
        }
        if is_prime(cur) {
            num = cur;
            length = ctr;
        }
    }
    (num,length as u64)
}

fn chain_from_nth(primes: &mut Vec<u64>, n: usize, sieve: &mut PrimeSieve) -> (u64,u64) {
    let mut num = 0;
    let mut length = 0;
    let mut cur = primes[n];
    let mut ctr = n+1;
    loop {
        while primes.len() <= ctr+1 {
            primes.push(sieve.next().unwrap());
        }
        cur += primes[ctr] + primes[ctr+1];
        ctr += 2;
        if cur >= 1_000_000 {
            break
        }
        if is_prime(cur) {
            num = cur;
            length = ctr-n;
        }
    }
    (num as u64,length as u64)
}

pub fn euler50() -> u64 {
    let mut pset = prime_sieve();
    let mut primes = Vec::new();
    for _ in 0..600 {
        primes.push(pset.next().unwrap());
    }
    let (mut out, mut length) = chain_from_two(&primes);
    let upper_lim = 1_000_000/length;
    let mut index = 1;
    loop {
        if primes[index] > upper_lim {
            break
        }
        let (sum, len) = chain_from_nth(&mut primes, index, &mut pset);
        if len > length {
            length = len;
            out = sum;
        }
        index += 1;
    }
    out
}

pub fn euler50_example() {
    println!("\nProblem: Which prime, below one-million, can be written as the sum of the most consecutive primes?");
    println!("\n\nThe sum has an even number of terms if and only if it starts with 2. We can use this to establish a limit on how large the starting term can be by finding the sum starting with 2 that has the greatest number of terms and divided 1,000,000 by the number of terms. A theorem of Bach and Shallit that the sum of the first n primes is close to 1/2*n^2*log(n) suggests that the sum starting with two should have around 550 terms. This allows us to skip a lot of precomputation.");
    let s = "
use crate::aux_funcs::{prime_sieve,is_prime,PrimeSieve};

fn chain_from_two(primes: &Vec<u64>) -> (u64,u64) {
    let mut num: u64 = 0;
    let mut length = 0;
    let mut cur = 0;
    let mut ctr = 0;
    loop {
        cur += primes[ctr] + primes[ctr+1];
        ctr += 2;
        if cur >= 1_000_000 {
            break
        }
        if is_prime(cur) {
            num = cur;
            length = ctr;
        }
    }
    (num,length as u64)
}

fn chain_from_nth(primes: &mut Vec<u64>, n: usize, sieve: &mut PrimeSieve) -> (u64,u64) {
    let mut num = 0;
    let mut length = 0;
    let mut cur = primes[n];
    let mut ctr = n+1;
    loop {
        while primes.len() <= ctr+1 {
            primes.push(sieve.next().unwrap());
        }
        cur += primes[ctr] + primes[ctr+1];
        ctr += 2;
        if cur >= 1_000_000 {
            break
        }
        if is_prime(cur) {
            num = cur;
            length = ctr-n;
        }
    }
    (num as u64,length as u64)
}

pub fn euler50() -> u64 {
    let mut pset = prime_sieve();
    let mut primes = Vec::new();
    for _ in 0..600 {
        primes.push(pset.next().unwrap());
    }
    let (mut out, mut length) = chain_from_two(&primes);
    let upper_lim = 1_000_000/length;
    let mut index = 1;
    loop {
        if primes[index] > upper_lim {
            break
        }
        let (sum, len) = chain_from_nth(&mut primes, index, &mut pset);
        if len > length {
            length = len;
            out = sum;
        }
        index += 1;
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler50());
}

#[test]
fn test50() {
    assert_eq!(euler50(),997651)
}