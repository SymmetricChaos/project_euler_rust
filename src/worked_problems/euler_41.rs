// What is the largest n-digit pandigital prime that exists?

/*
No number that is 1-9 pandigtal can also be prime because all of them are divisible by 3
The same is true for numbers for 8, 6, 5, 3
*/

use crate::aux_funcs::{is_prime,digits_to_int};
use itertools::Itertools;

pub fn euler41() -> u64 {
    let mut out = 0;
    let max_vals = [7,4,3,2];
    for max_val in max_vals.iter() {
        out = 0;
        //println!{"{}",max_val};
        let digits = (1..max_val+1u64).rev();
        let perms = digits.permutations(*max_val as usize).into_iter();
        for p in perms {
            let cur = digits_to_int(&p,10);
            if cur > out && is_prime(cur as u64) {
                out = cur;
            }
        }
        if out > 0 {
            break
        }
    }
    out as u64
}