// Find the sum of the only eleven primes that are both truncatable from left to right and right to left.

use crate::aux_funcs::{int_to_digits, prime_sieve, is_prime};

fn is_double_truncatable(n: u64) -> bool {
    let mut x = n;
    let mut modulus = 1;
    // Test right truncation and use the modulus we will use in the next test
    while x > 9 {
        modulus *= 10;
        x /= 10;
        if !is_prime(x) {
            return false
        }
    }
    // Test left truncation
    let mut x = n;
    while x > 9 {
        x = x % modulus;
        modulus /= 10;
        if !is_prime(x) {
            return false
        }
    }
    true
}

pub fn euler37() -> u64 {
    let mut out = 0;
    let mut ctr = 0;
    let mut primes = prime_sieve();
    let disallowed_digits = [0,4,6,8];

    'outer: while ctr < 11 {
        let p = primes.next().unwrap();

        if p < 10 {
            continue
        }
        let digits = int_to_digits(p,10);
        for d in digits.iter() {
            if disallowed_digits.contains(d) {
                continue 'outer
            }
        }
        if is_double_truncatable(p) {
            println!("{}",p);
            ctr += 1;
            out += p;
        }
    }
    out
}