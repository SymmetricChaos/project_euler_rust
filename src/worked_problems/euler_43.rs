// Find the sum of all 0 to 9 pandigital numbers with this property.

use itertools::Itertools;
use crate::aux_funcs::{digits_to_int};


pub fn euler43() -> u64 {
    let mut out = 0;

    // All the 1 to 9 pandigital numbers
    let perms = (1..10u64).permutations(9).into_iter();

    for p in perms {
        println!("{}",digits_to_int(p,10))
    }

    out
}