// Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.

/*
We can eliminate some products quickly. 

Products of one and two digit numbers cannot produce a product with more than four digits, 
since 99*99 = 9801, so none of those equations can be pandigital.

Likewise the smallest product pair of three digit numbers allowed is 135*246 = 33210 which
has too many digits to be pandigital.

Thus only products of a two digit and a three digit number can possibly work. This means
there are only about 72*35 = 2520 posibilities to check, no clever optimitizted needed.
*/

fn all_valid_two_digit() -> Vec<u64> {
    let digits = [1,2,3,4,5,6,7,8,9];
    let mut out = Vec::new();
    for a in digits.iter() {
        for b in digits.iter() {
            if a != b {
                out.push(a*10+b);
            }
        }
    }
    return out;
}

pub fn euler32() -> u64 {
    let two_digit = all_valid_two_digit();
    for t in two_digit {
        println!("{}",t);
    }
    return 0u64;
}