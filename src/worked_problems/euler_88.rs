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
It is not feasible to calculate every factorization for numbers unless we reach k = 12000.
What if we try to build the factorization from the tuple?
*/

use std::collections::HashSet;

fn prod_sum_num(k: usize) -> u64 {
    for i in 2.. {
        let mut set = vec![1u64; k];
        set[0] = i;
        for pos in 1..k {
            while set[pos] < set[pos-1] {
                set[pos] += 1;
                let s: u64 = set.iter().sum();
                let p: u64 = set.iter().product();
                if s == p {
                    println!("{:?}",set);
                    return s
                }
            }
        }
    }
    unreachable!()
}

pub fn euler88() -> u64 {
    let mut nums = HashSet::new();
    for k in 2..=6 {
        nums.insert(prod_sum_num(k));
    }
    let out: u64 = nums.iter().sum();
    out
}

pub fn euler88_example() {
    println!("\nProblem: What is the sum of all the minimal product-sum numbers for 2≤k≤12000?");
    println!("\n\n");
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