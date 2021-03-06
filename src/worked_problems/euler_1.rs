// Find the sum of all the multiples of 3 or 5 below 1000.

pub fn euler1() -> u64 {
    let mut s = 0;
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            s += i;
        }
    }
    s
}

pub fn euler1_example() {
    println!("\nProblem: Find the sum of all the multiples of 3 or 5 below 1000.");
    println!("\n\nThe code is short and simple. Iterate over the integers from 1 to 999 checking if they are multiples of 3 or 5");
    let s = "pub fn euler1() -> u64 {
    let mut s = 0;
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            s += i;
        }
    }
    s
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler1());
}

#[test]
fn test01() {
    assert_eq!(euler1(),233168)
}