// What is the 10 001st prime number?

// we import the primes crate in order to get Sieve and PrimeSet
// woud like to come up with own implementation
use crate::aux_funcs::{prime_sieve};

pub fn euler7() -> u64 {
    let pset = prime_sieve();
    let out = pset.skip(10_000).next().unwrap();
    out
}