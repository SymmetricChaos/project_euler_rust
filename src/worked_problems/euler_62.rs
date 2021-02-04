// Problem: Find the smallest cube for which exactly five permutations of its digits are cubes.
/*
Iterating through permutations of cubes is not feasible in this case. However we don't need to do that.
Instead we can hash known cubes based on their digits and see if those digits have shown up before
*/

use std::collections::HashMap;
use crate::aux_funcs::{int_to_digits};

pub fn euler62() -> u64 {
    // Start with five because the cube must have at least three digits
    let mut n = 5;
    let mut known_cubes: HashMap<String,Vec<u64>> = HashMap::new();
    loop {
        let cube = n*n*n;
        let mut digits = int_to_digits(cube,10);
        digits.sort();
        let s = format!("{:?}",digits);
        if known_cubes.contains_key(&s) {
            known_cubes.get_mut(&s).unwrap().push(cube);
            if known_cubes[&s].len() == 5 {
                return known_cubes[&s][0]
            }
        } else {
            known_cubes.insert(s.clone(),vec![cube]);
        }
        n += 1
    }
}

pub fn euler62_example() {
    println!("\nProblem: Find the smallest cube for which exactly five permutations of its digits are cubes.");
    println!("\n\nIterating through permutations of cubes is not feasible in this case. However we don't need to do that. Instead we can hash known cubes based on their digits and see if those digits have shown up before
    ");
    let s = "
use std::collections::HashMap;
use crate::aux_funcs::{int_to_digits};

pub fn euler62() -> u64 {
    // Start with five because the cube must have at least three digits
    let mut n = 5;
    let mut known_cubes: HashMap<String,Vec<u64>> = HashMap::new();
    loop {
        let cube = n*n*n;
        let mut digits = int_to_digits(cube,10);
        digits.sort();
        let s = format!(\"{:?}\",digits);
        if known_cubes.contains_key(&s) {
            known_cubes.get_mut(&s).unwrap().push(cube);
            if known_cubes[&s].len() == 5 {
                return known_cubes[&s][0]
            }
        } else {
            known_cubes.insert(s.clone(),vec![cube]);
        }
        n += 1
    }
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler62());
}


#[test]
fn test62() {
    assert_eq!(euler62(),127035954683)
}
