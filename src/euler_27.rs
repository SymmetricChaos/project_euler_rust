// Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum number of primes for consecutive values of n, starting with n=0.
// n^2 + an + b, for |a| < 1000 and |b| < 1000

use primes::is_prime;

struct Quadratic {
    a: i64,
    b: i64,
    n: i64,
}

impl Iterator for Quadratic {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        let t = Some(self.n*self.n + self.n*self.a + self.b);
        self.n += 1;
        //emit Some(number) or None if the iterator terminates
        return t;

    }
}


pub fn euler27() -> i64 {
    let mut longest_chain = 0;
    let mut out = 0;
    
    for a in -999..1000 {
        for b in -999..1000 {
            let mut ctr = 0;
            let mut q = Quadratic {a: a, b: b, n: 0};
            loop {
                let p = q.next().unwrap();
                if p < 0 {
                    break;
                } else if !is_prime(p.abs() as u64) {
                    break;
                }
                ctr += 1;
            }
            if ctr > longest_chain {
                longest_chain = ctr;
                out = a*b;
            }
        }
    }
    
    return out;
}