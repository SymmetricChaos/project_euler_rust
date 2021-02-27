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
*/

use std::collections::HashSet;

// This brings the position up to its maximum before the next number changes and never reduces it
// We want to get results in the order:
/*
1,1,1,1
1,1,1,2
1,1,2,2
1,1,2,3


*/
// We can bound our search in any situation where the sum becomes less than the product
// Probably have to do it recursively?
fn prod_sum_num(k: usize) -> u64 {
    for i in 2.. {
        let mut set = vec![1u64; k];
        set[0] = i;
        for pos in 1..k {
            if set[pos] < set[pos-1] {
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

// Too slow
fn prod_sum_num_recur(pos: usize, set: &mut Vec<u64>) -> Option<u64> {
    let s: u64 = set.iter().sum();
    let p: u64 = set.iter().product();
    if pos == set.len()-1 {
        return None
    }
    if s < p {
        return None
    } else if s == p {
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

pub fn euler88() -> u64 {

    let mut nums = HashSet::new();
    nums.insert(4);
    nums.insert(6);
    'outer: for k in 4..=12000 {
        for i in 2.. {
            let mut set = vec![1u64; k];
            set[0] = i;
            let num = prod_sum_num_recur(1, &mut set);
            if matches!(num,Some(_)) {
                //println!("{}",num.unwrap());
                nums.insert(num.unwrap());
                continue 'outer
            }
        }
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