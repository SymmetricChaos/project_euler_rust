// Problem: How many distinct arrangements of the two dice allow for all of the square numbers to be displayed?

/*
Important: USING THE DICE IN ANY ORDER
The number of possible six sided dice using the digits 0-9 without repetition is 210
So there can't be more than 210*210 = 44100 possibilities
Only half of those are distinct so really just 22050
That's possible to do by brute force
*/


pub fn euler90() -> u64 {
    let digits = [0,1,2,3,4,5,6,7,8,9];
    let squares = [01,04,09,16,25,36,49,64,81];



    0u64
}


pub fn euler90_example() {
    println!("\nProblem: How many distinct arrangements of the two dice allow for all of the square numbers to be displayed?");
    println!("\n\n");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler90());
}


#[test]
fn test90() {
    assert_eq!(euler90(),)
}
