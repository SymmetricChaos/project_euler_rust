// Problem: Considering natural numbers of the form, a^b, where a, b < 100, what is the maximum digital sum?

/*

*/


use crate::aux_funcs::{int_to_digits,digit_addition,digit_multiplication};



pub fn euler56() -> u64 {
    let v1 = int_to_digits(99,10);
    let v2 = int_to_digits(12,10);
    let v3 = digit_multiplication(&v1,&v2,10);
    println!("{:?} * {:?} = {:?}",v1,v2,v3);

    0u64
}

pub fn euler56_example() {
    println!("\nProblem: Considering natural numbers of the form, a^b, where a, b < 100, what is the maximum digital sum?");
    println!("\n\n");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler56());
}

#[test]
fn test56() {
    assert_eq!(euler56(),972)
}