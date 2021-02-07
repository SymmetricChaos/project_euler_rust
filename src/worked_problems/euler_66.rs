// Problem: For x^2-D*y^2 = 1 find the value of D ≤ 1000 in minimal solutions of x for which the largest value of x is obtained.
/*

*/

use num::traits::{Zero,One};
use num::bigint::BigUint;
use num::integer::{gcd,Roots};

#[derive(Debug)]
struct CFracState {
    n: BigUint,
    s: BigUint,
    a: BigUint,
    m: BigUint,
    d: BigUint,
    n0: BigUint,
    d0: BigUint,
    n1: BigUint,
    d1: BigUint,
}

impl Iterator for CFracState {
    type Item = (BigUint,BigUint);

    fn next(&mut self) -> Option<(BigUint,BigUint)> {

        let nt = self.n1.clone();
        self.n1 = &self.a * &self.n1 + &self.n0;
        self.n0 = nt;

        let dt = self.d1.clone();
        self.d1 = &self.a * &self.d1 + &self.d0;
        self.d0 = dt;

        let g = gcd(self.n1.clone(),self.d1.clone());
        self.n1 = &self.n1/&g;
        self.d1 = &self.d1/&g;

        let out = (self.n1.clone(),self.d1.clone());
        
        self.m = &self.d*&self.a-&self.m;
        self.d = (&self.n-(&self.m*&self.m))/&self.d;
        self.a = (&self.s+&self.m)/&self.d;

        Some(out)
    }
}

fn make_cfrac(n: u64) -> CFracState {
    CFracState{n: BigUint::from(n), s: BigUint::from(n.sqrt()), 
               a: BigUint::from(n.sqrt()), m: BigUint::zero(), 
               d: BigUint::one(), n0: BigUint::zero(), d0: BigUint::one(), 
               n1: BigUint::one(), d1: BigUint::zero()}
}

pub fn euler66() -> u64 {
    let mut biggest = BigUint::zero();
    let mut out = 0;
    for n in 2..1000 {
        if n.sqrt() * n.sqrt() == n {
            continue
        }
        let mut cfrac = make_cfrac(n);
        loop {
            let tup = cfrac.next().unwrap();
            if &n * &tup.1 * &tup.1 < &tup.0 * &tup.0 {
                if &tup.0 * &tup.0 - &n * &tup.1 * &tup.1 == BigUint::one() {
                    if &tup.0 > &biggest {
                        biggest = tup.0;
                        out = n;
                    }
                    break
                }
            }
        }
    }
    out
}

pub fn euler66_example() {
    println!("\nProblem: For x^2-D*y^2 = 1 find the value of D ≤ 1000 in minimal solutions of x for which the largest value of x is obtained. Consider only positive integer solutions.");
    println!("\n\nThis question is about Pell's equation (though it would be better to call it Brahmagupta's equation). It is no coincidence that this equation comes after two that explicitly ask about continued fractions. Minimual solution can be found by approaching this in terms of continued fractions.");
    let s = "
use num::traits::{Zero,One};
use num::bigint::BigUint;
use num::integer::{gcd,Roots};

#[derive(Debug)]
struct CFracState {
    n: BigUint,
    s: BigUint,
    a: BigUint,
    m: BigUint,
    d: BigUint,
    n0: BigUint,
    d0: BigUint,
    n1: BigUint,
    d1: BigUint,
}

impl Iterator for CFracState {
    type Item = (BigUint,BigUint);

    fn next(&mut self) -> Option<(BigUint,BigUint)> {

        let nt = self.n1.clone();
        self.n1 = &self.a * &self.n1 + &self.n0;
        self.n0 = nt;

        let dt = self.d1.clone();
        self.d1 = &self.a * &self.d1 + &self.d0;
        self.d0 = dt;

        let g = gcd(self.n1.clone(),self.d1.clone());
        self.n1 = &self.n1/&g;
        self.d1 = &self.d1/&g;

        let out = (self.n1.clone(),self.d1.clone());
        
        self.m = &self.d*&self.a-&self.m;
        self.d = (&self.n-(&self.m*&self.m))/&self.d;
        self.a = (&self.s+&self.m)/&self.d;

        Some(out)
    }
}

fn make_cfrac(n: u64) -> CFracState {
    CFracState{n: BigUint::from(n), s: BigUint::from(n.sqrt()), 
                a: BigUint::from(n.sqrt()), m: BigUint::zero(), 
                d: BigUint::one(), n0: BigUint::zero(), d0: BigUint::one(), 
                n1: BigUint::one(), d1: BigUint::zero()}
}

pub fn euler66() -> u64 {
    let mut biggest = BigUint::zero();
    let mut out = 0;
    for n in 2..1000 {
        if n.sqrt() * n.sqrt() == n {
            continue
        }
        let mut cfrac = make_cfrac(n);
        loop {
            let tup = cfrac.next().unwrap();
            if &n * &tup.1 * &tup.1 < &tup.0 * &tup.0 {
                if &tup.0 * &tup.0 - &n * &tup.1 * &tup.1 == BigUint::one() {
                    if &tup.0 > &biggest {
                        biggest = tup.0;
                        out = n;
                    }
                    break
                }
            }
        }
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler66());
}


#[test]
fn test66() {
    assert_eq!(euler66(),661)
}
