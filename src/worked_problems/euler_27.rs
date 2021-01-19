// Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum number of primes for consecutive values of n, starting with n=0.
// n^2 + an + b, for |a| < 1000 and |b| < 1000

use crate::aux_funcs::{is_prime};

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
        t
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
    out
}

pub fn euler27_example() {
    println!("\nProblem: Find the product of the coefficients, a and b, for the quadratic expression n^2 + an + b, for |a| < 1000 and |b| < 1000, that produces the maximum number of primes for consecutive values of n, starting with n=0.");
    println!("\n\nA struct to represent the quadratics with an iterator implemented makes interacting easy. The is_prime function is a deterministic 64-bit Miller Rabin test.");
    let s = "
pub fn is_prime(n: u64) -> bool {

    if n <= 1 {
        return false;
    }
    let witnesses = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for p in witnesses.iter() {
        if n == *p {
            return true;
        }
        if n % *p == 0 {
            return false;
        }
    }
    let mut d = (n-1)/2;
    let mut r = 1;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }
    'outer: for w in witnesses.iter() {
        let mut x = pow_mod(*w,d,n);
        
        if x == 1 || x == n-1 {
            continue 'outer;
        }
        for _ in 0..r-1 {
            x = pow_mod(x,2,n);
            
            if x == n-1 {
                 continue 'outer;
            }
        }
        return false;
    }
    true
}

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
        t
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
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler27());
}

#[test]
fn test27() {
    assert_eq!(euler27(),-59231)
}