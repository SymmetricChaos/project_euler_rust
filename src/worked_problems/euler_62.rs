// Problem: Find the smallest cube for which exactly five permutations of its digits are cube.
/*

*/

use num::integer::Roots;
use itertools::Itertools;
use crate::aux_funcs::{int_to_digits};

pub fn digits_to_int(digits: &Vec<&u8>, base: u64) -> u64 {
    let mut ctr = digits.len();
    let mut pow = 1;
    let mut out = 0;
    while ctr > 0 {
        ctr -= 1;
        let d = *digits[ctr] as u64;
        out += pow*d;
        pow = pow*base;
    }
    out as u64
}

fn is_cube(n: u64) -> bool {
    let cbrt = n.cbrt();
    if cbrt*cbrt*cbrt == n {
        return true
    }
    return false
}

pub fn euler62() -> u64 {
    // Start with five because there cube must have at least three digits
    let mut n = 5;
    loop {
        let mut ctr = 0;
        let cube = n*n*n;
        let digits = int_to_digits(cube,10);
        for p in digits.iter().permutations(digits.len()).unique() {
            if *p[0] == 0 {
                continue
            }
            let c = digits_to_int(&p,10);
            if is_cube(c) {
                ctr += 1
            }
            if ctr == 5 {
                return cube
            }
        }
        n += 1
    }
    0u64
}

pub fn euler62_example() {
    println!("\nProblem: Find the smallest cube for which exactly five permutations of its digits are cube.");
    println!("\n\n");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler62());
}


#[test]
fn test62() {
    assert_eq!(euler62(),127035954683)
}
