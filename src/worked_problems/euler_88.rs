// Problem: What is the sum of all the minimal product-sum numbers for 2≤k≤12000?

/*
4  = 2*2 = 2+2
6  = 1*2*3 = 1+2+3
8  = 1*1*2*4 = 1+1+2+4
8  = 1*1*2*2*2 = 1+1+2+2+2
12 = 1*1*1*1*2*6 = 1+1+1+1+2+6

The 1s are quite important here because they increase the sum but not the product.
For an k-tuple we essentially need a factorization and then some 1s to fill in the rest.
Primes can never qualify.
It is not feasible to calculate every factorization for numbers until we reach k = 12000.
What if we try to build the factorization from the tuple?

If the product of a set is greater than the sum then its not possible to get a product sum number by increasing any of the elements

Okay I looked up some hints and they note that for each k the lower limit of the minimal number is k and the upper limit is 2k.
https://www.mathblog.dk/project-euler-88-minimal-product-sum-numbers/
To make progress on this I'll have to look more closely as the explanation of a solution given here. It seems similar to mine but avoids recomputation.

Any set of factors can be turned into a product sum by appending product-sum 1s to it. 
*/

use std::collections::HashSet;

/*
// Much too slow, have to get rid of cloning and just mutate the vector
fn prod_sum_num_recur(pos: usize, set: &mut Vec<u64>) -> Option<u64> {
    let s: u64 = set.iter().sum();
    let p: u64 = set.iter().product();
    if pos == set.len()-1 || s < p {
        return None
    } else if s == p {
        println!("{:?}",set);
        return Some(s)
    } else {
        let mut vecs = Vec::<Option<u64>>::new();
        for i in 2..=set[pos-1] {
            set[pos] = i;
            vecs.push(prod_sum_num_recur(pos+1,&mut set.clone()))
        }
        let mut out = u64::MAX;
        for v in vecs {
            if matches!(v,Some(_)) {
                if v.unwrap() < out {
                    out = v.unwrap()
                }
            }
        }
        if out != u64::MAX {
            return Some(out)
        }
    }
    None
}
*/


// Iterative version with no cloning
fn prod_sum_num(k: usize) -> u64 {
    
    let mut set = vec![1u64;k];

    let mut pos: usize = 0;

    loop {
        // This has to be the bottleneck
        // How can we avoid adding and multiplying thousands of numbers?
        // Some way to bootstrap from previous?
        let p: u64 = set.iter().product();
        let s: u64 = set.iter().sum();
        
        if p > s {
            set[pos] = 1;
            pos -=1
        }
        if s == p {
            return s
        }

        if pos == 0 {
            // If we've backtracked to the root then increase by 1 and go forward
            set[pos] += 1;
            pos += 1;
        } else if set[pos] == set[pos-1] {
            // If we've already increased this position to match the previous set it to 1 and go back
            set[pos] = 1;
            pos -= 1;
        } else {
            // In all other cases increase the position by 1 and go forward
            set[pos] += 1;
            pos += 1;
        }
    }
}


// Can't get this attempt to recreate the C# code to work
fn prod_sum_once() -> [u64;12000] {
    // Array of candidates initialized with 24000, the greatest possible product sum number in the range
    let mut candidates = [24000u64;12000];
    let mut set = vec![2u64,1u64];
    candidates[0] = 0;
    candidates[1] = 0;
    let mut pos: usize = 0;
    
    loop {
        if pos == 0 {
            // Can't need more than 14 factors
            if set.len() == 15 {
                break
            }
            if set[0] < set[1] {
                set[0] += 1
            } else {
                set.push(u64::MAX);
                set[0] = 2
            }
            pos += 1;
            set[1] = set[0]-1
        } else if pos == set.len()-1 {
            println!("{:?}",set);
            set[pos] += 1;
            let p: u64 = set.iter().product();
            let s: u64 = set.iter().sum();
            if p > 24000 {
                pos -= 1
            } else {
                let k = p-s+(set.len() as u64);
                if k < 12000 {
                    if p < candidates[k as usize] {
                        candidates[k as usize] = p;
                    }
                }
            }
        } else if set[pos] < set[pos+1] {
            set[pos] += 1;
            set[pos+1] = set[pos]-1;
            pos += 1
        } else if set[pos] >= set[pos+1] {
            pos -= 1 
        }
    }
    
    candidates
}


// Generate pairs with product less than 24000
fn pairs() -> Vec<(u64,u64)> {

    let mut out = vec![];
    let mut a = 2;
    loop {
        if a > 154 {
            break
        }
        for b in a..(24000/a) {
            out.push((a,b))
        }
        a += 1
    }
    out
}


pub fn euler88() -> u64 {

    let nums = pairs();
    println!("{:?}",nums);
    /*
    let mut nums = HashSet::new();
    nums.insert(4);
    nums.insert(6);
    for k in 4..=12000 {
        println!("{}",k);
        let num = prod_sum_num(k);
        nums.insert(num);
    }
    let out: u64 = nums.iter().sum();
    out
    */
    0u64
}

pub fn euler88_example() {
    println!("\nProblem: What is the sum of all the minimal product-sum numbers for 2≤k≤12000?");
    println!("\n\nWe can set an upper limit on the value of such a minimal number for a set of k number as 2k because the set {{2,k}} with k-2 ones always works so we never need any number greater than 24000. Likewise because 2^15 > 24000 we know every product of more than 14 integers greater than 1 is too large for the previous limit.");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler88());
}

/*
#[test]
fn test88() {
    assert_eq!(euler88(),)
}
*/