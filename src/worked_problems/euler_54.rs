// Problem: How many hands from the provided file does Player 1 win?

/*

*/

use std::fs;

#[derive(Debug)]
struct Card {
    rank: u8,
    suit: char,
}

fn str_to_card(s: &str) -> Card {
    let v: Vec<char> = s.chars().collect();
    let rank = match v[0] {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 100
    };
    Card{ rank: rank, suit: v[1]}
}


fn compare_hands(p1: &Vec<&str>, p2: &Vec<&str>) -> u8 {
    let p1hand: Vec<Card> = p1.into_iter().map(|x| str_to_card(x)).collect();
    let p2hand: Vec<Card> = p2.into_iter().map(|x| str_to_card(x)).collect();

    println!("{:?}",p1hand);
    println!("{:?}",p2hand);
    0u8
}



pub fn euler54() -> u64 {

    let s = fs::read_to_string("Euler54Doc.txt").unwrap();
    let games: Vec<&str> = s.split("\r\n").collect();

    let mut p1_hands: Vec<Vec<&str>> = Vec::new();
    let mut p2_hands: Vec<Vec<&str>> = Vec::new();

    for g in games {
        p1_hands.push(g[0..14].split(" ").collect());
        p2_hands.push(g[15..29].split(" ").collect());
    }

    compare_hands(&p1_hands[0],&p2_hands[0]);

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