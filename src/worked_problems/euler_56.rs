// Problem: Considering natural numbers of the form, a^b, where a, b < 100, what is the maximum digital sum?

/*

*/


use crate::aux_funcs::{int_to_digits,digit_multiplication};

fn digit_sum(digits: &Vec<u8>) -> u64 {
    let mut sum = 0u64;
    for d in digits.iter() {
        sum += *d as u64
    }
    sum
}

pub fn euler56() -> u64 {
    let mut biggest = 0u64;
    for n in 2..100 {
        let base = int_to_digits(n,10);
        let mut a = int_to_digits(n,10);
        for _ in 0..99 {
            a = digit_multiplication(&a,&base,10);
            let s = digit_sum(&a);
            if s > biggest {
                biggest = s
            }
        }
    }
    biggest
}

pub fn euler56_example() {
    println!("\nProblem: Considering natural numbers of the form, a^b, where a, b < 100, what is the maximum digital sum?");
    println!("\n\n");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler56());
}

#[test]
fn test56() {
    assert_eq!(euler56(),972)
}