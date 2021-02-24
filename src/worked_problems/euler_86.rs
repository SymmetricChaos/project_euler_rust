// Problem: Find the least value of M such that for a the number of cuboids with dimensions up to MxMxM (ignoring rotations) in which the shortest route from corner to opposite corner has integer length first exceeds one million.

/*
Consider the example provided
6x5x3 -> 10
We are told "there are up to three shortest path candidates for any given cuboid"

Considering the net of a cube it can be seen that these candidates are always of the form: 
sqrt(x^2 + (y+z)^2)
This is obviously an integer only when x^2 + (y+z)^2 is a perfect square

If the cuboid is AxBxC then the candidates are
sqrt(a^2 + (b+c)^2)
sqrt(b^2 + (a+c)^2)
sqrt(c^2 + (a+b)^2)
*/

use num::integer::Roots;

fn is_square(n: u64) -> bool {
    n.sqrt()*n.sqrt() == n
}

pub fn euler86() -> u64 {
    let mut ctr = 0;
    for a in 1u64.. {
        //println!("{} -> {}",a-1,ctr);
        if ctr > 1_000_000 {
            return a-1
        }
        for b in 1..=a {
            for c in 1..=b {
                let paths = [a*a + (b+c)*(b+c),
                             b*b + (a+c)*(a+c),
                             c*c + (a+b)*(a+b)];
                let best = paths.iter().min().unwrap();
                if is_square(*best) {
                    ctr += 1;
                }
            }
        }
    }
    unreachable!()
}

pub fn euler86_example() {
    println!("\nProblem: Find the least value of M such that for a the number of cuboids with dimensions up to MxMxM (ignoring rotations) in which the shortest route from corner to opposite corner has integer length first exceeds one million.");
    println!("\n\nConsidering the net of a cube it can be seen that these candidates are always of the form: \"sqrt(x^2 + (y+z)^2)\". This is obviously an integer only when \"x^2 + (y+z)^2\" is a perfect square. It is then straighforward to check all these paths.");
    let s = "
use num::integer::Roots;

fn is_square(n: u64) -> bool {
    n.sqrt()*n.sqrt() == n
}

pub fn euler86() -> u64 {
    let mut ctr = 0;
    for a in 1u64.. {
        if ctr > 1_000_000 {
            return a-1
        }
        for b in 1..=a {
            for c in 1..=b {
                let paths = [a*a + (b+c)*(b+c),
                                b*b + (a+c)*(a+c),
                                c*c + (a+b)*(a+b)];
                let best = paths.iter().min().unwrap();
                if is_square(*best) {
                    ctr += 1;
                }
            }
        }
    }
    unreachable!()
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler86());
}


#[test]
fn test86() {
    assert_eq!(euler86(),1818)
}
