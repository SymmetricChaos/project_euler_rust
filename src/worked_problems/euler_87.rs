// Problem: How many numbers below fifty million can be expressed as the sum of a prime square, prime cube, and prime fourth power?

/*
The greater part of the sum will be the 3rd power and 4th power. Fortunately there are not many of these and in fact only 1552 possible sums less than fifty million.
There are 908 2nd powers we need to consider.
*/

use crate::aux_funcs::prime_sieve;
use std::collections::HashSet;

pub fn euler87() -> u64 {
    let mut little = Vec::new();
    let mut big = Vec::new();
    let mut cube = Vec::new();
    let mut quad = Vec::new();

    let mut primes = prime_sieve();

    // Get the squares, cubes, and 4th powers
    loop {
        let p = primes.next().unwrap();
        if p*p >= 50_000_000 {
            break
        }
        if p*p < 50_000_000 {
            little.push(p*p)
        }
        if p*p*p < 50_000_000 {
            cube.push(p*p*p)
        }
        if p*p*p*p < 50_000_000 {
            quad.push(p*p*p*p)
        }
    }

    // Merge the cubes and 4th powers
    for c in cube {
        for q in &quad {
            if c+q < 50_000_000 {
                big.push(c+q)
            }
        }
    }

    // Count
    let mut known = HashSet::<u64>::new();
    let mut out = 0;
    for n in big {
        let mut ctr = 0;
        for m in &little {
            let s = n+m;

            if s > 50_000_000 {
                break
            } else {
                if !known.contains(&s) {
                    ctr += 1;
                    known.insert(s);
                }
            }
        }
        out += ctr;
    }

    out as u64
}

pub fn euler87_example() {
    println!("\nProblem: How many numbers below fifty million can be expressed as the sum of a prime square, prime cube, and prime fourth power?");
    println!("\n\nThe first thing that should be obvious about this problem is that we can't do it by checking each number and seeing if it can be broken down this way, that's much too complicated to do fifty million times. Instead we'll create the number from the parts. To avoid doing extra work we can combine the 3rd and 4th powers into a single number these are all unique and there are only 1552 of them. Then we can just combine each of these numbers with each of the squares and check if the result is less than fifty millon. However these combinations are not all unique so we keep track of what we've seen before.");
    let s = "
use crate::aux_funcs::prime_sieve;
use std::collections::HashSet;

pub fn euler87() -> u64 {
    let mut little = Vec::new();
    let mut big = Vec::new();
    let mut cube = Vec::new();
    let mut quad = Vec::new();

    let mut primes = prime_sieve();

    // Get the squares, cubes, and 4th powers
    loop {
        let p = primes.next().unwrap();
        if p*p >= 50_000_000 {
            break
        }
        if p*p < 50_000_000 {
            little.push(p*p)
        }
        if p*p*p < 50_000_000 {
            cube.push(p*p*p)
        }
        if p*p*p*p < 50_000_000 {
            quad.push(p*p*p*p)
        }
    }

    // Merge the cubes and 4th powers
    for c in cube {
        for q in &quad {
            if c+q < 50_000_000 {
                big.push(c+q)
            }
        }
    }

    // Count
    let mut known = HashSet::<u64>::new();
    let mut out = 0;
    for n in big {
        let mut ctr = 0;
        for m in &little {
            let s = n+m;

            if s > 50_000_000 {
                break
            } else {
                if !known.contains(&s) {
                    ctr += 1;
                    known.insert(s);
                }
            }
        }
        out += ctr;
    }

    out as u64
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler87());
}


#[test]
fn test87() {
    assert_eq!(euler87(),1097343)
}
