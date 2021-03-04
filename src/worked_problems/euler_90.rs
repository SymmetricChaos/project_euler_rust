// Problem: How many distinct arrangements of the two dice allow for all of the square numbers to be displayed?

/*
D1 can only be [0,1,2,3,4,6,8] and [0,1,2,3,4,8,9]

The work is thus what the options are for D2.
The numbers 1,4,5 and then 6 or 9 are required with two spaces remaining
[1,4,5,6,_,_] 
[1,4,5,9,_,_] 
*/


pub fn euler90() -> u64 {

    // There are two optional spaces for D2 since 1,4,5 is always required along with 6/9
    // Including only 6 is the same as including only 9
    // Assuming we always include 6 to fulfilling the 6/9 requirement then we pick from these
    let ex = [2,3,7,8,9];

    // The actual answer is much bigger than I've come up with. Must have made some sigificant error.
    let mut ctr = 0;



    ctr * 2
}


pub fn euler90_example() {
    println!("\nProblem: How many distinct arrangements of the two dice allow for all of the square numbers to be displayed?");
    println!("\n\nD1 can only be [0,1,2,3,4,6,8] or [0,1,2,3,4,8,9].");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler90());
}


#[test]
fn test90() {
    assert_eq!(euler90(),)
}
