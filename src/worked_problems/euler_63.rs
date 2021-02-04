// Problem: How many n-digit positive integers exist which are also an nth power?

/*
Again we need to find an upper bound. One must exist or the problem is impossible.
A bit of playing with powers shows that the base of the power must be between 0 and 9 since 10^p always has more than p digits.
Similarly 1 can only produce powers with 1 digit it doesn't ever need to be checked.
Since powers are increasing on the naturals if b^p has more than p-digits then b^(p+1) also has more than p-digits
Multiplying by any digit 1 to 9 either gives a value that has same number of digits or one additional digit. Thus for any digit b if b^p has less than p digits b^(p+1) has less than p+1 digits.
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
    // All the digits are 1-digit 1th powers, zero is not positive so it is excluded
    let mut ctr = 9;
    let mut p = 2;
    'outer: loop {
        let mut lo = 2;
        for n in lo..10 {
            let ex = BigInt::from(n).pow(p);
            let c = count_digits(ex.clone());
            if c == p {
                ctr += 1
            }
            if c < p {
                lo = n+1;
                if lo == 10 {
                    break 'outer
                }
            }

        }
        p += 1;
    }
    ctr
}

pub fn euler63_example() {
    println!("\nProblem: How many n-digit positive integers exist which are also an nth power?");
    println!("\n\nThis can only happen if the base of the power has one digits since 10^p always has p+1 digits. Second note that multiplting by a one digit number either gives a value with the same number of digits or with one more digit. So if a sequence of powers falls behind it can never catch up later on.");
    let s = "
pub fn euler63() -> u64 {
    // All the digits are 1-digit 1th powers, zero is not positive so it is excluded
    let mut ctr = 9;
    let mut p = 2;
    'outer: loop {
        let mut lo = 2;
        for n in lo..10 {
            let ex = BigInt::from(n).pow(p);
            let c = count_digits(ex.clone());
            if c == p {
                ctr += 1
            }
            if c < p {
                lo = n+1;
                if lo == 10 {
                    break 'outer
                }
            }

        }
        p += 1;
    }
    ctr
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler63());
}


#[test]
fn test63() {
    assert_eq!(euler63(),49)
}
