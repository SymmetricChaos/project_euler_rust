// Problem: If, instead of using two 6-sided dice, two 4-sided dice are used, find the six-digit modal string representing the three most frequently visited squares in Monopoly.
/*
Monte-Carlo simulation?
*/

use std::collections::VecDeque;
use std::iter::FromIterator;
use rand::prelude::*;

struct Board {
    pos: u8,
    chest: VecDeque<String>,
    chance: VecDeque<String>,
    doubles_ctr: u8,
}

impl Board {
    fn roll(&self) {
        let roll =  rand::thread_rng().gen_range(1..=4) + rand::thread_rng().gen_range(1..=4);
        self.pos = (self.pos + roll) % 40;

        // Community Chest Squares
        if [2,17,33].contains(&self.pos) {
            let card = self.chest.pop_front().unwrap();


            self.chest.push_back(card);
        }

        // Chance Squares
        if [7,22,36].contains(&self.pos) {
            let card = self.chance.pop_front().unwrap();


            self.chance.push_back(card);
        }
    }

    fn new(chest: VecDeque<String>, chance: VecDeque<String>,) -> Board {
        // shuffle chest
        // shuffle chance
        Board {pos: 0, chest, chance, doubles_ctr: 0}
    }
}

pub fn euler84() -> u64 {
    let mut chest = VecDeque::from_iter(&["G","J","_","_","_","_","_","_","_","_","_","_","_","_","_","_"]);
    let mut chance = VecDeque::from_iter(&["G","J","C1","E3","H2","R1","R","R","U","3","_","_","_","_","_","_"]);

}

pub fn euler84_example() {
    println!("\nProblem: If, instead of using two 6-sided dice, two 4-sided dice are used, find the six-digit modal string representing the three most frequently visited squares in Monopoly.");
    println!("\n\nThe problem provides extensive specifics to understand this problem.");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler84());
}

/*
#[test]
fn test84() {
    assert_eq!(euler84(),)
}
*/