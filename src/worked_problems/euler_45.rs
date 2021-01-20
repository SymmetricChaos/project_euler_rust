// 40755 is triangular, pentagonal, and hexagonal. What is the next number with this property?

use num::integer::Roots;

fn is_pentagonal(n: u64) -> bool {
    let a = ((24*n+1).sqrt()+1)/6;
    if (3*a*a-a)/2 == n {
        return true
    }
    false
}

pub fn euler45() -> u64 {

    let mut n = 143;
    let out = loop {
        n += 1;
        let hex = n*(2*n-1);
        if is_pentagonal(hex) {
            break hex
        }
    };
    out
}

pub fn euler45_example() {
    println!("\nProblem: 40755 is triangular, pentagonal, and hexagonal. What is the next number with this property?");
    println!("\n\nAll hexagonal numbers are also triangular so we only actually need to check for being pentagonal and hexagonal. Fortunately hexagonal numbers are quite sparse. The same method of checking if a number is pentagonal is used as in the previous problem.");
    let s = "
use num::integer::Roots;

fn is_pentagonal(n: u64) -> bool {
    let a = ((24*n+1).sqrt()+1)/6;
    if (3*a*a-a)/2 == n {
        return true
    }
    false
}

pub fn euler45() -> u64 {

    let mut n = 143;
    let out = loop {
        n += 1;
        let hex = n*(2*n-1);
        if is_pentagonal(hex) {
            break hex
        }
    };
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler45());
}

#[test]
fn test45() {
    assert_eq!(euler45(),1533776805)
}