
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
                    //println!("{} = {}",r,s);
                    prod = prod * s;
                }
            }

            if num_1 == den_2 {
                let s = Rational{n: num_2, d: den_1};
                if r == s {
                    //println!("{} = {}",r,s);
                    prod = prod * s;
                }
            }

            if num_2 == den_1 {
                let s = Rational{n: num_1, d: den_2};
                if r == s {
                    //println!("{} = {}",r,s);
                    prod = prod * s;
                }
            }

            if num_2 == den_2 {
                let s = Rational{n: num_1, d: den_1};
                if r == s {
                    //println!("{} = {}",r,s);
                    prod = prod * s;
                }
            }
        }
    }
    prod.reduce();
    prod.d
}

pub fn euler31_example() {
    println!("\nProblem: ");
    println!("\n\n");
    let s = "
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
                        //println!("{} = {}",r,s);
                        prod = prod * s;
                    }
                }
    
                if num_1 == den_2 {
                    let s = Rational{n: num_2, d: den_1};
                    if r == s {
                        //println!("{} = {}",r,s);
                        prod = prod * s;
                    }
                }
    
                if num_2 == den_1 {
                    let s = Rational{n: num_1, d: den_2};
                    if r == s {
                        //println!("{} = {}",r,s);
                        prod = prod * s;
                    }
                }
    
                if num_2 == den_2 {
                    let s = Rational{n: num_1, d: den_1};
                    if r == s {
                        //println!("{} = {}",r,s);
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
    println!("The answer is: {}",euler31());
}


#[test]
fn test33() {
    assert_eq!(euler33(),100)
}