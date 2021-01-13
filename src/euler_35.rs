// How many circular primes are there below one million?

use crate::aux_funcs::{is_prime,pow_mod};

pub fn euler35() -> u64 {
    for i in 100..200 {
        if is_prime(i) {
            println!("{}",i);
        }
    }
    return 0u64;
}