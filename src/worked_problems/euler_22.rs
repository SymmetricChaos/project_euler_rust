// Begin by sorting the file into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.

// What is the total of all the name scores in the file?

use std::fs;

// Read the file and create a vector of strings
fn read_data() -> Vec<String> {
    let s = fs::read_to_string("Euler22Doc.txt").unwrap();
    let words = s.split(",");
    let mut v = Vec::new();
    for w in words {
        v.push(String::from(w.replace("\"","")))
    }
    v
}


fn alpha_score(s: String) -> u64 {
    let mut out = 0u64;
    for ch in s.chars() {
        out += ch.to_digit(36).unwrap() as u64 - 9u64;
    }
    out
}


pub fn euler22() -> u64 {
    let mut v = read_data();
    v.sort();
    let mut out = 0u64;
    let mut ctr = 1;
    for word in v {
        out += alpha_score(word)*ctr;
        ctr += 1;
    }
    out
}