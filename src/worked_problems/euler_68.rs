// Problem: Using the numbers 1 to 10, and depending on arrangements, it is possible to form 16- and 17-digit strings. What is the maximum 16-digit string for a "magic" 5-gon ring?

/*
Due to the details of the problem we know that 10 must be on one of the outer parts of the ring.
We do not need to seperately create solutions that are just rotations of each other as the rules make these equivalent.
These two facts mean we can always start with 10.
*/


    // Fill first group starting with 10, a, b
    // Mark the sum 10 + a + b as 'ring_const'
    // Fill the second row with c, b, (c+b-ring_const)
    // If that step fails try different values of c
    // If a working value is found continue
    // If a working value is not found go back to the first group
    // If the second group has been fill the third group with d, (c+b-ring_const), (d+(c+b-ring_const)-ring_const)
    // So on and so forth.
    // Last row needs to check only the outer number and sum since the middle and last are already filled
    // There are only six free variables
    // We select six without repetition and without replacement from a set of nine options
    // There are thus only 60480 possibilities to consider
    



fn fill_5_gon() {


    let a = 9;
    let b = 8;
    let row1 = [10,a,b];
    let ring_const: u8 = row1.iter().sum();

    let c = 7;
    let d = c+b-ring_const;
    let row2 = [c,b,d];

    let e = 6;
    let f = e+d-ring_const;
    let row3 = [e,d,f];

    let g = 6;
    let h = g+f-ring_const;
    let row4 = [g,f,h];

    let i = 5;
    let h = g+f-ring_const;
    let row4 = [i,h,b];
}



pub fn euler68() -> u64 {
    0u64
}

pub fn euler68_example() {
    println!("\nProblem: Using the numbers 1 to 10, and depending on arrangements, it is possible to form 16- and 17-digit strings. What is the maximum 16-digit string for a \"magic\" 5-gon ring?");
    println!("\n\n");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler68());
}


#[test]
fn test68() {
    assert_eq!(euler68(),6531031914842725)
}