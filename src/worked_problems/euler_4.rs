// Find the largest palindrome made from the product of two 3-digit numbers.

fn is_palindrome(n: u64) -> bool {
    let s1 = n.to_string();
    let s2 = s1.chars().rev().collect::<String>();
    return s1 == s2;
}

pub fn euler4() -> u64 {
    let mut biggest = 0;
    for n in 100..999 {
        for m in 100..999 {
            let p = n*m;
            if is_palindrome(p) && p > biggest {
                biggest = p;
            }
        }
    }
    biggest
}

pub fn euler4_example() {
    println!("\nProblem: Find the largest palindrome made from the product of two 3-digit numbers.");
    println!("\n\nA bit of string manipulation can be used to quickly test if a number is a palindrome.");
    let s = "
fn is_palindrome(n: u64) -> bool {
    let s1 = n.to_string();
    let s2 = s1.chars().rev().collect::<String>();
    return s1 == s2;
}

pub fn euler4() -> u64 {
    let mut biggest = 0;
    for n in 100..999 {
        for m in 100..999 {
            let p = n*m;
            if is_palindrome(p) && p > biggest {
                biggest = p;
            }
        }
    }
    biggest
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler4());
}

#[test]
fn test4() {
    assert_eq!(euler4(),906609)
}
