// Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.

/*
We can eliminate some products quickly. 

Products of one and two digit numbers cannot produce a product with more than four digits, 
since 99*99 = 9801, so none of those equations can be pandigital.

Likewise the smallest product pair of three digit numbers allowed is 135*246 = 33210 which
has too many digits to be pandigital.

It is possible for a one digit number and a for digit number to work as well.

Thus the only kinds of products to conider are 2D x 3D = 4D and 1D x 4D = 4D. This restricts
the possibiliites enough that no special tricks are needed.
*/

// Get all the 2-digit numbers allowed
fn all_valid_two_digit() -> Vec<[u32; 2]> {
    let digits = [1,2,3,4,5,6,7,8,9];
    let mut out = Vec::new();
    for a in digits.iter() {
        for b in digits.iter() {
            if a != b {
                out.push([*a,*b]);
            }
        }
    }
    return out;
}

// Get all the 3-digit numbers that pair with a given 2-digit number
fn three_digit_pair(n: [u32; 2]) -> Vec<[u32; 3]> {
    let mut digits = vec![1,2,3,4,5,6,7,8,9];
    if n[0] > n[1] {
        digits.remove(n[0] as usize - 1);
        digits.remove(n[1] as usize - 1);
    } else {
        digits.remove(n[1] as usize - 1);
        digits.remove(n[0] as usize - 1);
    }
    let mut out = Vec::new();
    for a in digits.iter() {
        for b in digits.iter() {
            for c in digits.iter() {
                if a != b && b != c && c != a{
                    out.push([*a,*b,*c]);
                }
            }
        }
    }
    return out;
}

fn int_to_digits(n: u64) -> [u32; 4] {
    let mut digits = [0u32; 4];
    let mut num = n;
    let mut ctr = 4;
    while num != 0 {
        ctr -= 1;
        let q = num/10;
        let r = num%10;
        digits[ctr] = r as u32;
        num = q;
    }
    return digits;
}

pub fn euler32() -> u64 {
    let mut out = 0;
    let two_digit = all_valid_two_digit();
    let mut prods = Vec::new();

    let ds = "[1, 2, 3, 4, 5, 6, 7, 8, 9]";

    for t in two_digit.iter() {
        let n = (10*t[0] + t[1]) as u64;

        let complement = three_digit_pair(*t);

        for c in complement.iter() {
            let m = (100*c[0] + 10*c[1] + c[2]) as u64;

            if n*m < 1000 || n*m > 9999 {
                continue
            }
            let p = int_to_digits(n*m);

            let mut v = Vec::new();

            for i in t.iter() {
                v.push(i);
            }
            for i in c.iter() {
                v.push(i);
            }
            for i in p.iter() {
                v.push(i);
            }

            v.sort();

            let vs = format!("{:?}",v);
            if vs == ds {
                if !prods.contains(&(m*n)) {
                    out += m*n;
                    prods.push(m*n);
                }
                println!("{} * {} = {}", n,m,n*m);
            }

        }
    }
    return out;
}