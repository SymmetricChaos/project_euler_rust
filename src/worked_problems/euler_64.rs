// Problem: How many continued fractions for the sqrt of n for n <= 10000 have an odd period?

/*
n = a*a+r
sqrt(n) = a + r/(a+sqrt(n))
*/

use num::integer::Roots;

#[derive(Debug)]
struct CFracState {
    n: u32,
    s: u32,
    a: u32,
    m: u32,
    d: u32,
}

impl Iterator for CFracState {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.m = self.d*self.a-self.m;
        self.d = (self.n-(self.m*self.m))/self.d;
        self.a = (self.s+self.m)/self.d;
        Some(self.a)
    }
}

fn make_cfrac(n: u32) -> CFracState {
    CFracState{n: n, s: n.sqrt(), a: n.sqrt(), m: 0, d: 1}
}



pub fn euler64() -> u64 {
    let mut ctr = 0;
    for n in 2..=10000 {
        let a0 = n.sqrt();
        if a0*a0 == n {
            continue
        }
        let mut c = make_cfrac(n);
        let mut v = Vec::new();
        let mut coef = vec![];
        loop {
            let a = c.next().unwrap();
            let st = format!("{:?}",c);
            if v.contains(&st) {
                break
            } else {
                v.push(st);
                coef.push(a);
            }
        }
        ctr += coef.len() % 2;
    }
    ctr as u64
}

pub fn euler64_example() {
    println!("\nProblem: How many continued fractions for the sqrt of n for n <= 10000 have an odd period?");
    println!("\n\nIt turns out that the continued fraction expansion of a sqrt is always periodic starting from the first index after the integer. So there's no need to find exactly where the period starts.");
    let s = "
use num::integer::Roots;

#[derive(Debug)]
struct CFracState {
    n: u32,
    s: u32,
    a: u32,
    m: u32,
    d: u32,
}

impl Iterator for CFracState {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.m = self.d*self.a-self.m;
        self.d = (self.n-(self.m*self.m))/self.d;
        self.a = (self.s+self.m)/self.d;
        Some(self.a)
    }
}

fn make_cfrac(n: u32) -> CFracState {
    CFracState{n: n, s: n.sqrt(), a: n.sqrt(), m: 0, d: 1}
}

pub fn euler64() -> u64 {
    let mut ctr = 0;
    for n in 2..=10000 {
        let a0 = n.sqrt();
        if a0*a0 == n {
            continue
        }
        let mut c = make_cfrac(n);
        let mut v = Vec::new();
        let mut coef = vec![];
        loop {
            let a = c.next().unwrap();
            let st = format!(\"{:?}\",c);
            if v.contains(&st) {
                break
            } else {
                v.push(st);
                coef.push(a);
            }
        }
        ctr += coef.len() % 2;
    }
    ctr as u64
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler64());
}


#[test]
fn test64() {
    assert_eq!(euler64(),1322)
}
