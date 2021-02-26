// Problem: What is the sum of all the minimal product-sum numbers for 2≤k≤12000?

/*
4  = 2*2 = 2+2
6  = 1*2*3 = 1+2+3
8  = 1*1*2*4 = 1+1+2+4
8  = 1*1*2*2*2 = 1+1+2+2+2
12 = 1*1*1*1*2*6 = 1+1+1+1+2+6

The 1s are quite important here because they increase the sum but not the product.
For an N-tuple we essentially need a factorization and then some 1s to fill in the rest.
*/

fn prod_sum_num(n: usize) -> u64 {
    let set = vec![1u64; n];
    let s: u64 = set.iter().sum();
    let p: u64 = set.iter().product();
    println!("{:?}",set);
    println!("{:?}",s);
    println!("{:?}",p);
    0u64
}

pub fn euler88() -> u64 {
    prod_sum_num(4)
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