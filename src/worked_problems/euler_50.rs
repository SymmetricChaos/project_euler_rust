// Which prime, below one-million, can be written as the sum of the most consecutive primes?

/*
The full text of the problem gives us the hint that the sum has at least 21 primes
The chain has an even number of terms if and only if it starts at 2, otherwise the sum
    would be even
We can also be sure that the primes are "small" otherwise the chain could not be long
Specifically the chain cannot start with a number greater than 48,000 since the sum would
    have to be greater than one million
*/

use crate::aux_funcs::{prime_sieve,is_prime};

fn chains_from_two() -> (u64,u64) {
    let mut pset = prime_sieve();
    let mut num: u64 = 0;
    let mut length: u64 = 0;
    let mut cur = 0;
    let mut ctr = 0;
    loop {
        cur += pset.next().unwrap() + pset.next().unwrap();
        ctr += 2;
        if cur >= 1_000_000 {
            break
        }
        if is_prime(cur) {
            num = cur;
            length = ctr;
        }
    }
    (num,length)
}

pub fn euler50() -> u64 {
    let (mut out, mut length) = chains_from_two();
    out
}

pub fn euler50_example() {
    println!("\nProblem: Which prime, below one-million, can be written as the sum of the most consecutive primes?");
    println!("\n\n");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler50());
}

#[test]
fn test50() {
    assert_eq!(euler50(),997651)
}