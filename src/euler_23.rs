// By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers.
// Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

fn aliquot_sum(n:u64) -> u64 {
    if n == 0 {
        return 0u64;
    }
    let lim = (n as f64).sqrt() as u64;
    let mut out = 1;
    for f in 2..lim {
        if n % f == 0 {
            out += f + n/f;
        }
    }
    return out;
}

// Get all the abundant numbers less than 28124
fn abudant_numbers() -> Vec<u64> {
    let mut v = Vec::new();
    for n in 1..28124 {
        if aliquot_sum(n) > n {
            v.push(n);
        }
    }
    return v;
}

fn is_ab_sum(n: u64, abundants: &Vec<u64>) -> bool {
    for a in abundants {
        if n > *a {
            if abundants.contains(&(n-a)) {
                return true;
            }
        } else {
            return false;
        }
    }
    return false;
}

// This either doesn't work or is much too slow
pub fn euler23() -> u64 {
    let mut out = 0u64;
    let abundants = abudant_numbers();
    for n in 2..28124 {
        if !is_ab_sum(n,&abundants) {
            out += n;
        }
    }
    return out;
}