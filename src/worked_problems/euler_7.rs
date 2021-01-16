// What is the 10 001st prime number?

// we import the primes crate in order to get Sieve and PrimeSet
// woud like to come up with own implementation
use primes::{Sieve, PrimeSet};

pub fn euler7() -> u64 {
    let mut pset = Sieve::new();
    let out = pset.iter().skip(10_000).next().unwrap();
    out
}