
use std::io;

mod euler_1;
mod euler_2;
mod euler_3;
mod euler_4;
mod euler_5;
mod euler_6;
mod euler_7;
mod euler_8;
mod euler_9;
mod euler_10;
mod euler_11;
mod euler_12;
mod euler_13;
mod euler_14;
mod euler_15;
mod euler_16;
mod euler_17;

fn main() {
    println!("\nWelcome to My Project Euler Rust Project!");
    println!("You may type \"quit\" to exit.");
    loop {
        println!("\n\nWhich Project Euler Problem Would You Like To See?");
        let mut val = String::new();
        io::stdin().read_line(&mut val).expect("Failed to read line");

        match val.trim()  {
            "1" => println!("Euler  1: {}",euler_1::euler1()),
            "2" => println!("Euler  2: {}",euler_2::euler2()),
            "3" => println!("Euler  3: {}",euler_3::euler3()),
            "4" => println!("Euler  4: {}",euler_4::euler4()),
            "5" => println!("Euler  5: {}",euler_5::euler5()),
            "6" => println!("Euler  6: {}",euler_6::euler6()),
            "7" => println!("Euler  7: {}",euler_7::euler7()),
            "8" => println!("Euler  8: {}",euler_8::euler8()),
            "9" => println!("Euler  9: {}",euler_9::euler9()),
            "10" => println!("Euler 10: {}",euler_10::euler10()),
            "11" => println!("Euler 11: {}",euler_11::euler11()),
            "12" => println!("Euler 12: {}",euler_12::euler12()),
            "13" => println!("Euler 13: {}",euler_13::euler13()),
            "14" => println!("Euler 14: {}",euler_14::euler14()),
            "15" => println!("Euler 15: {}",euler_15::euler15()),
            "16" => println!("Euler 16: {}",euler_16::euler16()),
            "17" => println!("Euler 17: {}",euler_17::euler17()),
            "quit" => break,
            _ => println!("Haven't gotten to that one yet, sorry.")
        }
    }
}