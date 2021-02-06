// Problem: How many Lychrel numbers are there below ten-thousand?

/*
Due to the theoretical nature of these numbers, and for the purpose of this problem, we shall assume that a number is Lychrel until proven otherwise.
In addition you are given that for every number below ten-thousand, it will either (i) become a palindrome in less than fifty iterations, or, (ii) no one, with all the computing power that exists, has managed so far to map it to a palindrome.
*/

use crate::aux_funcs::{int_to_digits,vec_identical,digit_addition};

fn lychrel_step(digits: &Vec<u8>) -> Vec<u8> {
    let mut rdigits = digits.clone();
    rdigits.reverse();
    digit_addition(digits,&rdigits,10)
}

fn is_palindrome(digits: &Vec<u8>) -> bool {
    let mut rdigits = digits.clone();
    rdigits.reverse();
    vec_identical(&digits,&rdigits)
}

pub fn euler55() -> u64 {
    let mut ctr = 10000;
    for i in 0..10000u32 {
        let mut x = int_to_digits(i,10);
        x = lychrel_step(&x);
        for _ in 0..50 {
            if is_palindrome(&x) {
                ctr -= 1;
                break
            }
            x = lychrel_step(&x);
        }
    }
    ctr
}

pub fn euler55_example() {
    println!("\nProblem: How many Lychrel numbers are there below ten-thousand?");
    println!("\n\nSince this question involves palindromes obviously we need to convert numbers to Vectors of digits. We don't need to convert back, however. By implementing the addition algorithm on those vectors we can skip converting back and forth between Vector and u64. This also means we can handle integers of arbitrary size. This implementation is inefficient because it uses u64 to represent each digit.");
    let s = "
use crate::aux_funcs::{int_to_digits,vec_identical,digit_addition};

fn op_to_zero(input: Option<u64>) -> u64 {
    match input {
        Some(x) => x,
        None => 0,
    }
}

pub fn digit_addition<T: Unsigned + Zero + Copy>(a: &Vec<T>, b: &Vec<T>, base: T) -> Vec<T> {
    let mut ta = a.clone();
    let mut tb = b.clone();

    let zero = T::zero();

    let mut out: Vec<T> = Vec::new();

    let mut carry = zero;
    while ta.len() > 0 && tb.len() > 0 {
        let v1 = ta.pop().unwrap_or(zero);
        let v2 = tb.pop().unwrap_or(zero);
        let mut val = v1 + v2 + carry;
        carry = val / base;
        val = val % base;
        out.push(val)
    }

    if carry != zero {
        out.push(carry)
    }

    out.reverse();
    out
}

fn lychrel_step(digits: &Vec<u64>) -> Vec<u64> {
    let mut rdigits = digits.clone();
    rdigits.reverse();
    digit_addition(digits,&rdigits,10)
}

fn is_palindrome(digits: &Vec<u64>) -> bool {
    let mut rdigits = digits.clone();
    rdigits.reverse();
    vec_identical(&digits,&rdigits)
}

pub fn euler55() -> u64 {
    let mut ctr = 10000;
    for i in 0..10000 {
        let mut x = int_to_digits(i,10);
        x = lychrel_step(&x);
        for _ in 0..50 {
            if is_palindrome(&x) {
                ctr -= 1;
                break
            }
            x = lychrel_step(&x);
        }
    }
    ctr
}
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler55());
}

#[test]
fn test55() {
    assert_eq!(euler55(),249)
}