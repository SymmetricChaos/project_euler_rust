// Problem: The key to this encrypted message consists of three lowercase letters XOR'd with valid ASCII text. Decrypt the message and find the sum of the ASCII values in the original text.

/*
Knowing that the password consists of three lowercase letters less us that there are only 17576 possibilities to check. That is trivial by brute force.
Thus our effort should focus on identifying the real text. These words need to be common enough that we can be sure to see them but long enough they won't often appear at random. For instance "I" and "a" are common words but will surely show up randomly.
Lets try looking for "the " and "and " which being for symbols in length are very unlikely to occur by accident.


*/

use std::fs;

fn apply_xor(numbers: &Vec<u8>,key: &Vec<u8>) -> Vec<u8> {
    let mut long_key = key.iter().cycle();
    let mut out = Vec::<u8>::new();
    for (v,k) in numbers.iter().zip(&mut long_key) {
        out.push(v^k);
    }
    out
}

fn numbers_to_text(numbers: &Vec<u8>) -> String {
    let letters: Vec<char> = numbers.iter().map(|x| *x as char).collect();
    let text: String = letters.iter().collect();
    text
}

fn sum_of_u8(digits: &Vec<u8>) -> u64 {
    let mut sum = 0u64;
    for d in digits.iter() {
        sum += *d as u64
    }
    sum
}

pub fn euler59() -> u64 {
    let s = fs::read_to_string("files\\Euler59Doc.txt").unwrap();
    let strings: Vec<&str> = s.split(",").collect();
    let numbers: Vec<u8> = strings.iter().map(|x| str::parse::<u8>(x).unwrap()).collect();
    for a in 97..=122 {
        for b in 97..=122 {
            for c in 97..=122 {
                let plain_num = apply_xor(&numbers, &vec![a,b,c]);
                let plain_text = numbers_to_text(&plain_num);
                if plain_text.contains(" the ") {
                    return sum_of_u8(&plain_num);
                }
            }
        }
    }
    0u64
}

pub fn euler59_example() {
    println!("\nProblem: The key to this encrypted message consists of three lowercase letters XOR'd with valid ASCII text. Decrypt the message and find the sum of the ASCII values in the original text.");
    println!("\n\nKnowing that the password consists of three lowercase letters less us that there are only 17576 possibilities to check. Thus the focus should be on identifying real text. To do this we need sequences of symbols we can almost guarantee to find. The most obvious is \" the \" which is five symbols long and thus very unlikely to occur by random. Running through the possibilities until this occurs produces the answer.");
    let s = "
use std::fs;

fn apply_xor(numbers: &Vec<u8>,key: &Vec<u8>) -> Vec<u8> {
    let mut long_key = key.iter().cycle();
    let mut out = Vec::<u8>::new();
    for (v,k) in numbers.iter().zip(&mut long_key) {
        out.push(v^k);
    }
    out
}

fn numbers_to_text(numbers: &Vec<u8>) -> String {
    let letters: Vec<char> = numbers.iter().map(|x| *x as char).collect();
    let text: String = letters.iter().collect();
    text
}

fn sum_of_u8(digits: &Vec<u8>) -> u64 {
    let mut sum = 0u64;
    for d in digits.iter() {
        sum += *d as u64
    }
    sum
}

pub fn euler59() -> u64 {
    let s = fs::read_to_string(\"Euler59Doc.txt\").unwrap();
    let strings: Vec<&str> = s.split(\",\").collect();
    let numbers: Vec<u8> = strings.iter().map(|x| str::parse::<u8>(x).unwrap()).collect();
    for a in 97..=122 {
        for b in 97..=122 {
            for c in 97..=122 {
                let plain_num = apply_xor(&numbers, &vec![a,b,c]);
                let plain_text = numbers_to_text(&plain_num);
                if plain_text.contains(\" the \") {
                    return sum_of_u8(&plain_num);
                }
            }
        }
    }
    0u64
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler59());
}

#[test]
fn test59() {
    assert_eq!(euler59(),129448)
}