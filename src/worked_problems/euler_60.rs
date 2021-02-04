// Problem: Find the lowest sum for a set of five primes for which any two primes concatenate to produce another prime.
/*
We can exclude 2 and 5, obviously.
Memoization will probably help here as we are only interested in concatenating pairs of primes.

*/

use crate::aux_funcs::{is_prime,prime_sieve};
use std::collections::HashMap;
use itertools::Itertools;

fn digit_mask(n: u64) -> u64 {
    let mut mul = 10;
    let mut n = n;
    n /= 10;
    while n != 0 {
        mul *= 10;
        n /= 10;
    }
    mul
}

fn is_pair(a: u64, b: u64) -> bool {
    let p1 = a+b*digit_mask(a);
    let p2 = b+a*digit_mask(b);
    if is_prime(p1) && is_prime(p2) {
        return true
    }
    false
}


// We will pick numbers that are all in some set so we don't need to check the first number
fn check_set_of_five(b: u64, c: u64, d: u64, e: u64, hmap: &HashMap<u64,Vec<u64>>) -> bool {
    if hmap[&b].contains(&c) && hmap[&b].contains(&d) && hmap[&b].contains(&e) {
        if hmap[&c].contains(&d) && hmap[&c].contains(&e) {
            if hmap[&d].contains(&e) {
                return true
            }
        }
    }
    return false
}

pub fn euler60() -> u64 {
    let mut valid_pairs: HashMap<u64,Vec<u64>> = HashMap::new();

    let mut prime_iter = prime_sieve();
    let mut primes = Vec::<u64>::new();
    let mut ctr = 0;
    loop {
        ctr += 1;
        let p = prime_iter.next().unwrap();
        if p == 2 || p == 5 {
            continue
        }
        valid_pairs.insert(p,Vec::<u64>::new());
        for v in primes.iter() {
            if is_pair(p,*v) {
                valid_pairs.get_mut(v).unwrap().push(p);
                valid_pairs.get_mut(&p).unwrap().push(*v);
            }
        }
        primes.push(p);
        if ctr % 300 == 0 {
            for p in primes.iter() {
                if valid_pairs[p].len() >= 5 {
                    //println!("{}: {:?}",p,valid_pairs[p]);
                    for quad in valid_pairs[p].iter().combinations(4) {
                        if check_set_of_five(*quad[0],*quad[1],*quad[2],*quad[3],&valid_pairs) {
                            //println!("{}: {:?}",p,quad);
                            return p+*quad[0]+*quad[1]+*quad[2]+*quad[3]
                        }
        
                    }
                }
                
            }
        }
    }
}

pub fn euler60_example() {
    println!("\nProblem: Find the lowest sum for a set of five primes for which any two primes concatenate to produce another prime.");
    println!("\n\nThis is the first PE problem I've solved in Rust but not in another language. This brute force method works very slowly. There are graph theoretic solutions to this which are much faster.");
    let s = "
use crate::aux_funcs::{is_prime,prime_sieve};
use std::collections::HashMap;
use itertools::Itertools;

fn digit_mask(n: u64) -> u64 {
    let mut mul = 10;
    let mut n = n;
    n /= 10;
    while n != 0 {
        mul *= 10;
        n /= 10;
    }
    mul
}

fn is_pair(a: u64, b: u64) -> bool {
    let p1 = a+b*digit_mask(a);
    let p2 = b+a*digit_mask(b);
    if is_prime(p1) && is_prime(p2) {
        return true
    }
    false
}


// We will pick numbers that are all in some set so we don't need to check the first number
fn check_set_of_five(b: u64, c: u64, d: u64, e: u64, hmap: &HashMap<u64,Vec<u64>>) -> bool {
    if hmap[&b].contains(&c) && hmap[&b].contains(&d) && hmap[&b].contains(&e) {
        if hmap[&c].contains(&d) && hmap[&c].contains(&e) {
            if hmap[&d].contains(&e) {
                return true
            }
        }
    }
    return false
}

pub fn euler60() -> u64 {
    let mut valid_pairs: HashMap<u64,Vec<u64>> = HashMap::new();

    let mut prime_iter = prime_sieve();
    let mut primes = Vec::<u64>::new();
    let mut ctr = 0;
    loop {
        ctr += 1;
        let p = prime_iter.next().unwrap();
        if p == 2 || p == 5 {
            continue
        }
        valid_pairs.insert(p,Vec::<u64>::new());
        for v in primes.iter() {
            if is_pair(p,*v) {
                valid_pairs.get_mut(v).unwrap().push(p);
                valid_pairs.get_mut(&p).unwrap().push(*v);
            }
        }
        primes.push(p);
        if ctr % 300 == 0 {
            for p in primes.iter() {
                if valid_pairs[p].len() >= 5 {
                    //println!(\"{}: {:?}\",p,valid_pairs[p]);
                    for quad in valid_pairs[p].iter().combinations(4) {
                        if check_set_of_five(*quad[0],*quad[1],*quad[2],*quad[3],&valid_pairs) {
                            //println!(\"{}: {:?}\",p,quad);
                            return p+*quad[0]+*quad[1]+*quad[2]+*quad[3]
                        }
        
                    }
                }
                
            }
        }
    }
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler60());
}


#[test]
fn test60() {
    assert_eq!(euler60(),26033)
}
