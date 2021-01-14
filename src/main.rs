
use std::io;

mod rationals;
mod aux_funcs;

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
mod euler_18;
mod euler_19;
mod euler_20;
mod euler_21;
mod euler_22;
mod euler_23;
mod euler_24;
mod euler_25;
mod euler_26;
mod euler_27;
mod euler_28;
mod euler_29;
mod euler_30;
mod euler_31;
mod euler_32;
mod euler_33;
mod euler_34;
mod euler_35;
mod euler_36;
mod euler_37;
mod euler_38;


fn main() {
    println!("\nWelcome to My Project Euler Rust Project!");
    println!("You may type \"quit\" to exit.");
    loop {
        println!("\n\nWhich Project Euler Problem Would You Like To See?");
        let mut val = String::new();
        io::stdin().read_line(&mut val).expect("Failed to read line");

        match val.trim()  {
            "1" =>  println!("\nEuler  1: {}",euler_1::euler1()),
            "2" =>  println!("\nEuler  2: {}",euler_2::euler2()),
            "3" =>  println!("\nEuler  3: {}",euler_3::euler3()),
            "4" =>  println!("\nEuler  4: {}",euler_4::euler4()),
            "5" =>  println!("\nEuler  5: {}",euler_5::euler5()),
            "6" =>  println!("\nEuler  6: {}",euler_6::euler6()),
            "7" =>  println!("\nEuler  7: {}",euler_7::euler7()),
            "8" =>  println!("\nEuler  8: {}",euler_8::euler8()),
            "9" =>  println!("\nEuler  9: {}",euler_9::euler9()),
            "10" => println!("\nEuler 10: {}",euler_10::euler10()),
            "11" => println!("\nEuler 11: {}",euler_11::euler11()),
            "12" => println!("\nEuler 12: {}",euler_12::euler12()),
            "13" => println!("\nEuler 13: {}",euler_13::euler13()),
            "14" => println!("\nEuler 14: {}",euler_14::euler14()),
            "15" => println!("\nEuler 15: {}",euler_15::euler15()),
            "16" => println!("\nEuler 16: {}",euler_16::euler16()),
            "17" => println!("\nEuler 17: {}",euler_17::euler17()),
            "18" => println!("\nEuler 18: {}",euler_18::euler18()),
            "19" => println!("\nEuler 19: {}",euler_19::euler19()),
            "20" => println!("\nEuler 20: {}",euler_20::euler20()),
            "21" => println!("\nEuler 21: {}",euler_21::euler21()),
            "22" => println!("\nEuler 22: {}",euler_22::euler22()),
            "23" => println!("\nEuler 23: {}",euler_23::euler23()),
            "24" => println!("\nEuler 24: {}",euler_24::euler24()),
            "25" => println!("\nEuler 25: {}",euler_25::euler25()),
            "26" => println!("\nEuler 26: {}",euler_26::euler26()),
            "27" => println!("\nEuler 27: {}",euler_27::euler27()),
            "28" => println!("\nEuler 28: {}",euler_28::euler28()),
            "29" => println!("\nEuler 29: {}",euler_29::euler29()),
            "30" => println!("\nEuler 30: {}",euler_30::euler30()),
            "31" => println!("\nEuler 31: {}",euler_31::euler31()),
            "32" => println!("\nEuler 32: {}",euler_32::euler32()),
            "33" => println!("\nEuler 33: {}",euler_33::euler33()),
            "34" => println!("\nEuler 34: {}",euler_34::euler34()),
            "35" => println!("\nEuler 35: {}",euler_35::euler35()),
            "36" => println!("\nEuler 36: {}",euler_36::euler36()),
            "37" => println!("\nEuler 37: {}",euler_37::euler37()),
            "38" => println!("\nEuler 38: {}",euler_38::euler38()),
            "quit" => break,
            _ => println!("Haven't gotten to that one yet, sorry.")
        }
    }
}