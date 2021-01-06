// Evaluate the sum of all the amicable numbers under 10000.

fn aliquot_sum(n:u64) -> u64 {
    let mut out = 0;
    for f in 1..n {
        if n % f == 0 {
            out += f;
        }
    }
    return out;
}


pub fn euler21() -> u64 {
    let mut out = 0u64;
    for n in 2..10000 {
        let m = aliquot_sum(n);
        if m > n {
            if m < 10000{
                if aliquot_sum(m) == n {
                    out += m + n;
                }
            } else {
                if aliquot_sum(m) == n {
                    out += n;
                }
            }
        }
    }
    return out;
}