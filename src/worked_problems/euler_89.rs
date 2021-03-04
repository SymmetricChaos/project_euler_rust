// Problem: Find the number of characters saved by writing each of the Roman Numeral from the provided file in their minimal form.

/*

*/

use std::{
    fs,
    collections::HashMap,
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


fn parse_rn(s: &String, map: &HashMap::<char,u64>) -> u64 {
    let mut nums = Vec::new();
    for c in s.chars() {
        nums.push(map[&c])
    }

    let mut out = 0u64;
    let mut pos = 0;
    loop {
        if pos >= nums.len() {
            break
        }
        if pos == nums.len() - 1 {
            out += nums[pos];
            break
        } else if nums[pos] < nums[pos+1] {
            out += nums[pos+1] - nums[pos];
            pos += 1
        } else {
            out += nums[pos]
        }
        pos += 1
    }
    out
}


fn int_to_rn(n: u64) -> String {
    let mut n = n;
    let mut chars = Vec::new();
    while n > 0 {
        if n >= 1000 {
            n -= 1000;
            chars.push('M')
        } else if n >= 900 {
            n -= 900;
            chars.push('C');
            chars.push('M')
        } else if n >= 500 {
            n -= 500;
            chars.push('D')
        } else if n >= 400 {
            n -= 400;
            chars.push('C');
            chars.push('D')
        } else if n >= 100 {
            n -= 100;
            chars.push('C')
        } else if n >= 90 {
            n -= 90;
            chars.push('X');
            chars.push('C')
        } else if n >= 50 {
            n -= 50;
            chars.push('L')
        } else if n >= 40 {
            n -= 40;
            chars.push('X');
            chars.push('L')
        } else if n >= 10 {
            n -= 10;
            chars.push('X')
        } else if n >= 9 {
            n -= 9;
            chars.push('I');
            chars.push('X')
        } else if n >= 5  {
            n -= 5;
            chars.push('V')
        } else if n >= 4 {
            n -= 4;
            chars.push('I');
            chars.push('V')
        } else {
            n -= 1;
            chars.push('I')
        }
    }
    let out: String = chars.iter().collect();
    out
}


pub fn euler89() -> u64 {
    let data = read_data();
    let map: HashMap::<char,u64> = [('I',1), ('V',5), ('X',10), ('L',50), 
                                    ('C',100), ('D',500), ('M',1000)].iter().cloned().collect();

    let mut orig_chars = 0u64;
    let mut new_chars = 0u64;

    for d in data.iter() {
        orig_chars += d.len() as u64;
        
        let n = parse_rn(&d,&map);
        let c = int_to_rn(n);
        new_chars += c.len() as u64;
    }

    orig_chars-new_chars
}


pub fn euler89_example() {
    println!("\nProblem: Find the number of characters saved by writing each of the Roman Numeral from the provided file in their minimal form.");
    println!("\n\nTo parse the Roman Numerals as integers we look ahead by one symbol to see if subtractive notation is used and otherwise just add up the symbols. I'm not aware of any \'elegant\' way to produce minimal Roman Numerals if we just use a lot of if-else for that.");
    let s = "
use std::{
    fs,
    collections::HashMap,
};

fn read_data() -> Vec<String> {
    let s = fs::read_to_string(\"Euler89Doc.txt\").unwrap();
    let rows = s.split(\"\r\n\");
    let mut vec = Vec::new();
    for r in rows {
        vec.push(r.to_owned());
    }
    vec
}

fn parse_rn(s: &String, map: &HashMap::<char,u64>) -> u64 {
    let mut nums = Vec::new();
    for c in s.chars() {
        nums.push(map[&c])
    }

    let mut out = 0u64;
    let mut pos = 0;
    loop {
        if pos >= nums.len() {
            break
        }
        if pos == nums.len() - 1 {
            out += nums[pos];
            break
        } else if nums[pos] < nums[pos+1] {
            out += nums[pos+1] - nums[pos];
            pos += 1
        } else {
            out += nums[pos]
        }
        pos += 1
    }
    out
}

fn int_to_rn(n: u64) -> String {
    let mut n = n;
    let mut chars = Vec::new();
    while n > 0 {
        if n >= 1000 {
            n -= 1000;
            chars.push('M')
        } else if n >= 900 {
            n -= 900;
            chars.push('C');
            chars.push('M')
        } else if n >= 500 {
            n -= 500;
            chars.push('D')
        } else if n >= 400 {
            n -= 400;
            chars.push('C');
            chars.push('D')
        } else if n >= 100 {
            n -= 100;
            chars.push('C')
        } else if n >= 90 {
            n -= 90;
            chars.push('X');
            chars.push('C')
        } else if n >= 50 {
            n -= 50;
            chars.push('L')
        } else if n >= 40 {
            n -= 40;
            chars.push('X');
            chars.push('L')
        } else if n >= 10 {
            n -= 10;
            chars.push('X')
        } else if n >= 9 {
            n -= 9;
            chars.push('I');
            chars.push('X')
        } else if n >= 5  {
            n -= 5;
            chars.push('V')
        } else if n >= 4 {
            n -= 4;
            chars.push('I');
            chars.push('V')
        } else {
            n -= 1;
            chars.push('I')
        }
    }
    let out: String = chars.iter().collect();
    out
}

pub fn euler89() -> u64 {
    let data = read_data();
    let map: HashMap::<char,u64> = [('I',1), ('V',5), ('X',10), ('L',50), 
                                    ('C',100), ('D',500), ('M',1000)].iter().cloned().collect();

    let mut orig_chars = 0u64;
    let mut new_chars = 0u64;

    for d in data.iter() {
        orig_chars += d.len() as u64;
        
        let n = parse_rn(&d,&map);
        let c = int_to_rn(n);
        new_chars += c.len() as u64;
    }

    orig_chars-new_chars
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler89());
}


#[test]
fn test89() {
    assert_eq!(euler89(),743)
}
