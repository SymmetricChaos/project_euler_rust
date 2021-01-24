use std::convert::TryFrom;
use std::collections::HashMap;
use mod_exp::mod_exp;

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

// 64-bit primality test
// First checks small possible factors then switches to deterministic Miller-Rabin
pub fn is_prime(n: u64) -> bool {

    if n <= 1 {
        return false;
    }

    // Check all primes below 100 and all witnesses
    // This quickly eliminates the vast majority of composite numbers
    let small_factors = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 325, 9375, 28178, 450775, 9780504, 1795265022];

    for p in small_factors.iter() {
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

    // Witnesses found by Jim Sinclair
    let witnesses = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
    
    'outer: for w in witnesses.iter() {
        let mut x = mod_exp(*w,d,n);
        
        if x == 1 || x == n-1 {
            continue 'outer;
        }
        for _ in 0..r-1 {
            x = mod_exp(x,2,n);
            
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

pub fn vec_identical<T: PartialEq>(va: &[T], vb: &[T]) -> bool {
    (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter()
       .zip(vb)
       .all(|(a,b)| *a == *b)
}

/*
fn should_swap(list: &Vec<u64>, index: usize, pos: usize) -> bool {
    for i in index..pos {
        if list[i] == list[pos] {
            return false
        }
    }
    true
}

// By generating distinct permutations we can produce digits with 0s for blank spaces
pub fn distinct_permutations(list: &mut Vec<u64>, index: usize) -> Vec<Vec<u64>> {
    let length = list.len();
    let mut out = Vec::new();
    if index == length {
        return vec![list.clone().to_vec()]
    }
    for i in index..length {
        if should_swap(&list, index, i) {
            list.swap(i,index);
            out.extend(distinct_permutations(list,index+1));
            list.swap(i,index);
        }
    }
    out
}
*/