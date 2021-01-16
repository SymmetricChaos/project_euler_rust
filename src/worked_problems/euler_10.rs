// Find the sum of all the primes below two million.

// we import the primes crate in order to get Sieve and PrimeSet
// woud like to come up with own implementation
use primes::{Sieve, PrimeSet};

pub fn euler10() -> u64 {
    let mut pset = Sieve::new();
    let mut out = 0;
    for p in pset.iter() {
        if p > 2_000_000 {
            break;
        }
        out += p;
    }
    out
}