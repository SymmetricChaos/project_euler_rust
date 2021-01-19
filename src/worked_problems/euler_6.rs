// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

pub fn euler6() -> u64 {
    let mut sum = 0;
    let mut sq_sum = 0;
    for n in 1..101 {
        sum += n;
        sq_sum += n*n;
    }
    (sum*sum)-sq_sum
}

pub fn euler6_example() -> u64 {
    println!("\nProblem: Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum of the first one hundred natural numbers.");
    println!("\n\nThis one is easy to solve directly. Nothing clever involved.");
    let s = "
pub fn euler6() -> u64 {
    let mut sum = 0;
    let mut sq_sum = 0;
    for n in 1..101 {
        sum += n;
        sq_sum += n*n;
    }
    (sum*sum)-sq_sum
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler6());
    0u64
}

#[test]
fn test6() {
    assert_eq!(euler6(),25164150)
}
