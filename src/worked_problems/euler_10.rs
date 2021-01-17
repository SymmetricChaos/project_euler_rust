// Find the sum of all the primes below two million.

use crate::aux_funcs::{prime_sieve};

pub fn euler10() -> u64 {
    let pset = prime_sieve();
    let mut out = 0;
    for p in pset {
        if p > 2_000_000 {
            break;
        }
        out += p;
    }
    out
}