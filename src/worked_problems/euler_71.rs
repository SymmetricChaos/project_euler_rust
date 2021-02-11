// Problem: By listing the set of reduced proper fractions for d ≤ 1,000,000 in ascending order of size, find the numerator of the fraction immediately to the left of 3/7.

/*
*/

pub fn euler71() -> u64 {
    let mut lower = (2,5);
    let upper = (3,7);
    loop {
        if lower.1 + upper.1 > 1_000_000 {
            break lower.0
        }
        lower.0 += upper.0;
        lower.1 += upper.1;
    }
}

pub fn euler71_example() {
    println!("\nProblem: By listing the set of reduced proper fractions for d ≤ 1,000,000 in ascending order of size, find the numerator of the fraction immediately to the left of 3/7.");
    println!("\n\nThis is a question about Farey Sequences. One of the best known properties of these sequences is that given three sequential terms the middle one is the mediant of outer two. Since this is always the case it is possible to determine the term to the left of 3/7 by finding that 5/12 is between 2/5 and 3/7 then finding what is between 5/12 and 3/7 and so on until the denominator is too large.");
    let s = "
pub fn euler71() -> u64 {
    let mut lower = (2,5);
    let upper = (3,7);
    loop {
        if lower.1 + upper.1 > 1_000_000 {
            break lower.0
        }
        lower.0 += upper.0;
        lower.1 += upper.1;
    }
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler71());
}


#[test]
fn test71() {
    assert_eq!(euler71(),428570)
}