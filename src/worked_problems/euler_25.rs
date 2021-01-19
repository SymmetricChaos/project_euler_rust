// What is the index of the first term in the Fibonacci sequence to contain 1000 digits?

use num::bigint::BigInt;

pub fn euler25() -> u64 {
    let mut a = BigInt::from(0);
    let mut b = BigInt::from(1);
    let mut ctr = 0u64;
    loop {
        if a.to_str_radix(10).len() >= 1000 {
            break;
        } else {
            let new = &a+&b;
            a = b;
            b = new;
        }
        ctr += 1;
    }
    ctr
}

pub fn euler25_example() {
    println!("\nProblem: What is the index of the first term in the Fibonacci sequence to contain 1000 digits?");
    println!("\n\nRather than use a struct like in the previous Fibonacci problem here we just loop through Fibonacci numbers using the BigInt library.");
    let s = "
use num::bigint::BigInt;

pub fn euler25() -> u64 {
    let mut a = BigInt::from(0);
    let mut b = BigInt::from(1);
    let mut ctr = 0u64;
    loop {
        if a.to_str_radix(10).len() >= 1000 {
            break;
        } else {
            let new = &a+&b;
            a = b;
            b = new;
        }
        ctr += 1;
    }
    ctr
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler25());
}

#[test]
fn test25() {
    assert_eq!(euler25(),4782)
}