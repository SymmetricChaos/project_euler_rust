// Problem: If, instead of using two 6-sided dice, two 4-sided dice are used, find the six-digit modal string representing the three most frequently visited squares in Monopoly.
/*
Monte-Carlo simulation?
*/

use std::collections::VecDeque;
use rand::prelude::*;

const CHEST: [u8;3] = [2,17,33];
const CHANCE: [u8;3] = [7,22,36];
const RAIL: [u8;4] = [5,15,25,35];
const UTIL: [u8;2] = [12,18];
const GO: u8 = 0;
const JAIL: u8 = 10;
const GTJ: u8 = 30;


struct Board {
    pos: u8,
    chest: VecDeque<String>,
    chance: VecDeque<String>,
    doubles_ctr: u8,
}

impl Board {
    fn roll(&mut self) -> usize {
        //println!("At {}", self.pos);
        let d1 = rand::thread_rng().gen_range(1..=4);
        let d2 = rand::thread_rng().gen_range(1..=4);
        let roll = d1 + d2;

        if d1 == d2 {
            //println!("Rolled Doubles");
            self.doubles_ctr += 1
        } else {
            self.doubles_ctr = 0
        }
        if self.doubles_ctr == 3 {
            //println!("Rolled doubles 3 times in a row, GO TO JAIL");
            self.pos = JAIL;
            return JAIL as usize
        }

        self.pos = (self.pos + roll) % 40;

        //println!("Rolled {}, landed on {}", roll, self.pos);

        if self.pos == GTJ {
            //println!("GO TO JAIL");
            self.pos = JAIL
        }

        // Community Chest Squares
        if CHEST.contains(&self.pos) {
            //println!("COMMUNITY CHEST");
            let card = self.chest.pop_front().unwrap();
            if card == "G" {
                //println!("GO TO GO");
                self.pos = GO
            }
            if card == "J" {
                //println!("GO TO JAIL");
                self.pos = JAIL
            }
            self.chest.push_back(card);
        }

        // Chance Squares
        if CHANCE.contains(&self.pos) {
            //println!("CHANCE");
            let card = self.chance.pop_front().unwrap();
            if card == "G" {
                //println!("GO TO GO");
                self.pos = GO
            } else if card == "J" {
                //println!("GO TO JAIL");
                self.pos = JAIL
            } else if card == "C1" {
                //println!("GO TO C1");
                self.pos = 11
            } else if card == "E3" {
                //println!("GO TO E3");
                self.pos = 24
            } else if card == "H2" {
                //println!("GO TO H2");
                self.pos = 39
            } else if card == "R1" {
                //println!("GO TO R1");
                self.pos = 5
            } else if card == "R" {
                //println!("GO TO NEXT RAIL");
                while !RAIL.contains(&self.pos) {
                    self.pos = (self.pos + 1) % 40;
                }
            } else if card == "U" {
                //println!("GO TO NEXT UTIL");
                while !UTIL.contains(&self.pos) {
                    self.pos = (self.pos + 1) % 40;
                }
            } else if card == "3" {
                //println!("GO BACK 3");
                self.pos += 40;
                self.pos = (self.pos - 3) % 40;
            }

            self.chance.push_back(card);
        }

        self.pos as usize
    }

    fn new(chest: VecDeque<String>, chance: VecDeque<String>) -> Board {
        Board {pos: 0, chest, chance, doubles_ctr: 0}
    }
}

pub fn euler84() -> u64 {
    let mut cc_cards = vec!["G","J","_","_","_","_","_","_","_","_","_","_","_","_","_","_"];
    let mut ch_cards = vec!["G","J","C1","E3","H2","R1","R","R","U","3","_","_","_","_","_","_"];
    let mut rng = rand::thread_rng();

    let mut spaces = [0u32;40];

    for _ in 0..2000 {
        cc_cards.shuffle(&mut rng);
        ch_cards.shuffle(&mut rng);
    
        let mut chest = VecDeque::<String>::new();
        let mut chance = VecDeque::<String>::new();
        for i in cc_cards.iter() {
            chest.push_back(i.to_string())
        }
        for i in ch_cards.iter() {
            chance.push_back(i.to_string())
        }

        let mut game = Board::new(chest,chance);

        for _ in 0..2000 {
            spaces[game.roll()] += 1;
        }
    }

    let mut v = Vec::new();
    for (pos, val) in spaces.iter().enumerate() {
        v.push( (pos,val) )
    }

    v.sort_by_key(|x| x.1);

    (v[39].0*10000 + v[38].0*100 + v[37].0) as u64
}

