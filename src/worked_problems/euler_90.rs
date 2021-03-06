// Problem: How many distinct arrangements of the two dice allow for all of the square numbers to be displayed?

/*
Important: USING THE DICE IN ANY ORDER
The number of possible six sided dice using the digits 0-9 without repetition is 210
So there can't be more than 210*210 = 44100 possibilities
Only half of those are distinct so really just 22050
That's possible to do by brute force
*/

use std::collections::HashSet;
use itertools::Itertools;

// Needs to allow for the 6/9 flip
fn check_dice(d1: &Vec<u8>, d2: &Vec<u8>, set: &HashSet<u8>) -> bool {
    let mut new_set = HashSet::<u8>::new();
    let mut d1 = d1.clone();
    let mut d2 = d2.clone();
    if d1.contains(&6) {
        d1.push(9)
    } else if d1.contains(&9) {
        d1.push(6)
    }
    if d2.contains(&6) {
        d2.push(9)
    } else if d2.contains(&9) {
        d2.push(6)
    }

    for a in d1.iter() {
        for b in d2.iter() {
            let x = 10*a + b;
            let y = 10*b + a;
            if set.contains(&x) {
                new_set.insert(x);
            }
            if set.contains(&y) {
                new_set.insert(y);
            }
        }
        if new_set.len() == set.len() {
            return true
        }
    }
    false
}

pub fn euler90() -> u64 {
    let set: HashSet::<u8> = [01,04,09,16,25,36,49,64,81].iter().cloned().collect();
    let mut known = HashSet::new();
    let mut ctr = 0;
    let comb1 = (0..=9).combinations(6);
    let comb2: Vec<Vec<u8>> = (0..=9).combinations(6).into_iter().collect();
    for a in comb1 {   
        // Can never be two of the same die because it would need to have more than six digits on it
        // So once we generate an example for the first die we're never interested in seeing it for the second
        known.insert(format!("{:?}",a));
        for b in comb2.iter() {
            if known.contains(&format!("{:?}",b)) {
                continue
            }
            if check_dice(&a,&b,&set) {
                ctr += 1
            }
        }
        
    }
    ctr
}


pub fn euler90_example() {
    println!("\nProblem: How many distinct arrangements of the two dice allow for all of the square numbers to be displayed?");
    println!("\n\nI made a whole lot of dumb mistakes on this one but the solution is straightforward. Generating all the combinations and checking them is easy as there are only 210 per die. Then some code to validate them and make sure we ignore repeats.");
    let s = "
use std::collections::HashSet;
use itertools::Itertools;

fn check_dice(d1: &Vec<u8>, d2: &Vec<u8>, set: &HashSet<u8>) -> bool {
    let mut new_set = HashSet::<u8>::new();

    // Allow for flipping 6 and 9 here
    let mut d1 = d1.clone();
    let mut d2 = d2.clone();
    if d1.contains(&6) {
        d1.push(9)
    } else if d1.contains(&9) {
        d1.push(6)
    }
    if d2.contains(&6) {
        d2.push(9)
    } else if d2.contains(&9) {
        d2.push(6)
    }

    for a in d1.iter() {
        for b in d2.iter() {
            let x = 10*a + b;
            let y = 10*b + a;
            if set.contains(&x) {
                new_set.insert(x);
            }
            if set.contains(&y) {
                new_set.insert(y);
            }
        }
        if new_set.len() == set.len() {
            return true
        }
    }
    false
}

pub fn euler90() -> u64 {
    let set: HashSet::<u8> = [01,04,09,16,25,36,49,64,81].iter().cloned().collect();
    let mut known = HashSet::new();
    let mut ctr = 0;
    let comb1 = (0..=9).combinations(6);
    let comb2: Vec<Vec<u8>> = (0..=9).combinations(6).into_iter().collect();
    for a in comb1 {   
        // Can never be two of the same die because it would need to have more than six digits on it
        // So once we generate an example for the first die we're never interested in seeing it for the second
        known.insert(format!(\"{:?}\",a));
        for b in comb2.iter() {
            if known.contains(&format!(\"{:?}\",b)) {
                continue
            }
            if check_dice(&a,&b,&set) {
                ctr += 1
            }
        }
        
    }
    ctr
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler90());
}


#[test]
fn test90() {
    assert_eq!(euler90(),1217)
}
