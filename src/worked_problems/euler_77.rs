// Problem: What is the first value which can be written as the sum of primes in over five thousand different ways?
/*
*/

use crate::aux_funcs::{is_prime32};

pub fn euler77() -> u64 {
    let mut ctr = 2;
    let mut known_partitions = vec![0,0];
    let mut primes = vec![];
    loop {
        let mut num = 0;
        if is_prime32(ctr as u32) {
            primes.push(ctr);
            num += 1
        }
        for p in primes.iter() {
            let diff = ctr-p;
            if diff < *p {
                break
            }
            if primes.contains(&diff) {
                num += 1;
            }
            num += known_partitions[diff as usize]
        }
        println!("{}: {}",ctr,num);
        if num > 5000 {
            break ctr
        }
        known_partitions.push(num);
        ctr += 1;
    }
}


pub fn euler77_example() {
    println!("\nProblem: What is the first value which can be written as the sum of primes in over five thousand different ways?");
    println!("\n\n");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler77());
}


#[test]
fn test77() {
    assert_eq!(euler77(),71)
}