// Problem: How many hands from the provided file does Player 1 win?

/*

*/

use maplit::hashmap;
use std::fs;

// Return 1 for p1 wins, 2 for p2 wins, and 0 for a tie
fn high_card_winner(p1: &Vec, p2: &Vec, rank: &Vec) -> u8 {
     
}


pub fn euler54() -> u64 {
    let rankmap = hashmap! {
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "J" => 10,
        "Q" => 11,
        "K" => 12,
        "A" => 13,
    };
    println!("{} -> {}","Q",rankmap["Q"]);

    let s = fs::read_to_string("Euler54Doc.txt").unwrap();
    let games: Vec<&str> = s.split("\r\n").collect();

    let mut p1_hands: Vec<Vec<&str>> = Vec::new();
    let mut p2_hands: Vec<Vec<&str>> = Vec::new();

    for g in games {
        p1_hands.push(g[0..14].split(" ").collect());
        p2_hands.push(g[15..29].split(" ").collect());
    }



    0u64
}

pub fn euler54_example() {
    println!("\nProblem: How many hands from the provided file does Player 1 win?");
    println!("\n\n");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler54());
}

#[test]
fn test54() {
    assert_eq!(euler54(),376)
}