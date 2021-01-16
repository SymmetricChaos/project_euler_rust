// Evaluate the sum of all the amicable numbers under 10000.

fn aliquot_sum(n:u64) -> u64 {
    if n == 0 {
        return 0u64;
    }
    let lim = (n as f64).sqrt().floor() as u64;
    let mut out = 1;
    for f in 2..lim+1 {
        if n % f == 0 {
            if f != n/f {
                out += f + n/f;
            } else {
                out += f;
            }
        }
    }
    out
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
    out
}