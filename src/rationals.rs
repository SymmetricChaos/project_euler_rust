
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Rational {
    pub n: u64,
    pub d: u64,
}

// Implemented display nicely
impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.n,self.d)
    }
}

// Implement equality
impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        let ra = reduced(self);
        let rb = reduced(other);
        ra.n == rb.n && ra.d == rb.d
    }
}

// Implement custom methods
impl Rational {
    // Instance method to reduce the fraction assuming it is mutable
    fn reduce(&mut self) {
        let g = gcd(self.n,self.d);
        self.n = self.n/g;
        self.d = self.d/g;
    }
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

pub fn reduced(r: &Rational) -> Rational {
    let x = r.n/gcd(r.n,r.d);
    let y = r.d/gcd(r.n,r.d);
    Rational{n: x, d: y}
}


