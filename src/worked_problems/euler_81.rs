// Problem: Find the minimal path sum from the top left to the bottom right by only moving right and down in the provided file.

/*

*/
use std::fs;
use std::cmp::min;

// The sum will not overflow a u32
fn read_data() -> Vec<Vec<u32>> {
    let s = fs::read_to_string("Euler81Doc.txt").unwrap();
    let rows = s.split("\r\n");
    let mut vec = Vec::new();
    for r in rows {
        let elems = r.split(",");
        let v = elems.map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        vec.push(v);
    }
    vec
}

fn top_triangle(s: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut t = Vec::new();
    for x in 0..80 {
        let mut r = vec![];
        for (i,j) in (0..=x).rev().zip(0..=x) {
            r.push(s[i][j])
        }
        t.push(r)
    }
    t
}

// Work from the bottom up!
// I expect it is easier to think of this in terms of a diamond rather than a square
// As a diamond it has 159 rows (79+80)
/*
fn search_diamond(t: &mut Vec<Vec<u32>>, row: usize, col: usize) -> u32 {


    /*
    let mut row = 159;
    while row != 0 {
        for (pos,val) in t[row].clone().iter().enumerate() {
            t[row][pos] = val + min(t[row+1][pos],t[row+1][pos+1])
        }
        row -= 1
    }
    return t[0][0] + min(t[1][0],t[1][0])
    */
}
*/

pub fn euler81() -> u64 {
    let mat = read_data();
    println!("{:?}",top_triangle(&mat));
    0u64
}

pub fn euler81_example() {
    println!("\nProblem: Find the minimal path sum from the top left to the bottom right by only moving right and down in the provided file.");
    println!("\n\n");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler81());
}

/*
#[test]
fn test81() {
    assert_eq!(euler81(),)
}
*/