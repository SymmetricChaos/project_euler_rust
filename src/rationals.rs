
use std::fmt;
use std::ops::{Mul};
use num::integer::gcd;

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
        let ra = self.reduced();
        let rb = other.reduced();
        ra.n == rb.n && ra.d == rb.d
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let num = self.n * rhs.n;
        let den = self.d * rhs.d;
        Rational{n: num, d: den}
    }
}


// Implement methods
impl Rational {
    // Instance method to reduce the fraction assuming it is mutable
    pub fn reduce(&mut self) {
        let g = gcd(self.n,self.d);
        self.n = self.n/g;
        self.d = self.d/g;
    }

    pub fn reduced(&self) -> Self {
        let g = gcd(self.n,self.d);
        Rational{n: self.n/g, d: self.d/g}
    }

    /*
    pub fn invert(&mut self) {
        let t = self.n;
        self.n = self.d;
        self.d = t;
    }

    pub fn inverted(&self) -> Self {
        Rational{n: self.d, d: self.n}
    }
    */
}