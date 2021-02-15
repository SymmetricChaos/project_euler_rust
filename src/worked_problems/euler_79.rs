// Problem: Analyse the provided file so as to determine the shortest possible secret passcode of unknown length.
/*
Only 4 and 5 never appear in the file
Thus the passcode must have at least 8 digits
7 only appears in the first position and 0 only appears in the last position
The last two digits must be 90 since only 0 ever appears after 9
To the left of 0 we only ever find 1,2,6,8, or 9 which means none of those are the first digit

7------0

1 <- 6
1 <- 9
3 <- 1
6 <- 8
1 <- 8
6 <- 9
1 <- 2
2 <- 9
6 <- 2
8 <- 9
6 <- 2
3 <- 6
3 <- 1
2 <- 8

7 <- 3 <- 1 <- 6 <- 2 <- 8 <- 9 <- 0

*/


pub fn euler79() -> u64 {
    0u64
}

pub fn euler79_example() {
    println!("\nProblem: Analyse the provided file so as to determine the shortest possible secret passcode of unknown length.");
    println!("\n\nThis problem is exceptionally fair and I ended up solving it by hand.");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler79());
}

/*
#[test]
fn test79() {
    assert_eq!(euler79(),73162890)
}
*/