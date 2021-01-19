// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.

// We have to narrow down the range of options to do so we'll start with the fifth powers of
// each digit.
// 0^5 = 0
// 1^5 = 1
// 2^5 = 32
// 3^5 = 243
// 4^5 = 1024
// 5^5 = 3125
// 6^5 = 7776
// 7^5 = 16807
// 8^5 = 32768
// 9^5 = 59049

// An upper bound is not too hard to find. All numbers that qualify must be less than
// 999999 since the sum of the 5th powers of its digits is only 354294.

fn int_to_digits(n: u64) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut num = n;
    while num != 0 {
        let q = num/10;
        let r = num%10;
        digits.insert(0,r as u8);
        num = q;
    }
    digits
}

fn pow_digit_sum(n: u64, p: u64) -> u64 {
    let digits = int_to_digits(n);
    let mut out = 0;
    for d in digits {
        let digit = d as u64;
        out += digit.pow(p as u32);
    }
    out
}

pub fn euler30() -> u64 {
    let mut out = 0;
    for n in 2..1000000 {
        if pow_digit_sum(n,5u64) == n {
            out += n;
        }
    }
    out
}

pub fn euler30_example() {
    println!("\nProblem: Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.");
    println!("\n\nObserve that the sum of the 5th powers of 999999 is 354294, which is enough to set a loose upper bound.");
    let s = "
fn int_to_digits(n: u64) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut num = n;
    while num != 0 {
        let q = num/10;
        let r = num%10;
        digits.insert(0,r as u8);
        num = q;
    }
    digits
}

fn pow_digit_sum(n: u64, p: u64) -> u64 {
    let digits = int_to_digits(n);
    let mut out = 0;
    for d in digits {
        let digit = d as u64;
        out += digit.pow(p as u32);
    }
    out
}

pub fn euler30() -> u64 {
    let mut out = 0;
    for n in 2..1000000 {
        if pow_digit_sum(n,5u64) == n {
            out += n;
        }
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler30());
}

#[test]
fn test30() {
    assert_eq!(euler30(),443839)
}