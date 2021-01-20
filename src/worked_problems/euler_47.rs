// Find the first four consecutive integers to have four distinct prime factors each. What is the first of these numbers?
use std::collections::HashMap;

pub fn euler47() -> u64 {
    let mut n = 1;
    let mut hmap = HashMap::<u64,Vec<u64>>::new();
    let mut ctr = 0;
    let out = loop {
        n += 1;
        if !hmap.contains_key(&n) {
            hmap.insert(n+n,vec![n]);
            ctr = 0
        } else {
            let factors = &hmap[&n].clone();
            if factors.len() == 4 {
                ctr += 1
            } else {
                ctr = 0
            }
            if ctr == 4 {
                break n-3;
            }
            for factor in factors {
                if hmap.contains_key(&(factor+n)) {
                    hmap.get_mut(&(factor+n)).unwrap().push(*factor);
                } else {
                    hmap.insert(factor+n,vec![*factor]);
                }
            }
            hmap.remove(&n);
        }
    };
    out
}

pub fn euler47_example() {
    println!("\nProblem: Find the first four consecutive integers to have four distinct prime factors each. What is the first of these numbers?");
    println!("\n\nThe method we have been using to sieve primes also builds up in memory the distinct prime factors of each integer as it goes.");
    let s = "
use std::collections::HashMap;

pub fn euler47() -> u64 {
    let mut n = 1;
    let mut hmap = HashMap::<u64,Vec<u64>>::new();
    let mut ctr = 0;
    let out = loop {
        n += 1;
        if !hmap.contains_key(&n) {
            hmap.insert(n+n,vec![n]);
            ctr = 0
        } else {
            let factors = &hmap[&n].clone();
            if factors.len() == 4 {
                ctr += 1
            } else {
                ctr = 0
            }
            if ctr == 4 {
                break n-3;
            }
            for factor in factors {
                if hmap.contains_key(&(factor+n)) {
                    hmap.get_mut(&(factor+n)).unwrap().push(*factor);
                } else {
                    hmap.insert(factor+n,vec![*factor]);
                }
            }
            hmap.remove(&n);
        }
    };
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler47());
}

#[test]
fn test47() {
    assert_eq!(euler47(),134043)
}