// Problem: How many hands from the provided file does Player 1 win?

/*
High Card       - 1
One Pair        - 2
Two Pair        - 3
Three of a Kind - 4
Straight        - 5
Flush           - 6
Full House      - 7
Four of a Kind  - 8
Straight Flush  - 9
*/

use std::fs;
use std::cmp::{Ordering,max};
use itertools::Itertools;
use std::collections::HashMap;

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

// Are the cards a sequential sequence?
fn is_straight(hand: &Vec<Card>) -> bool {
    let low = hand.first().unwrap().rank;
    let mut ctr = 0;
    for i in hand {
        if i.rank-ctr != low {
            return false;
        }
        ctr += 1;
    }
    return true
}

fn rank_map(hand: &Vec<Card>) -> HashMap<u8,u8> {
    let mut rank_counts: HashMap<u8,u8> = HashMap::new();
    for card in hand {
        *rank_counts.entry(card.rank).or_insert(0) += 1;
    }
    rank_counts
}


fn score_hand(hand: &Vec<Card>) -> (u8,u8,Vec<u8>) {

    let mut unique_ranks = hand.iter().map(|card| card.rank).unique().collect_vec();
    unique_ranks.sort();
    let rank_counts = rank_map(&hand);


    if hand.iter().all(|x| x.suit == hand[0].suit) {
        if is_straight(hand) {
            return (9,*unique_ranks.last().unwrap(),unique_ranks)
        }
        return (6,*unique_ranks.last().unwrap(),unique_ranks)
    }

    if is_straight(hand) {
        return (5,*unique_ranks.last().unwrap(),unique_ranks)
    }

    for (rank,count) in rank_counts.iter() {
        if *count == 4 {
            return (8,*rank,unique_ranks)
        }
    }
    
    for (rank1,count1) in rank_counts.iter() {
        if *count1 == 3 {
            for (rank2,count2) in rank_counts.iter() {
                if *count2 == 2 {
                    return (7,max(*rank1,*rank2),unique_ranks)
                }
            }
            return (4,*rank1,unique_ranks)
        }
    }

    for (rank1,count1) in rank_counts.iter() {
        if *count1 == 2 {
            for (rank2,count2) in rank_counts.iter() {
                if *count2 == 2 && *rank2 != *rank1 {
                    return (3,max(*rank1,*rank2),unique_ranks)
                }
            }
            return (2,*rank1,unique_ranks)
        }
    }

    return (1,*unique_ranks.last().unwrap(),unique_ranks)
}

// Return true for p1 win, return false for p2 win or tie
fn compare_hands(p1: &Vec<&str>, p2: &Vec<&str>) -> bool {
    let mut p1_hand: Vec<Card> = p1.into_iter().map(|x| str_to_card(x)).collect();
    let mut p2_hand: Vec<Card> = p2.into_iter().map(|x| str_to_card(x)).collect();

    p1_hand.sort();
    p2_hand.sort();

    let mut p1_score = score_hand(&p1_hand);
    let mut p2_score = score_hand(&p2_hand);
    
    if p1_score.0 > p2_score.0 {
        return true
    } else if p1_score.0 == p2_score.0 {
        if p1_score.1 > p2_score.1 {
            return true
        } else if p1_score.1 == p2_score.1 {
            while p1_score.2.len() > 0 && p2_score.2.len() > 0 {
                let p1h = p1_score.2.pop().unwrap();
                let p2h = p2_score.2.pop().unwrap();
                if p1h > p2h {
                    return true
                } else if p1h < p2h {
                    return false
                }
            if p1_score.2.len() > p2_score.2.len() {
                return true
            }
            }
        }
    }
    false
}


