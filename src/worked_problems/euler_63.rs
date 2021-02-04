// Problem: How many n-digit positive integers exist which are also an nth power?

/*
Again we need to find an upper bound. One must exist or the problem is impossible.
A bit of playing with powers shows that the base of the power must be between 0 and 9 since 10^p always has more than p digits.
Since powers are increasing on the naturals if 9^p has more than p-digits then 9^(p+1) also has more than p-digits
*/

use num::bigint::BigInt;
use num::traits::{Zero};

fn count_digits(n: BigInt) -> u32 {
    let mut ctr = 1;
    let mut n = n;
    n /= 10;
    while n != BigInt::zero() {
        ctr += 1;
        n /= 10;
    }
    ctr
}


pub fn euler63() -> u64 {
    // All the digits are 1-digit 1th powers
    let mut ctr = 10;
    let mut p = 2;
    'outer: loop {
        for n in 2..10 {
            let ex = BigInt::from(n).pow(p);
            let c = count_digits(ex.clone());
            if c == p {
                println!("{}^{} = {}",n,p,ex);
                ctr += 1
            } else if c > p {
                if n == 9 {
                    break 'outer;
                }
                break
            }
        }
        p += 1;
    }
    ctr
}

pub fn euler63_example() {
    println!("\nProblem: How many n-digit positive integers exist which are also an nth power?");
    println!("\n\n");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler63());
}


#[test]
fn test63() {
    assert_eq!(euler63(),49)
}
