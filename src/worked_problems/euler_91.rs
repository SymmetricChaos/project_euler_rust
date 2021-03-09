// Problem: How many right tirangles with integer coordinates including the origin can be made given that 0 ≤ x1, y1, x2, y2 ≤ 50?

/*
Many of these right triangles will have their right angle aligned with the grid. The real challenge is the other ones.

The aligned triangles should be straightforward to count using combinatorics.
If the right angle is at the origin then x1=0 and y1=0 and there is one triangle for every permutation values of x2 and y2. In this case 50*50 = 2500
If the right angle is on the x-axis but not the origin then y1=0 and x1=x2, so again there are 2500.
The same argument applies if the right angle is on the y-axis so 2500 more.

Perhaps we can produce pythagorean triples and use them?
We want the hypotenuse along the x or y-axis.
*/



pub fn euler91() -> u64 {
    // Start with the count of trivial right triangles
    let mut out = 7500;



    out
}


pub fn euler91_example() {
    println!("\nProblem: How many right tirangles with integer coordinates including the origin can be made given that 0 ≤ x1, y1, x2, y2 ≤ 50?");
    println!("\n\n");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler91());
}

/*
#[test]
fn test91() {
    assert_eq!(euler91(),)
}
*/
