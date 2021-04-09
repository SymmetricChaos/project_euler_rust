// Problem: How many right tirangles with integer coordinates including the origin can be made given that 0 ≤ x1, y1, x2, y2 ≤ 50?

/*
A huge portion of the triangles can be found by hand. 
Observe that if the right angle is at the origin then only x2 and y2 are free so there are 50*50 = 2500 triangles of that kind. 
Likewise if the triangle lies on the x-axis or if it lies along the y-axis there are only two free variables. 
There are thus 7500 of these trivial triangles.


Perhaps we can produce pythagorean triples and use them?
We want the hypotenuse along the x or y-axis.

Assume the hypotenuse lies along the x axis now using the right triangle altitude theorem we know that the following holds for height h that splits the hypotenuse into e and d
h*h = e*d
thus if e*d is a perfect square then h is an integer

I am currently missing All THE triangle that do not lie along ANY axis
*/



pub fn euler91() -> u64 {
    // Start with the count of trivial right triangles
    let mut out = 7500;
    
    // Perfect squares with root less than or equal to 50
    let squares: Vec<u32> = (1..=50).map(|x| x*x).collect();
    println!{"{:?}",squares}

    // Iterate over ordered pairs (a,b) and sum a+b <= 50
    for a in 1..=50 {
        for b in 1..=50-a {
            // If a*b is in the set of squares then the heigher is an integer
            if squares.contains(&(a*b)) {
                println!("{},{},{}",a,b,a*b);
                out += 2
            }
        }
    }
    out
}


pub fn euler91_example() {
    println!("\nProblem: How many right tirangles with integer coordinates including the origin can be made given that 0 ≤ x1, y1, x2, y2 ≤ 50?");
    println!("\n\nA huge portion of the triangles can be found by hand. Observe that if the right angle is at the origin then only x2 and y2 are free so there are 50*50 = 2500 triangles of that kind. Likewise if the triangle lies on the x-axis or if it lies along the y-axis there are only two free variables. There are thus 7500 of these trivial triangles.");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler91());
}

/*
#[test]
fn test91() {
    assert_eq!(euler91(),14234 )
}
*/
