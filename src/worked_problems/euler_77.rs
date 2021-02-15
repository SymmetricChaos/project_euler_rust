// Problem: What is the first value which can be written as the sum of primes in over five thousand different ways?
/*
*/

use crate::aux_funcs::{prime_sieve};

fn restricted_partition(coins: &Vec<u64>, pos: usize, cur: u64) -> u64 {
    let mut total = 0;
    for (n,c) in coins.iter().enumerate() {
        if n < pos {
            // Skip any coins smaller than the one currently being used
            continue;
        }
        if *c > cur {
            // Do nothing if the coin type is too big
        } else if *c == cur {
            // If we reach exactly zero add one to the total
            total += 1;
        } else {
            // Otherwise continue to recur
            total += restricted_partition(coins,n,cur-c);
        }
    }
    total
}

pub fn euler77() -> u64 {
    let mut sieve = prime_sieve();
    let mut primes = vec!{sieve.next().unwrap()};
    let mut ctr = 10;
    loop {
        ctr += 1;
        while *primes.last().unwrap() < ctr {
            primes.push(sieve.next().unwrap())
        }
        if restricted_partition(&primes,0,ctr) > 5000 {
            break
        }
    }
    ctr
}


pub fn euler77_example() {
    println!("\nProblem: What is the first value which can be written as the sum of primes in over five thousand different ways?");
    println!("\n\nEssentially the same method that worked for Problem 31 also works here. We need only change the array to a vector and grow it as necessary for each new value.");
    let s = "
use crate::aux_funcs::{prime_sieve};

fn restricted_partition(coins: &Vec<u64>, pos: usize, cur: u64) -> u64 {
    let mut total = 0;
    for (n,c) in coins.iter().enumerate() {
        if n < pos {
            // Skip any coins smaller than the one currently being used
            continue;
        }
        if *c > cur {
            // Do nothing if the coin type is too big
        } else if *c == cur {
            // If we reach exactly zero add one to the total
            total += 1;
        } else {
            // Otherwise continue to recur
            total += restricted_partition(coins,n,cur-c);
        }
    }
    total
}

pub fn euler77() -> u64 {
    let mut sieve = prime_sieve();
    let mut primes = vec!{sieve.next().unwrap()};
    let mut ctr = 10;
    loop {
        ctr += 1;
        while *primes.last().unwrap() < ctr {
            primes.push(sieve.next().unwrap())
        }
        if restricted_partition(&primes,0,ctr) > 5000 {
            break
        }
    }
    ctr
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler77());
}


#[test]
fn test77() {
    assert_eq!(euler77(),71)
}