pub fn euler84_example() {
    println!("\nProblem: If, instead of using two 6-sided dice, two 4-sided dice are used, find the six-digit modal string representing the three most frequently visited squares in Monopoly.");
    println!("\n\nThe problem provides extensive specifics to understand this problem. There is probably some clever way to solve this using transition matricies but I decided to just use a Monte-Carlo Simulation. That is to say, I simulated a large number of games are simulated and used the staistics from those.");
    let s = "
use std::collections::VecDeque;
use rand::prelude::*;

const CHEST: [u8;3] = [2,17,33];
const CHANCE: [u8;3] = [7,22,36];
const RAIL: [u8;4] = [5,15,25,35];
const UTIL: [u8;2] = [12,18];
const GO: u8 = 0;
const JAIL: u8 = 10;
const GTJ: u8 = 30;

struct Board {
    pos: u8,
    chest: VecDeque<String>,
    chance: VecDeque<String>,
    doubles_ctr: u8,
}

impl Board {
    fn roll(&mut self) -> usize {
        let d1 = rand::thread_rng().gen_range(1..=4);
        let d2 = rand::thread_rng().gen_range(1..=4);
        let roll = d1 + d2;

        if d1 == d2 {
            self.doubles_ctr += 1
        } else {
            self.doubles_ctr = 0
        }
        if self.doubles_ctr == 3 {
            self.pos = JAIL;
            return JAIL as usize
        }

        self.pos = (self.pos + roll) % 40;

        if self.pos == GTJ {
            self.pos = JAIL
        }

        // Community Chest Squares
        if CHEST.contains(&self.pos) {
            let card = self.chest.pop_front().unwrap();
            if card == \"G\" {
                self.pos = GO
            }
            if card == \"J\" {
                self.pos = JAIL
            }
            self.chest.push_back(card);
        }

        // Chance Squares
        if CHANCE.contains(&self.pos) {
            let card = self.chance.pop_front().unwrap();
            if card == \"G\" {
                self.pos = GO
            } else if card == \"J\" {
                self.pos = JAIL
            } else if card == \"C1\" {
                self.pos = 11
            } else if card == \"E3\" {
                self.pos = 24
            } else if card == \"H2\" {
                self.pos = 39
            } else if card == \"R1\" {
                self.pos = 5
            } else if card == \"R\" {
                while !RAIL.contains(&self.pos) {
                    self.pos = (self.pos + 1) % 40;
                }
            } else if card == \"U\" {
                while !UTIL.contains(&self.pos) {
                    self.pos = (self.pos + 1) % 40;
                }
            } else if card == \"3\" {
                self.pos += 40;
                self.pos = (self.pos - 3) % 40;
            }

            self.chance.push_back(card);
        }

        self.pos as usize
    }

    fn new(chest: VecDeque<String>, chance: VecDeque<String>) -> Board {
        Board {pos: 0, chest, chance, doubles_ctr: 0}
    }
}

pub fn euler84() -> u64 {
    let mut cc_cards = vec![\"G\",\"J\",\"_\",\"_\",\"_\",\"_\",\"_\",\"_\",\"_\",\"_\",\"_\",\"_\",\"_\",\"_\",\"_\",\"_\"];
    let mut ch_cards = vec![\"G\",\"J\",\"C1\",\"E3\",\"H2\",\"R1\",\"R\",\"R\",\"U\",\"3\",\"_\",\"_\",\"_\",\"_\",\"_\",\"_\"];
    let mut rng = rand::thread_rng();

    let mut spaces = [0u32;40];

    for _ in 0..2000 {
        cc_cards.shuffle(&mut rng);
        ch_cards.shuffle(&mut rng);
    
        let mut chest = VecDeque::<String>::new();
        let mut chance = VecDeque::<String>::new();
        for i in cc_cards.iter() {
            chest.push_back(i.to_string())
        }
        for i in ch_cards.iter() {
            chance.push_back(i.to_string())
        }

        let mut game = Board::new(chest,chance);

        for _ in 0..2000 {
            spaces[game.roll()] += 1;
        }
    }

    let mut v = Vec::new();
    for (pos, val) in spaces.iter().enumerate() {
        v.push( (pos,val) )
    }

    v.sort_by_key(|x| x.1);

    (v[39].0*10000 + v[38].0*100 + v[37].0) as u64
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler84());
}


#[test]
fn test84() {
    assert_eq!(euler84(),101524)
}
