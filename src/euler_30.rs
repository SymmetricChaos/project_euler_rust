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

// 

fn int_to_digits(n: u64) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut num = n;
    while num != 0 {
        let q = num/10;
        let r = num%10;
        digits.insert(0,r as u8);
        num = q;
    }
    return digits;
}

pub fn euler30() -> u64 {
    let v = int_to_digits(1234);
    println!("{:?}",v);
    let mut s = 0;
    for d in v {
        let digit = d as u64;
        s += digit.pow(5);
    }
    println!("{:?}",s);
    return 0u64;
}