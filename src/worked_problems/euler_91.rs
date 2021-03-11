// Problem: How many right tirangles with integer coordinates including the origin can be made given that 0 ≤ x1, y1, x2, y2 ≤ 50?

/*
Many of these right triangles will have their right angle aligned with the grid. The real challenge is the other ones.

The aligned triangles should be straightforward to count using combinatorics.
If the right angle is at the origin then x1=0 and y1=0 and there is one triangle for every permutation values of x2 and y2. In this case 50*50 = 2500
If the right angle is on the x-axis but not the origin then y1=0 and x1=x2, so again there are 2500.
The same argument applies if the right angle is on the y-axis but not the origin so 2500 more.

Perhaps we can produce pythagorean triples and use them?
We want the hypotenuse along the x or y-axis.

Assume the hypotenuse lies along the x axis now using the right triangle altitude theorem we know that the following holds for height h the splits the hypotenuse into e and d
h*h = e*d
thus is e*d is a perfect square then h is an integer

*/



pub fn euler91() -> u64 {
    // Start with the count of trivial right triangles
    let mut out = 7500;
    
    // Perfect squares with root less than or equal to 50
    let squares: Vec<u32> = (1..50).map(|x| x*x).collect();
    for a in 1..=50 {
        for b in a..=50-a {
            if squares.contains(&(a*b)) {
                if a == b {
                    out += 2
                } else {
                    out += 4
                }
            }
        }
    }
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
