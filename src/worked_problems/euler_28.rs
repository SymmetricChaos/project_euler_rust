// What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

// This can be solved by inspection with no need to build the whole spiral in memory.
// Every turn of the spiral has four numbers we care about and they're seperated by succesive
// odd numbers.
pub fn euler28() -> u64 {
    let mut out = 1;
    let mut cur = 1;
    let mut side_len = 2;
    
    loop {
        for _ in 0..4 {
            cur += side_len;
            out += cur;
        }
        side_len += 2;
        if side_len > 1001 {
            break;
        }
    }
    out
}

pub fn euler28_example() {
    println!("\nProblem: What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way as the example?");
    println!("\n\nEvery turn of the spiral has four numbers we care about and they're seperated by succesive odd numbers.");
    let s = "
pub fn euler28() -> u64 {
    let mut out = 1;
    let mut cur = 1;
    let mut side_len = 2;
    
    loop {
        for _ in 0..4 {
            cur += side_len;
            out += cur;
        }
        side_len += 2;
        if side_len > 1001 {
            break;
        }
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler28());
}

#[test]
fn test28() {
    assert_eq!(euler28(),669171001)
}