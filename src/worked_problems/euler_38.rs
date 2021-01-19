// What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated product of an integer with (1,2, ... , n) where n > 1?

/*
The additional information provided gives us a big boost here as we know that there are
qualifying numbers at least as big as 918273645. Because of this we can immediately reject
any number that does not has 9 as its first digit.

We also need to know when to stop
*/

use crate::aux_funcs::{int_to_digits, digits_to_int};

pub fn euler38() -> u64 {
    let mut out = 0;
    let ds = "[1, 2, 3, 4, 5, 6, 7, 8, 9]";
    let mut cur = 9;

    'outer: while cur < 10_000 {
        cur += 1;

        let mut digits = int_to_digits(cur,10);
        if digits[0] != 9 {
            continue 'outer
        }

        let mut n = 2;
        loop {
            let product = int_to_digits(cur*n,10);
            digits.extend(product);
            n += 1;
            if digits.contains(&0) {
                continue 'outer
            }
            if digits.len() == 9 {
                break;
            }
            if digits.len() > 9 {
                continue 'outer
            }
        }

        let val = digits_to_int(&digits.clone(),10);
        digits.sort();

        if ds == format!("{:?}",digits) && val > out {
            //println!("{}",val);
            out = val;
        }
    }
    out
}

#[test]
fn test38() {
    assert_eq!(euler38(),932718654)
}