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

/*
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
*/


// We want to do the equivalent of producing pairs then triplets and so on, growing the set as needed
fn prod_sum_num1() -> [u64;12001] {
    
    // Array of possible values set to the greatest value allowed
    // We ignore k = 0 and k = 1 since we only need to know from 2 to 12000
    let mut candidates = [24000u64;12001];
    candidates[0] = 0;
    candidates[1] = 0;

    // The list of factors we are working with
    let mut list = vec![1u64,24000u64];

    let mut pos: usize = 0;

    loop {
        // If we pass the determined limit end
        if list.len() == 15 {
            break
        }
        // The logic for producing the sets of factors
        // The example makes the root always the smallest and the end the biggest

        // At the root
        if pos == 0 {

            // If the root can get bigger do so
            if list[0] < list[1] {
                list[0] += 1;
            
            // Otherwise reset the root to 2 and append some large number to the end
            } else {
                list[0] = 2;
                list.push(24000)
            }

            // Either way move forward and set position 1 to be one less than the root
            pos += 1;
            list[1] = list[0]-1
        
            // At the last position
            } else if pos == list.len() - 1 {

                list[pos] += 1;
                // Current product and sum
                let p: u64 = list.iter().product();
                let s: u64 = list.iter().sum();

                // If the sequence is disallowed go back
                if p > 24000 {
                    pos -= 1
                } else {
                    // Otherwise find the k-tuple for which, by appending 1s, both the product and sum equal p
                    let k = p-s+(list.len() as u64);
                    // If we can assign to that position do so
                    if k <= 12000 {
                        if p < candidates[k as usize] {
                            candidates[k as usize] = p
                        }
                    }
                }

        // In every other position
        } else {
            // If the sequence is increasing left to right, then increment, reset the next factor, and move forward
            if list[pos] < list[pos+1] {
                list[pos] += 1;
                list[pos+1] = list[pos]-1;
                pos += 1;

            // Otherwise we have an already searched sequence and need to go back
            } else {
                pos -= 1
            }
        }
    }
    candidates
}



pub fn euler88() -> u64 {
    let mut out = 0;
    let c = prod_sum_num1();
    let mut h = HashSet::new();
    for n in c.iter() {
        if !h.contains(&n) {
            out += n;
            h.insert(n);
        }
    }
    out
}

pub fn euler88_example() {
    println!("\nProblem: What is the sum of all the minimal product-sum numbers for 2≤k≤12000?");
    println!("\n\nCan't claim to have solved this one myself but I did come up with a technique similar to this, though mine tried to recompute everything for each number which is infeasibly slow. We can set an upper limit on the value of such a minimal number for a set of k number as 2k because the set {{2,k}} with k-2 ones always works so we never need any number greater than 24000. In general for a list of numbers with product \'p\' and sum \'s\' the product and sum are equal when p-s 1s are appended. Likewise because 2^15 > 24000 we know every product of more than 14 integers greater than 1 is too large for the previous limit. Knowing this it is possible to generate all 2-tuples with a product less than 24000 then all 3-tuples, and so on up to 14-tuples.");
    let s = "
// We want to do the equivalent of producing pairs then triplets and so on, growing the set as needed
fn prod_sum_num1() -> [u64;12001] {
    
    // Array of possible values set to the greatest value allowed
    // We ignore k = 0 and k = 1 since we only need to know from 2 to 12000
    let mut candidates = [24000u64;12001];
    candidates[0] = 0;
    candidates[1] = 0;

    // The list of factors we are working with
    // We start with (1,24000) so that the logic below will make the first step (2,1) which is where we want to begin
    let mut list = vec![1u64,24000u64];

    let mut pos: usize = 0;

    loop {
        // If we pass the determined limit then end
        if list.len() == 15 {
            break
        }
        // The logic for producing the sets of factors
        // The example makes the root always the smallest and the end the biggest

        // At the root
        if pos == 0 {

            // If the root can get bigger do so
            if list[0] < list[1] {
                list[0] += 1;
            
            // Otherwise reset the root to 2 and append some large number to the end
            } else {
                list[0] = 2;
                list.push(24000)
            }

            // Either way move forward and set position 1 to be one less than the root
            pos += 1;
            list[1] = list[0]-1
        
            // At the last position
            } else if pos == list.len() - 1 {

                list[pos] += 1;
                // Current product and sum
                let p: u64 = list.iter().product();
                let s: u64 = list.iter().sum();

                // If the sequence is disallowed go back
                if p > 24000 {
                    pos -= 1
                } else {
                    // Otherwise find the k-tuple for which, by appending 1s, both the product and sum equal p
                    let k = p-s+(list.len() as u64);
                    // If we can assign to that position do so
                    if k <= 12000 {
                        if p < candidates[k as usize] {
                            candidates[k as usize] = p
                        }
                    }
                }

        // In every other position
        } else {
            // If the sequence is increasing left to right, then increment, reset the next factor, and move forward
            if list[pos] < list[pos+1] {
                list[pos] += 1;
                list[pos+1] = list[pos]-1;
                pos += 1;

            // Otherwise we have an already searched sequence and need to go back
            } else {
                pos -= 1
            }
        }
    }
    candidates
}

pub fn euler88() -> u64 {
    let mut out = 0;
    let c = prod_sum_num1();
    let mut h = HashSet::new();
    for n in c.iter() {
        if !h.contains(&n) {
            out += n;
            h.insert(n);
        }
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler88());
}


#[test]
fn test88() {
    assert_eq!(euler88(),7587457)
}
