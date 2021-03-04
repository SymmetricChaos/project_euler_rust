// What is the 10 001st prime number?

use crate::aux_funcs::{prime_sieve};

pub fn euler7() -> u64 {
    let pset = prime_sieve();
    let out = pset.skip(10_000).next().unwrap();
    out
}

pub fn euler7_example() {
    println!("\nWhat is the 10 001st prime number?");
    println!("\nTo solve this a Sieve of Eratosthenes is used.");
    let s = "
struct PrimeSieve {
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

fn prime_sieve() -> PrimeSieve {
    PrimeSieve{
        sieve: HashMap::<u64,Vec<u64>>::new(),
        n: 1u64}
}

pub fn euler7() -> u64 {
    let pset = prime_sieve();
    let out = pset.skip(10_000).next().unwrap();
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler7());
}

#[test]
fn test07() {
    assert_eq!(euler7(),104743)
}
