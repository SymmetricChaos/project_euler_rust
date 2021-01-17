use std::convert::TryFrom;
use std::collections::HashMap;

pub fn int_to_digits(n: u64, base: u64) -> Vec<u64> {
    let mut digits = Vec::new();
    let mut num = n;
    while num != 0 {
        let q = num/base;
        let r = num%base;
        digits.insert(0,r as u64);
        num = q;
    }
    return digits;
}

pub fn digits_to_int<T: Copy + Into<u64>>(digits: &Vec<T>, base: u64) -> u64 {
    let mut ctr = digits.len();
    let mut pow = 1;
    let mut out = 0;
    while ctr > 0 {
        ctr -= 1;
        out += pow*u64::try_from(digits[ctr]).unwrap();
        pow = pow*base;
    }
    out as u64
}

pub fn gcd(a: u64, b: u64) -> u64 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    return x;
}

pub fn pow_mod(n: u64, e: u64, m: u64) -> u64 {
    if e == 0 {
        return 1;
    }
    let mut result = 1u128;
    let base = n as u128;
    let modulus = m as u128;
    for _ in 0..e {
        result = (result * base) % modulus;
    }
    result as u64
}

// 64-bit primality test
// First checks small possible factors then switches to deterministic Miller-Rabin
pub fn is_prime(n: u64) -> bool {

    if n <= 1 {
        return false;
    }

    // Check the witnesses using simple division
    // There are two reasons for this.
    // First it eliminates 85% of composite numbers immediately
    // Secondly if we don't do this the witnesses themselves will be reported as
    // composite by the later portion of the test
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



pub struct PrimeSieve {
    sieve: HashMap::<u64,Vec<u64>>,
    n: u64,
}

impl Iterator for PrimeSieve {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            self.n += 1;
            if !self.sieve.contains_key(&self.n) {
                self.sieve.insert(self.n+self.n,vec![self.n]);
                return Some(self.n)
            } else {
                let factors = &self.sieve[&self.n].clone();
                for factor in factors {
                    if self.sieve.contains_key(&(factor+self.n)) {
                        self.sieve.get_mut(&(factor+self.n)).unwrap().push(*factor);
                    } else {
                        self.sieve.insert(factor+self.n,vec![*factor]);
                    }
                }
                self.sieve.remove(&self.n);
            }
        }
    }
}

pub fn prime_sieve() -> PrimeSieve {
    PrimeSieve{
        sieve: HashMap::<u64,Vec<u64>>::new(),
        n: 1u64}
}

pub fn prime_factorization(n: u64) -> HashMap<u64,u64> {
    let mut canon = HashMap::new();
    let mut x = n;
    let mut primes = prime_sieve();
    loop {
        let p = primes.next().unwrap();
        println!("{}",x);
        let mut ctr = 0;
        while x % p == 0 {
            x /= p;
            ctr += 1;
        }
        if ctr != 0 {
            canon.insert(p,ctr);
        }
        if x == 1 {
            break
        }
    }
    canon
}