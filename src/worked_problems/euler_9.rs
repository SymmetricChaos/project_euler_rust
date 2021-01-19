// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

pub fn euler9() -> u64 {
    let mut out = 0;
    // Using 'outer here allows us to short circut the nested loop when we find the 
    // answer
    'outer: for a in 1..1000 {
        for b in 1..1000-a {
            let c = 1000-a-b;
            if a*a + b*b == c*c {
                out = a*b*c;
                break 'outer;
            }
        }
    }
    out
}

pub fn euler9_example() {
    println!("\nThere exists exactly one Pythagorean triplet for which a + b + c = 1000.");
    let s = "
pub fn euler9() -> u64 {
    let mut out = 0;
    // Using 'outer here allows us to short circut the nested loop when we find the 
    // answer
    'outer: for a in 1..1000 {
        for b in 1..1000-a {
            let c = 1000-a-b;
            if a*a + b*b == c*c {
                out = a*b*c;
                break 'outer;
            }
        }
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler9());
}

#[test]
fn test9() {
    assert_eq!(euler9(),31875000)
}