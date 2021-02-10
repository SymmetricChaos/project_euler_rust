// Problem: Find the value of n ≤ 1,000,000 for which n/φ(n) is a maximum.
/*

*/

use std::collections::HashMap;

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
                let out = Some(self.n*num/den);
                self.sieve.remove(&self.n);
                return out
            }
        }
    }
}

pub fn totient_sieve() -> TotientSieve {
    TotientSieve{
        sieve: HashMap::<u64,Vec<u64>>::new(),
        n: 1u64}
}

pub fn euler69() -> u64 {
    let mut tot = totient_sieve();
    let mut biggest = f64::from(3);
    let mut record = 10;
    for n in 2..=1_000_000 {
        let num: f64 = n as f64;
        let den: f64 = tot.next().unwrap() as f64;
        if num / den > biggest {
            biggest = num / den;
            record = n;
        }
    }
    record
}

pub fn euler69_example() {
    println!("\nProblem: Find the value of n ≤ 1,000,000 for which n/φ(n) is a maximum.");
    println!("\n\nThe sieving method we've used previously for primes also generates totients in the process.");
    let s = "
use std::collections::HashMap;

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
                let out = Some(self.n*num/den);
                self.sieve.remove(&self.n);
                return out
            }
        }
    }
}

pub fn totient_sieve() -> TotientSieve {
    TotientSieve{
        sieve: HashMap::<u64,Vec<u64>>::new(),
        n: 1u64}
}

pub fn euler69() -> u64 {
    let mut tot = totient_sieve();
    let mut biggest = f64::from(3);
    let mut record = 10;
    for n in 2..=1_000_000 {
        let num: f64 = n as f64;
        let den: f64 = tot.next().unwrap() as f64;
        if num / den > biggest {
            biggest = num / den;
            record = n;
        }
    }
    record
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler69());
}


#[test]
fn test69() {
    assert_eq!(euler69(),510510)
}