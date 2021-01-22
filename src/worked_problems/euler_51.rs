// Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits) with the same digit, is part of an eight prime value family.

/*
Because we are asked for an eight digit family that means at least one of the replacements will be of either 3s, 6s, or 9s. Thus we can eliminate any canididate that is
divisble by 3 when the replacement is set to 0. Otherwise those digits would give a number that is still divisible by 3 and not prime. Also for a family of eight we have
to use at least one even number so the replacement cannot be the last digit.

Five digit numbers with two replacements
**ddd
d**dd
dd**d
*d*dd
*dd*d
d*d*d

Five digit numbers with three replacements
***dd
**d*d
*d**d
d***d

*/

use crate::aux_funcs::{is_prime};

fn should_swap(list: &Vec<u64>, index: usize, pos: usize) -> bool {
    for i in index..pos {
        if list[i] == list[pos] {
            return false
        }
    }
    true
}

fn distinct_permutations(list: &mut Vec<u64>, index: usize) -> Vec<Vec<u64>> {
    let length = list.len();
    let mut out = Vec::new();
    if index == length {
        return vec![list.clone().to_vec()]
    }
    for i in index..length {
        if should_swap(&list, index, i) {
            list.swap(i,index);
            out.extend(distinct_permutations(list,index+1));
            list.swap(i,index);
        }
    }
    out
}

pub fn euler51() -> u64 {
    let v = distinct_permutations(&mut vec![0,0,0,1,1],0);
    println!("{:?}",v);
    0u64
}

pub fn euler51_example() {
    println!("\nProblem: Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits) with the same digit, is part of an eight prime value family.");
    println!("\n\n");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler51());
}

#[test]
fn test51() {
    assert_eq!(euler51(),121313)
}