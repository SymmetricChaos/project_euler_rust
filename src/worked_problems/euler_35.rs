// How many circular primes are there below one million?

use crate::aux_funcs::{is_prime, int_to_digits, digits_to_int, prime_sieve};

fn cycle_digits(n: u64) -> u64 {
    let mut digits = int_to_digits(n,10);
    let d = digits.pop().unwrap();
    digits.insert(0,d);
    digits_to_int(&digits,10)
}


pub fn euler35() -> u64 {
    let mut p = prime_sieve();
    let mut out = 2; // include 2 and 5 which are otherwise excluded
    let disallowed_digits = [0,2,4,5,6,8];
    'outer: loop {
        let cur = p.next().unwrap();
        if cur > 1_000_000 {
            break;
        }
        let digits = int_to_digits(cur,10);
        for d in digits.iter() {
            if disallowed_digits.contains(d) {
                continue 'outer
            }
        }
        let mut temp = cur;
        for _ in 0..digits.len() {
            temp = cycle_digits(temp);
            if !is_prime(temp) {
                continue 'outer
            }
        }
        //println!("{}",cur);
        out += 1;
    }
    out
}
