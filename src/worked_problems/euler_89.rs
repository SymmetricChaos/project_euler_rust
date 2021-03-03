// Problem: Find the number of characters saved by writing each of the Roman Numeral from the provided file in their minimal form.

/*

*/

use std::{
    fs
};



fn read_data() -> Vec<String> {
    let s = fs::read_to_string("Euler89Doc.txt").unwrap();
    let rows = s.split("\r\n");
    let mut vec = Vec::new();
    for r in rows {
        vec.push(r.to_owned());
    }
    vec
}

/*
fn parse_rn(s: &str) -> u64 {

}

fn int_to_rn(n: u64) -> String {

}
*/

pub fn euler89() -> u64 {
    println!("{:?}",read_data());
    0u64
}

pub fn euler89_example() {
    println!("\nProblem: Find the number of characters saved by writing each of the Roman Numeral from the provided file in their minimal form.");
    println!("\n\n");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler89());
}


#[test]
fn test89() {
    assert_eq!(euler89(),)
}
