// Problem: How many elements would be contained in the set of reduced proper fractions for d ≤ 1,000,000?

/*
*/
use crate::aux_funcs::{totient_sieve};

pub fn euler72() -> u64 {
    let mut length = 0;
    let mut tot = totient_sieve();
    for _ in 0..=999_998 {
        length = length+tot.next().unwrap()
    }
    length
}

pub fn euler72_example() {
    println!("\nProblem: How many elements would be contained in the set of reduced proper fractions for d ≤ 1,000,000?");
    println!("\n\nThe length of Farey Sequences can be found by a recurrence relation and Euler's totient function.");
    let s = "
use crate::aux_funcs::{totient_sieve};

pub fn euler72() -> u64 {
    let mut length = 0;
    let mut tot = totient_sieve();
    for _ in 0..=999_998 {
        length = length+tot.next().unwrap()
    }
    length
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler72());
}


#[test]
fn test72() {
    assert_eq!(euler72(),303963552391)
}