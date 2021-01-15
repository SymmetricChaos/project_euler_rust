
use std::io;

mod rationals;
mod aux_funcs;

mod worked_problems;


fn main() {
    println!("\nWelcome to My Project Euler Rust Project!");
    println!("You may type \"quit\" to exit.");
    loop {
        println!("\n\nWhich Project Euler Problem Would You Like To See?");
        let mut val = String::new();
        io::stdin().read_line(&mut val).expect("Failed to read line");

        match val.trim()  {
            "1" =>  println!("\nEuler  1: {}",worked_problems::euler_1::euler1()),
            "2" =>  println!("\nEuler  2: {}",worked_problems::euler_2::euler2()),
            "3" =>  println!("\nEuler  3: {}",worked_problems::euler_3::euler3()),
            "4" =>  println!("\nEuler  4: {}",worked_problems::euler_4::euler4()),
            "5" =>  println!("\nEuler  5: {}",worked_problems::euler_5::euler5()),
            "6" =>  println!("\nEuler  6: {}",worked_problems::euler_6::euler6()),
            "7" =>  println!("\nEuler  7: {}",worked_problems::euler_7::euler7()),
            "8" =>  println!("\nEuler  8: {}",worked_problems::euler_8::euler8()),
            "9" =>  println!("\nEuler  9: {}",worked_problems::euler_9::euler9()),
            "10" => println!("\nEuler 10: {}",worked_problems::euler_10::euler10()),
            "11" => println!("\nEuler 11: {}",worked_problems::euler_11::euler11()),
            "12" => println!("\nEuler 12: {}",worked_problems::euler_12::euler12()),
            "13" => println!("\nEuler 13: {}",worked_problems::euler_13::euler13()),
            "14" => println!("\nEuler 14: {}",worked_problems::euler_14::euler14()),
            "15" => println!("\nEuler 15: {}",worked_problems::euler_15::euler15()),
            "16" => println!("\nEuler 16: {}",worked_problems::euler_16::euler16()),
            "17" => println!("\nEuler 17: {}",worked_problems::euler_17::euler17()),
            "18" => println!("\nEuler 18: {}",worked_problems::euler_18::euler18()),
            "19" => println!("\nEuler 19: {}",worked_problems::euler_19::euler19()),
            "20" => println!("\nEuler 20: {}",worked_problems::euler_20::euler20()),
            "21" => println!("\nEuler 21: {}",worked_problems::euler_21::euler21()),
            "22" => println!("\nEuler 22: {}",worked_problems::euler_22::euler22()),
            "23" => println!("\nEuler 23: {}",worked_problems::euler_23::euler23()),
            "24" => println!("\nEuler 24: {}",worked_problems::euler_24::euler24()),
            "25" => println!("\nEuler 25: {}",worked_problems::euler_25::euler25()),
            "26" => println!("\nEuler 26: {}",worked_problems::euler_26::euler26()),
            "27" => println!("\nEuler 27: {}",worked_problems::euler_27::euler27()),
            "28" => println!("\nEuler 28: {}",worked_problems::euler_28::euler28()),
            "29" => println!("\nEuler 29: {}",worked_problems::euler_29::euler29()),
            "30" => println!("\nEuler 30: {}",worked_problems::euler_30::euler30()),
            "31" => println!("\nEuler 31: {}",worked_problems::euler_31::euler31()),
            "32" => println!("\nEuler 32: {}",worked_problems::euler_32::euler32()),
            "33" => println!("\nEuler 33: {}",worked_problems::euler_33::euler33()),
            "34" => println!("\nEuler 34: {}",worked_problems::euler_34::euler34()),
            "35" => println!("\nEuler 35: {}",worked_problems::euler_35::euler35()),
            "36" => println!("\nEuler 36: {}",worked_problems::euler_36::euler36()),
            "37" => println!("\nEuler 37: {}",worked_problems::euler_37::euler37()),
            "38" => println!("\nEuler 38: {}",worked_problems::euler_38::euler38()),
            "39" => println!("\nEuler 39: {}",worked_problems::euler_39::euler39()),
            "40" => println!("\nEuler 40: {}",worked_problems::euler_40::euler40()),
            "41" => println!("\nEuler 41: {}",worked_problems::euler_41::euler41()),
            "quit" => break,
            "q" => break,
            _ => println!("Haven't gotten to that one yet, sorry.")
        }
    }
}