pub fn euler54() -> u64 {

    let s = fs::read_to_string("files\\Euler54Doc.txt").unwrap();
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
    println!("\n\nThere's a ton of code here working out how to score the ranks of poker hands. Not much interesting to say. ");
    let s = "
use std::fs;
use std::cmp::{Ordering,max};
use itertools::Itertools;
use std::collections::HashMap;

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

// Are the cards a sequential sequence?
fn is_straight(hand: &Vec<Card>) -> bool {
    let low = hand.first().unwrap().rank;
    let mut ctr = 0;
    for i in hand {
        if i.rank-ctr != low {
            return false;
        }
        ctr += 1;
    }
    return true
}

fn rank_map(hand: &Vec<Card>) -> HashMap<u8,u8> {
    let mut rank_counts: HashMap<u8,u8> = HashMap::new();
    for card in hand {
        *rank_counts.entry(card.rank).or_insert(0) += 1;
    }
    rank_counts
}

fn score_hand(hand: &Vec<Card>) -> (u8,u8,Vec<u8>) {

    let mut unique_ranks = hand.iter().map(|card| card.rank).unique().collect_vec();
    unique_ranks.sort();
    let rank_counts = rank_map(&hand);

    if hand.iter().all(|x| x.suit == hand[0].suit) {
        if is_straight(hand) {
            return (9,*unique_ranks.last().unwrap(),unique_ranks)
        }
        return (6,*unique_ranks.last().unwrap(),unique_ranks)
    }

    if is_straight(hand) {
        return (5,*unique_ranks.last().unwrap(),unique_ranks)
    }

    for (rank,count) in rank_counts.iter() {
        if *count == 4 {
            return (8,*rank,unique_ranks)
        }
    }
    
    for (rank1,count1) in rank_counts.iter() {
        if *count1 == 3 {
            for (rank2,count2) in rank_counts.iter() {
                if *count2 == 2 {
                    return (7,max(*rank1,*rank2),unique_ranks)
                }
            }
            return (4,*rank1,unique_ranks)
        }
    }

    for (rank1,count1) in rank_counts.iter() {
        if *count1 == 2 {
            for (rank2,count2) in rank_counts.iter() {
                if *count2 == 2 && *rank2 != *rank1 {
                    return (3,max(*rank1,*rank2),unique_ranks)
                }
            }
            return (2,*rank1,unique_ranks)
        }
    }

    return (1,*unique_ranks.last().unwrap(),unique_ranks)
}

// Return true for p1 win, return false for p2 win or tie
fn compare_hands(p1: &Vec<&str>, p2: &Vec<&str>) -> bool {
    let mut p1_hand: Vec<Card> = p1.into_iter().map(|x| str_to_card(x)).collect();
    let mut p2_hand: Vec<Card> = p2.into_iter().map(|x| str_to_card(x)).collect();

    p1_hand.sort();
    p2_hand.sort();

    let mut p1_score = score_hand(&p1_hand);
    let mut p2_score = score_hand(&p2_hand);
    
    if p1_score.0 > p2_score.0 {
        return true
    } else if p1_score.0 == p2_score.0 {
        if p1_score.1 > p2_score.1 {
            return true
        } else if p1_score.1 == p2_score.1 {
            while p1_score.2.len() > 0 && p2_score.2.len() > 0 {
                let p1h = p1_score.2.pop().unwrap();
                let p2h = p2_score.2.pop().unwrap();
                if p1h > p2h {
                    return true
                } else if p1h < p2h {
                    return false
                }
            if p1_score.2.len() > p2_score.2.len() {
                return true
            }
            }
        }
    }
    false
}

pub fn euler54() -> u64 {

    let s = fs::read_to_string(\"Euler54Doc.txt\").unwrap();
    let games: Vec<&str> = s.split(\"\r\n\").collect();

    let mut p1_hands: Vec<Vec<&str>> = Vec::new();
    let mut p2_hands: Vec<Vec<&str>> = Vec::new();

    for g in games {
        p1_hands.push(g[0..14].split(\" \").collect());
        p2_hands.push(g[15..29].split(\" \").collect());
    }

    let mut ctr = 0;
    for i in 0..1000 {
        if compare_hands(&p1_hands[i],&p2_hands[i]) {
            ctr += 1;
        }
    }
    
    ctr as u64
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler54());
}

#[test]
fn test54() {
    assert_eq!(euler54(),376)
}