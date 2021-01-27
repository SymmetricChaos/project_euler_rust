// Problem: How many hands from the provided file does Player 1 win?

/*

*/

use std::fs;
use std::cmp::{Ordering,max};
use itertools::Itertools;


#[derive(Debug,Hash)]
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

fn straight_flush(hand: &Vec<Card>) -> u8 {
    if flush(hand) > 0 && straight(hand) > 0 {
        return hand[4].rank
    }
    0
}

// Does any rank appear four times?
fn four_of_a_kind(hand: &Vec<Card>) -> u8 {
    let uniques = hand.iter().map(|card| card.rank).unique().collect_vec();
    for u in uniques {
        if hand.iter().filter(|elem| elem.rank == u).count() == 4 {
            return u
        }
    }
    0
}

// Are there exactly two ranks in the hand and one with length 2?
fn full_house(hand: &Vec<Card>) -> u8 {
    let uniques = hand.iter().map(|card| card.rank).unique().collect_vec();
    if uniques.len() == 2 {
        for u in uniques {
            if hand.iter().filter(|elem| elem.rank == u).count() == 2 {
                return hand[4].rank
            }
        }
    }
    0
}

// Are all the suits the same?
fn flush(hand: &Vec<Card>) -> u8 {
    if hand.iter().all(|x| x.suit == hand[0].suit) {
        return hand[4].rank
    }
    0
}

// Are the cards a sequential sequence?
fn straight(hand: &Vec<Card>) -> u8 {
    let low = hand.first().unwrap().rank;
    let mut ctr = 0;
    for i in hand {
        if i.rank-ctr != low {
            return 0;
        }
        ctr += 1;
    }
    return hand[4].rank
}

// Does any rank appear three times?
fn three_of_a_kind(hand: &Vec<Card>) -> u8 {
    let uniques = hand.iter().map(|card| card.rank).unique().collect_vec();
    for u in uniques {
        if hand.iter().filter(|elem| elem.rank == u).count() == 3 {
            return u
        }
    }
    0
}

// Does two ranks appear two times each?
fn two_pair(hand: &Vec<Card>) -> u8 {
    let uniques = hand.iter().map(|card| card.rank).unique().collect_vec();
    let mut high = 0u8;
    for u in uniques {
        if hand.iter().filter(|elem| elem.rank == u).count() == 2 {
            if high != 0 {
                return max(high,u)
            }
            high = u
        }
    }
    0
}

// Does any rank appear two times?
fn pair(hand: &Vec<Card>) -> u8 {
    let uniques = hand.iter().map(|card| card.rank).unique().collect_vec();
    for u in uniques {
        if hand.iter().filter(|elem| elem.rank == u).count() == 2 {
            return u
        }
    }
    0
}



// Return true for p1 win, return false for p2 win or tie
fn compare_hands(p1: &Vec<&str>, p2: &Vec<&str>) -> bool {
    let mut p1_hand: Vec<Card> = p1.into_iter().map(|x| str_to_card(x)).collect();
    let mut p2_hand: Vec<Card> = p2.into_iter().map(|x| str_to_card(x)).collect();

    p1_hand.sort();
    p2_hand.sort();

    if straight_flush(&p1_hand) > straight_flush(&p2_hand) {
        return true
    }
    if four_of_a_kind(&p1_hand) > four_of_a_kind(&p2_hand) {
        return true
    }
    if full_house(&p1_hand) > full_house(&p2_hand) {
        return true
    }
    if flush(&p1_hand) > flush(&p2_hand) {
        return true
    }
    if straight(&p1_hand) > straight(&p2_hand) {
        return true
    }
    if three_of_a_kind(&p1_hand) > three_of_a_kind(&p2_hand) {
        return true
    }
    if two_pair(&p1_hand) > two_pair(&p2_hand) {
        return true
    }
    if pair(&p1_hand) > pair(&p2_hand) {
        return true
    }
    if p1_hand[4].rank > p2_hand[4].rank {
        return true
    }
    false
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

    let mut ctr = 0;
    for i in 0..1000 {
        if compare_hands(&p1_hands[i],&p2_hands[i]) {
            ctr += 1;
        }
    }
    
    ctr as u64
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