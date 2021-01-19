// How many routes are there through a 20×20 grid starting in the top left corner and only being able to move to the right and down?

// This recursive algorithm works but is impractically slow
/*
fn paths(x: u64, y: u64) -> u64 {
    match x == 0 || y == 0 {
        true => 1,
        false => paths(x-1,y) + paths(x,y-1),
    }
}
*/

// Its much faster to use binomial coefficients
// The numerator overflows with 64 bit arithmetic so internall we must use 128 bit
// numbers
pub fn euler15() -> u64 {
    let n = 20u128;
    let mut numerator = 1u128;
    let mut denominator = 1u128;
    for i in 0..n {
        numerator *= 2*n-i;
        denominator *= n-i;
    }
    let out = numerator/denominator;
    return out as u64;
}

pub fn euler15_example() -> u64 {
    println!("\nProblem: How many routes are there through a 20×20 grid starting in the top left corner and only being able to move to the right and down?");
    println!("\n\nThe paths can be counted by a very simple algorithm, however that turns out to be infeasibly slow for a grid of the size requested. The number required also turns out to be a central binomial coefficient which can be calculate very easily.");
    let s = "
// The slow recursive algorithm which is at least very pretty
fn paths(x: u64, y: u64) -> u64 {
    match x == 0 || y == 0 {
        true => 1,
        false => paths(x-1,y) + paths(x,y-1),
    }
}

pub fn euler15() -> u64 {
    let n = 20u128;
    let mut numerator = 1u128;
    let mut denominator = 1u128;
    for i in 0..n {
        numerator *= 2*n-i;
        denominator *= n-i;
    }
    let out = numerator/denominator;
    return out as u64;
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler15());
    0u64
}

#[test]
fn test15() {
    assert_eq!(euler15(),137846528820)
}