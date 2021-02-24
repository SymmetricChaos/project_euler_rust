// Problem:  Find the maximum total from top to bottom in the provided text file containing a triangle with one-hundred rows.
/*

*/

use std::fs;
use std::cmp::max;

// We can use u16s here as the greatest possible path sum in 9900 which fits within a u16
fn read_data() -> Vec<Vec<u16>> {
    let s = fs::read_to_string("Euler67Doc.txt").unwrap();
    let rows = s.split("\r\n");
    let mut vec = Vec::new();
    for r in rows {
        let elems = r.split(" ");
        let v = elems.map(|x| x.parse::<u16>().unwrap()).collect::<Vec<u16>>();
        vec.push(v);
    }
    vec
}

// Work from the bottom up!
fn search_triangle(t: &mut Vec<Vec<u16>>) -> u16 {
    let mut row = 98;
    while row != 0 {
        for (pos,val) in t[row].clone().iter().enumerate() {
            t[row][pos] = val + max(t[row+1][pos],t[row+1][pos+1])
        }
        row -= 1
    }
    return t[0][0] + max(t[1][0],t[1][1])
}

pub fn euler67() -> u64 {
    let mut triangle = read_data();
    let out = search_triangle(&mut triangle);
    out as u64
}

pub fn euler67_example() {
    println!("\nProblem: Find the maximum total from top to bottom in the provided text file containing a triangle with one-hundred rows.");
    println!("\n\nThe full text of the problem explains why a brute force method is impossible in this case unlike in Problem 18. We can get around this by working from the bottom upward. Starting with the row index 98 we check what the best next step would be by adding to each position the greater of the two below it. Then, knowing that, we move backward to row 97 and look at row 98 to see what the best next step would be. This repeats until we get back the the row indexed 0.");
    let s = "
use std::fs;
use std::cmp::max;

// We can use u16s here as the greatest possible path sum in 9900 which fits within a u16
fn read_data() -> Vec<Vec<u16>> {
    let s = fs::read_to_string(\"Euler67Doc.txt\").unwrap();
    let rows = s.split(\"\\r\\n\");
    let mut vec = Vec::new();
    for r in rows {
        let elems = r.split(\" \");
        let v = elems.map(|x| x.parse::<u16>().unwrap()).collect::<Vec<u16>>();
        vec.push(v);
    }
    vec
}

fn search_triangle(t: &mut Vec<Vec<u16>>) -> u16 {
    let mut row = 98;
    while row != 0 {
        for (pos,val) in t[row].clone().iter().enumerate() {
            t[row][pos] = val + max(t[row+1][pos],t[row+1][pos+1])
        }
        row -= 1
    }
    return t[0][0] + max(t[1][0],t[1][1])
}

pub fn euler67() -> u64 {
    let mut triangle = read_data();
    let out = search_triangle(&mut triangle);
    println!(\"{}\",out);
    0u64
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler67());
}


#[test]
fn test67() {
    assert_eq!(euler67(),7273)
}