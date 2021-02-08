// Find the maximum total from top to bottom of the triangle below:

/*
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
*/

use std::fs;
use std::cmp::max;

// Read the file and create a vector of vectors that contains u16s
fn read_data() -> Vec<Vec<u16>> {
    let s = fs::read_to_string("Euler18Doc.txt").unwrap();
    let rows = s.split("\r\n");
    let mut vec = Vec::new();
    for r in rows {
        let elems = r.split(" ");
        let v = elems.map(|x| x.parse::<u16>().unwrap()).collect::<Vec<u16>>();
        vec.push(v);
    }
    vec
}

// Search through the triangle created for the greatest sum
fn search_triangle(t: &Vec<Vec<u16>>, a: usize, b: usize) -> u16 {
    if a == 14 {
        return t[a][b];
    } else {
        return max(t[a][b] + search_triangle(t,a+1,b), t[a][b] + search_triangle(t,a+1,b+1));
    }
}

pub fn euler18() -> u64 {
    let triangle = read_data();
    search_triangle(&triangle,0,0) as u64
}

pub fn euler18_example() {
    println!("\nProblem: Find the maximum total from top to bottom of the triangle provided.");
    println!("\n\nAlthough the description provided warns that a clever method is needed for a larger example in this case there is no difficulty in simply trying every route.");
    let s = "
use std::fs;
use std::cmp::max;

// Read the file and create a vector of vectors that contains u16s
fn read_data() -> Vec<Vec<u16>> {
    let s = fs::read_to_string(\"Euler18Doc.txt\").unwrap();
    let rows = s.split(\"\\r\\n\");
    let mut vec = Vec::new();
    for r in rows {
        let elems = r.split(\" \");
        let v = elems.map(|x| x.parse::<u16>().unwrap()).collect::<Vec<u16>>();
        vec.push(v);
    }
    vec
}

// Search through the triangle created for the greatest sum
fn search_triangle(t: &Vec<Vec<u16>>, a: usize, b: usize) -> u16 {
    if a == 14 {
        return t[a][b];
    } else {
        return max(t[a][b] + search_triangle(t,a+1,b), t[a][b] + search_triangle(t,a+1,b+1));
    }
}

pub fn euler18() -> u64 {
    let triangle = read_data();
    search_triangle(&triangle,0,0) as u64
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler18());
}

#[test]
fn test18() {
    assert_eq!(euler18(),1074)
}