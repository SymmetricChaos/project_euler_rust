// What is the largest n-digit pandigital prime that exists?

/*
No number that is 1-9 pandigtal can also be prime because all of them are divisible by 3
The same is true for numbers for 8, 6, 5, 3
*/

use crate::aux_funcs::{is_prime,digits_to_int};
use itertools::Itertools;

pub fn euler41() -> u64 {
    let mut out = 0u64;
    let max_vals = [7,4,3,2];
    for max_val in max_vals.iter() {
        out = 0;
        let digits = (1..=*max_val).rev();
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
    out
}

pub fn euler41_example() {
    println!("\nWhat is the largest n-digit pandigital prime that exists?");
    println!("\n\nTo make this efficient enough to finish quickly we need several shortcuts. First we will use permutations to generate the pandigital numbers we want rather than search for them through all the integers. Second we can use the divisibility by 3 rule to determien that the only pandigital numbers of length 2, 3, 4, and 7 can be prime.");
    let s = "
use crate::aux_funcs::{is_prime,digits_to_int};
use itertools::Itertools;

pub fn euler41() -> u64 {
    let mut out = 0;
    let max_vals = [7,4,3,2];
    for max_val in max_vals.iter() {
        out = 0;
        //println!{\"{}\",max_val};
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
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler41());
}

#[test]
fn test41() {
    assert_eq!(euler41(),7652413)
}