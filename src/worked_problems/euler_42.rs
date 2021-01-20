// Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly two-thousand common English words, how many are triangle words?

use std::fs;

// Read the file and create a vector of strings
fn read_data() -> Vec<String> {
    let s = fs::read_to_string("Euler42Doc.txt").unwrap();
    let words = s.split(",");
    let mut v = Vec::new();
    for w in words {
        v.push(String::from(w.replace("\"","")))
    }
    v
}

fn triangle_nums(limit: u64) -> Vec<u64> {
    let mut num = 0;
    let mut n = 0;
    let mut triang = Vec::new();
    while num < limit {
        n += 1;
        num += n;
        triang.push(num as u64);
    }
    triang
}

fn alpha_score(s: &str) -> u64 {
    let mut out = 0u64;
    for ch in s.chars() {
        out += ch.to_digit(36).unwrap() as u64 - 9u64;
    }
    out
}

pub fn euler42() -> u64 {
    let mut out = 0;
    let words = read_data();
    // Find the largest alpha_score possible using simple multiplication
    let mut limit = 0;
    for word in &words {
        if word.len()*26 > limit {
            limit = word.len()*26;
        }
    }
    // Get all the triangle numbers up to and including that limit
    let triang = triangle_nums(limit as u64);
    // Calculate and match
    for word in words {
        if triang.contains(&alpha_score(&word)) {
            out += 1
        }
    }
    out
}

pub fn euler42_example() {
    println!("\nProblem: A triangle word shall be words such that the sum of the indicies of its letters (A=1, B=2, etc) sum to a triangle number. Using the provided text file containing nearly two-thousand common English words, how many are triangle words?");
    println!("\n\nCalculting the value of the letters uses the same trick as previously. A quick pass over the list can quickly set and upper limit on the triangle numbers that need to be generated. Then once the words are scored they can quickly be tested for being triangluar numbers by checking the list.");
    let s = "
use std::fs;

// Read the file and create a vector of strings
fn read_data() -> Vec<String> {
    let s = fs::read_to_string(\"Euler42Doc.txt\").unwrap();
    let words = s.split(\",\");
    let mut v = Vec::new();
    for w in words {
        v.push(String::from(w.replace(\"\\\"\",\"\")))
    }
    v
}

fn triangle_nums(limit: u64) -> Vec<u64> {
    let mut num = 0;
    let mut n = 0;
    let mut triang = Vec::new();
    while num < limit {
        n += 1;
        num += n;
        triang.push(num as u64);
    }
    triang
}

fn alpha_score(s: &str) -> u64 {
    let mut out = 0u64;
    for ch in s.chars() {
        out += ch.to_digit(36).unwrap() as u64 - 9u64;
    }
    out
}

pub fn euler42() -> u64 {
    let mut out = 0;
    let words = read_data();
    // Find the largest alpha_score possible using simple multiplication
    let mut limit = 0;
    for word in &words {
        if word.len()*26 > limit {
            limit = word.len()*26;
        }
    }
    // Get all the triangle numbers up to and including that limit
    let triang = triangle_nums(limit as u64);
    // Calculate and match
    for word in words {
        if triang.contains(&alpha_score(&word)) {
            out += 1
        }
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler42());
}

#[test]
fn test42() {
    assert_eq!(euler42(),162)
}