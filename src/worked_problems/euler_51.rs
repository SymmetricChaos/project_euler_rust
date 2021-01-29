// Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits) with the same digit, is part of an eight prime value family.

/*
Because we are asked for an eight digit family that means at least one of the replacements will be of either 3s, 6s, or 9s. Thus we can eliminate any canididate that is
divisble by 3 when the replacement is set to 0. Otherwise those digits would give a number that is still divisible by 3 and not prime. Also for a family of eight we have
to use at least one even number so the replacement cannot be the last digit.

Five digit numbers with two replacements
**ddd
d**dd
dd**d
*d*dd
*dd*d
d*d*d

Five digit numbers with three replacements
***dd
**d*d
*d**d
d***d

*/

use crate::aux_funcs::{is_prime,digits_to_int};
use itertools::izip;

fn test_n(n: usize, any: &Vec<usize>, final_digit: u64, positions: &Vec<usize>) -> Vec<u64> {
    let mut out = Vec::new();
    for x in 0..10 {
        let mut v = vec![x;n-1];
        v.push(final_digit as u8);
        for (pos,val) in izip!(positions.iter(), any.iter()) {
            v[*pos] = *val as u8;
        }
        if v[0] == 0 {
            continue
        }
        if is_prime(digits_to_int(&v,10)) {
            out.push(digits_to_int(&v,10));
        }
    }

    out
}

fn test_all_6_2() -> u64 {
    let mut out = 0;
    let constant_positions = [vec![0,1], vec![0,2], vec![0,3], vec![0,4], vec![1,2], vec![1,3], vec![1,4], vec![2,3], vec![2,4], vec![3,4]];
    let final_digits = [1,3,7,9];
    let digits = [0,1,2,3,4,5,6,7,8,9];
    for c in &constant_positions {
        for f in &final_digits {
            for d1 in &digits {
                for d2 in &digits {
                    let t = test_n(6,&vec![*d1,*d2],*f,c);
                    if t.len() == 8 {
                        out = t[0];
                    }
                }
            }
        }
    }
    out
}

pub fn euler51() -> u64 {
    test_all_6_2()
}

pub fn euler51_example() {
    println!("\nProblem: Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits) with the same digit, is part of an eight prime value family.");
    println!("\n\nI can't find a way to properly automate this. Testing all possible 6 digit numbers with two replacements turns out to work.");
    let s = "
use crate::aux_funcs::{is_prime,digits_to_int};
use itertools::izip;

fn test_n(n: usize, any: &Vec<usize>, final_digit: u64, positions: &Vec<usize>) -> Vec<u64> {
    let mut out = Vec::new();
    for x in 0..10 {
        let mut v = vec![x;n-1];
        v.push(final_digit);
        for (pos,val) in izip!(positions.iter(), any.iter()) {
            v[*pos] = *val as u64;
        }
        if v[0] == 0 {
            continue
        }
        if is_prime(digits_to_int(&v,10)) {
            out.push(digits_to_int(&v,10));
        }
    }
    out
}

fn test_all_6_2() -> u64 {
    let mut out = 0;
    let constant_positions = [vec![0,1], vec![0,2], vec![0,3], vec![0,4], vec![1,2], vec![1,3], vec![1,4], vec![2,3], vec![2,4], vec![3,4]];
    let final_digits = [1,3,7,9];
    let digits = [0,1,2,3,4,5,6,7,8,9];
    for c in &constant_positions {
        for f in &final_digits {
            for d1 in &digits {
                for d2 in &digits {
                    let t = test_n(6,&vec![*d1,*d2],*f,c);
                    if t.len() == 8 {
                        out = t[0];
                    }
                }
            }
        }
    }
    out
}

pub fn euler51() -> u64 {
    test_all_6_2()
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler51());
}

#[test]
fn test51() {
    assert_eq!(euler51(),121313)
}