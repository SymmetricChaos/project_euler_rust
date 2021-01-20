
// There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two digits in the numerator and denominator.
// If the product of these four fractions is given in its lowest common terms, find the value of the denominator.

use crate::rationals::Rational;

pub fn euler33() -> u64 {
    let mut prod = Rational{n: 1, d: 1};

    for n in 10..100 {
        for d in n+1..100 {
            let r = Rational{n: n, d: d};
            let num_1 = n/10;
            let num_2 = n%10;
            let den_1 = d/10;
            let den_2 = d%10;

            if den_2 == 0 {
                continue
            }

            if num_1 == den_1 {
                let s = Rational{n: num_2, d: den_2};
                if r == s {
                    prod = prod * s;
                }
            }

            if num_1 == den_2 {
                let s = Rational{n: num_2, d: den_1};
                if r == s {
                    prod = prod * s;
                }
            }

            if num_2 == den_1 {
                let s = Rational{n: num_1, d: den_2};
                if r == s {
                    prod = prod * s;
                }
            }

            if num_2 == den_2 {
                let s = Rational{n: num_1, d: den_1};
                if r == s {
                    prod = prod * s;
                }
            }
        }
    }
    prod.reduce();
    prod.d
}

pub fn euler33_example() {
    println!("\nProblem: There are exactly four non-trivial examples of a fraction that appears to be simplified by \"cancelling\" digits, less than one in value, and containing two digits in the numerator and denominator. If the product of these four fractions is given in its lowest common terms, find the value of the denominator.");
    println!("\n\nThis one ends up involving rather a lot of preamble. Althought it could be solved just using a Greatest Common Divisor function it is significantly neater to implement a Rational type to handle the fractions. This is also an interesting exercise in and of itself. All we need is a notion of equality, the ability to reduce fractions, and the ability to multiply them. With that Rational type created it is straightforward (if verbose) to check the necessary properties of the fractions.");
    let s = "

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

#[derive(Debug, Copy, Clone)]
pub struct Rational {
    pub n: u64,
    pub d: u64,
}

// Implement Equality
impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        let ra = self.reduced();
        let rb = other.reduced();
        ra.n == rb.n && ra.d == rb.d
    }
}

// Implement multiplication
impl Mul for Rational {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let num = self.n * rhs.n;
        let den = self.d * rhs.d;
        Rational{n: num, d: den}
    }
}

// Implement methods to reduce fractions and to output a reduced fraction
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
}

pub fn euler33() -> u64 {
    let mut prod = Rational{n: 1, d: 1};

    for n in 10..100 {
        for d in n+1..100 {
            let r = Rational{n: n, d: d};
            let num_1 = n/10;
            let num_2 = n%10;
            let den_1 = d/10;
            let den_2 = d%10;

            if den_2 == 0 {
                continue
            }

            if num_1 == den_1 {
                let s = Rational{n: num_2, d: den_2};
                if r == s {
                    prod = prod * s;
                }
            }

            if num_1 == den_2 {
                let s = Rational{n: num_2, d: den_1};
                if r == s {
                    prod = prod * s;
                }
            }

            if num_2 == den_1 {
                let s = Rational{n: num_1, d: den_2};
                if r == s {
                    prod = prod * s;
                }
            }

            if num_2 == den_2 {
                let s = Rational{n: num_1, d: den_1};
                if r == s {
                    prod = prod * s;
                }
            }
        }
    }
    prod.reduce();
    prod.d
}
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler33());
}


#[test]
fn test33() {
    assert_eq!(euler33(),100)
}