// Problem: How many hands from the provided file does Player 1 win?

/*

*/

use std::fs;
use std::cmp::Ordering;

#[derive(Debug)]
struct Card {
    rank: u8,
    suit: char,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
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

fn is_flush(hand: &Vec<Card>) -> bool {
    hand.iter().all(|x| x.suit == hand[0].suit)
}

fn is_straight(hand: &Vec<Card>) -> bool {
    let low = hand.first().unwrap().rank;
    let mut ctr = 0;
    for i in hand {
        if i.rank-ctr != low {
            return false
        }
        ctr += 1;
    }
    true
}

fn is_full_house(hand: &Vec<Card>) -> bool {

}

// Return true for p1 win, return false for p2 win or tie
fn compare_hands(p1: &Vec<&str>, p2: &Vec<&str>) -> bool {
    let mut p1_hand: Vec<Card> = p1.into_iter().map(|x| str_to_card(x)).collect();
    let mut p2_hand: Vec<Card> = p2.into_iter().map(|x| str_to_card(x)).collect();

    p1_hand.sort();
    p2_hand.sort();

    if is_straight(&p1_hand) {
        println!("{:?}",p1_hand);
    }
    if is_straight(&p2_hand) {
        println!("{:?}",p2_hand);
    }

    true
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

    for i in 0..1000 {
        compare_hands(&p1_hands[i],&p2_hands[i]);
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