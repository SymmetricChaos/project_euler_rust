// Problem: For x^2-D*y^2 = 1 find the value of D ≤ 1000 in minimal solutions of x for which the largest value of x is obtained.
/*

*/

use num::traits::{Zero,One};
use num::bigint::BigUint;
use num::integer::{gcd,Roots};

#[derive(Debug)]
struct CFracState {
    n: u64,
    s: u64,
    a: u64,
    m: u64,
    d: u64,
    n0: u64,
    d0: u64,
    n1: u64,
    d1: u64,
}

impl Iterator for CFracState {
    type Item = (u64,u64);

    fn next(&mut self) -> Option<(u64,u64)> {

        let nt = self.n1;
        self.n1 = self.a * self.n1 + self.n0;
        self.n0 = nt;

        let dt = self.d1;
        self.d1 = self.a * self.d1 + self.d0;
        self.d0 = dt;

        let g = gcd(self.n1,self.d1);
        self.n1 = self.n1/g;
        self.d1 = self.d1/g;

        let out = (self.n1,self.d1);
        
        self.m = self.d*self.a-self.m;
        self.d = (self.n-(self.m*self.m))/self.d;
        self.a = (self.s+self.m)/self.d;

        Some(out)
    }
}

fn make_cfrac(n: u64) -> CFracState {
    CFracState{n: n, s: n.sqrt(), a: n.sqrt(), m: 0, d: 1, n0: 0, d0: 1, n1: 1, d1: 0}
}

pub fn euler66() -> u64 {
    let mut biggest = 0;
    let mut out = 0;
    for n in 2..1000 {
        if n.sqrt() * n.sqrt() == n {
            continue
        }
        println!("{}",n);
        let mut cfrac = make_cfrac(n);
        loop {
            let tup = cfrac.next().unwrap();
            if n * tup.1 * tup.1 < tup.0 * tup.0 {
                if tup.0 * tup.0 - n * tup.1 * tup.1 == 1 {
                    if tup.0 > biggest {
                        biggest = tup.0;
                        out = n;
                    }
                    println!("{}^2 - {}*{}^2 = 1",tup.0,n,tup.1);
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
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler66());
}


#[test]
fn test66() {
    assert_eq!(euler66(),661)
}
