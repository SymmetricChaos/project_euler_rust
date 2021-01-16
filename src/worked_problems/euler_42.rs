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


fn alpha_score(s: &str) -> u64 {
    let mut out = 0u64;
    for ch in s.chars() {
        out += ch.to_digit(36).unwrap() as u64 - 9u64;
    }
    out
}


pub fn euler42() -> u64 {
    let mut out = 0;
    let v = read_data();
    
    println!("{}",alpha_score("SKY"));
    
    out
}