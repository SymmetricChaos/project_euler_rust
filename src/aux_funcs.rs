use std::fmt::Debug;
use std::convert::TryFrom;
use std::collections::HashMap;
use mod_exp::mod_exp;
use num::traits::{Unsigned,Zero,ToPrimitive};
use std::cmp::max;

pub fn int_to_digits<T: Unsigned + Zero + ToPrimitive + Clone>(n: T, base: T) -> Vec<u8> {
    if n == T::zero() {
        return vec![0u8]
    }
    let mut digits = Vec::new();
    let mut num = n;
    while num != T::zero() {
        let number = num.clone();
        let divisor = base.clone();
        let q = number.clone() / divisor.clone();
        let r = number % divisor;

        digits.insert(0,r.to_u8().unwrap());
        num = q;
    }
    return digits;
}

pub fn digits_to_int(digits: &Vec<u8>, base: u64) -> u64 {
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

pub fn digit_addition<T: Unsigned + Zero + Copy>(a: &Vec<T>, b: &Vec<T>, base: T) -> Vec<T> {
    let mut ta = a.clone();
    let mut tb = b.clone();

    let length = max(ta.len(),tb.len());
    let zero = T::zero();

    let mut out: Vec<T> = Vec::new();

    while ta.len() < tb.len() {
        ta.insert(0,zero)
    }
    while tb.len() < ta.len() {
        tb.insert(0,zero)
    }

    let mut carry = zero;
    for _ in 0..length {
        let v1 = ta.pop().unwrap_or(zero);
        let v2 = tb.pop().unwrap_or(zero);
        let mut val = v1 + v2 + carry;
        carry = val / base;
        val = val % base;
        out.push(val)
    }

    if carry != zero {
        out.push(carry)
    }

    out.reverse();
    out
}

pub fn digit_multiplication<T: Unsigned + Zero + Copy + Debug>(a: &Vec<T>, b: &Vec<T>, base: T) -> Vec<T> {

    let zero = T::zero();
    let mut out = vec![zero];
    let mut offset = 0;

    for d1 in b.iter().rev() {
        let mut partial: Vec<T> = Vec::new();
        for _ in 0..offset {
            partial.push(zero);
        }
        let mut carry = zero;
        for d2 in a.iter().rev() {
            let mut val = *d1 * *d2 + carry;
            carry = val / base;
            val = val % base;
            partial.push(val)
        }
        if carry != zero {
            partial.push(carry)
        }
        out = digit_addition(&partial,&out,base);
        offset += 1;
    }
    out.reverse();
    out
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
        let mut x = mod_exp(*w as u128,d as u128,n as u128) as u64;
        
        if x == 1 || x == n-1 {
            continue 'outer;
        }
        for _ in 0..r-1 {
            x = mod_exp(x as u128, 2u128, n as u128) as u64;
            
            if x == n-1 {
                 continue 'outer;
            }
        }
        return false;
    }
    true
}

// 32-bit primality test
// First checks small possible factors then switches to deterministic Miller-Rabin
pub fn is_prime32(n: u32) -> bool {

    if n <= 1 {
        return false;
    }

    // Check all primes below 62 and all witnesses
    // This quickly eliminates the vast majority of composite numbers
    let small_factors = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61];

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

    let witnesses = [2, 7, 61];
    
    'outer: for w in witnesses.iter() {
        let mut x = mod_exp(*w as u64,d as u64,n as u64) as u32;
        
        if x == 1 || x == n-1 {
            continue 'outer;
        }
        for _ in 0..r-1 {
            x = mod_exp(x as u64, 2u64, n as u64) as u32;
            
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

pub struct TotientSieve {
    sieve: HashMap::<u64,Vec<u64>>,
    n: u64,
}

impl Iterator for TotientSieve {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            self.n += 1;
            if !self.sieve.contains_key(&self.n) {
                self.sieve.insert(self.n+self.n,vec![self.n]);
                return Some(self.n-1)
            } else {
                let factors = &self.sieve[&self.n].clone();
                let mut num = 1;
                let mut den = 1;
                for factor in factors {
                    num *= factor-1;
                    den *= factor;
                    if self.sieve.contains_key(&(factor+self.n)) {
                        self.sieve.get_mut(&(factor+self.n)).unwrap().push(*factor);
                    } else {
                        self.sieve.insert(factor+self.n,vec![*factor]);
                    }
                }
                self.sieve.remove(&self.n);
                return Some((self.n*num)/den)
            }
        }
    }
}

pub fn totient_sieve() -> TotientSieve {
    TotientSieve{
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

