// Problem: Given that L is the perimeter, for how many values of L ≤ 1,500,000 can exactly one integer sided right angle triangle be formed?

/*
*/

use num::integer::gcd;
use std::collections::HashMap;

pub fn euler75() -> u64 {
    let mut map = HashMap::<u32,u8>::new();
    let mut out = 0;
    // sqrt(1,500,000) is approximately 1225
    for m in 2..=1225 {
        for n in 1..m {
            if gcd(m,n) != 1 || (m-n)%2 == 0 {
                continue
            }
            let a = m*m-n*n;
            let b = 2*m*n;
            let c = m*m+n*n;
            let peri = a+b+c;
            let mut p = peri;
            // Loop through all members of the family and if they're new set them to 1, otherwise set to zero
            loop {
                if p > 1_500_000 {
                    break
                }
                if map.contains_key(&p) {
                    *map.get_mut(&p).unwrap() = 0;
                } else {
                    map.insert(p,1);
                }
                p += peri;
            }
        }
    }
    for (_,v) in map.iter() {
        out += *v as u64
    }
    out
}


pub fn euler75_example() {
    println!("\nProblem: Given that L is the perimeter, for how many values of L ≤ 1,500,000 can exactly one integer sided right angle triangle be formed?");
    println!("\n\nEuclid provides a well known formula for generating pythagorean triples.");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler75());
}


#[test]
fn test75() {
    assert_eq!(euler75(),161667)
}