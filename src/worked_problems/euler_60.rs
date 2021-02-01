// Problem: Find the lowest sum for a set of five primes for which any two primes concatenate to produce another prime.
/*
We can exclude 2 and 5, obviously.
Memoization will probably help here as we are only interested in concatenating pairs of primes.

*/

use crate::aux_funcs::{is_prime,prime_sieve};
use std::collections::HashMap;


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

pub fn euler60() -> u64 {
    let mut out = 0u64;

    let mut valid_pairs: HashMap<u64,Vec<u64>> = HashMap::new();

    let mut prime_iter = prime_sieve();
    let mut primes = Vec::<u64>::new();

    for _ in 0..20 {
        let p = prime_iter.next().unwrap();
        if p == 2 || p == 5 {
            continue
        }
        for v in primes.iter() {
            if is_pair(p,*v) {
                valid_pairs.get_mut(v).unwrap().push(p);
            }
        }
        primes.push(p)
    }
    println!("{:?}",valid_pairs);
    out
}

pub fn euler60_example() {
    println!("\nProblem: Find the lowest sum for a set of five primes for which any two primes concatenate to produce another prime.");
    println!("\n\n");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler60());
}

/*
#[test]
fn test60() {
    assert_eq!(euler60(),)
}
*/