
use std::io;

mod rationals;
mod aux_funcs;
mod worked_problems;

fn make_message(input: &str) {
    let out_string = format!("\nSee Problem At: https://projecteuler.net/problem={}",input);
    let n = match input {
        "1" => worked_problems::euler_1::euler1().to_string(),
        "2" => worked_problems::euler_2::euler2().to_string(),
        "3" => worked_problems::euler_3::euler3().to_string(),
        "4" => worked_problems::euler_4::euler4().to_string(),
        "5" => worked_problems::euler_5::euler5().to_string(),
        "6" => worked_problems::euler_6::euler6().to_string(),
        "7" => worked_problems::euler_7::euler7().to_string(),
        "8" => worked_problems::euler_8::euler8().to_string(),
        "9" => worked_problems::euler_9::euler9().to_string(),
        "10" => worked_problems::euler_10::euler10().to_string(),
        "11" => worked_problems::euler_11::euler11().to_string(),
        "12" => worked_problems::euler_12::euler12().to_string(),
        "13" => worked_problems::euler_13::euler13().to_string(),
        "14" => worked_problems::euler_14::euler14().to_string(),
        "15" => worked_problems::euler_15::euler15().to_string(),
        "16" => worked_problems::euler_16::euler16().to_string(),
        "17" => worked_problems::euler_17::euler17().to_string(),
        "18" => worked_problems::euler_18::euler18().to_string(),
        "19" => worked_problems::euler_19::euler19().to_string(),
        "20" => worked_problems::euler_20::euler20().to_string(),
        "21" => worked_problems::euler_21::euler21().to_string(),
        "22" => worked_problems::euler_22::euler22().to_string(),
        "23" => worked_problems::euler_23::euler23().to_string(),
        "24" => worked_problems::euler_24::euler24().to_string(),
        "25" => worked_problems::euler_25::euler25().to_string(),
        "26" => worked_problems::euler_26::euler26().to_string(),
        "27" => worked_problems::euler_27::euler27().to_string(),
        "28" => worked_problems::euler_28::euler28().to_string(),
        "29" => worked_problems::euler_29::euler29().to_string(),
        "30" => worked_problems::euler_30::euler30().to_string(),
        "31" => worked_problems::euler_31::euler31().to_string(),
        "32" => worked_problems::euler_32::euler32().to_string(),
        "33" => worked_problems::euler_33::euler33().to_string(),
        "34" => worked_problems::euler_34::euler34().to_string(),
        "35" => worked_problems::euler_35::euler35().to_string(),
        "36" => worked_problems::euler_36::euler36().to_string(),
        "37" => worked_problems::euler_37::euler37().to_string(),
        "38" => worked_problems::euler_38::euler38().to_string(),
        "39" => worked_problems::euler_39::euler39().to_string(),
        "40" => worked_problems::euler_40::euler40().to_string(),
        "41" => worked_problems::euler_41::euler41().to_string(),
        _ => "Haven't gotten to that one yet, sorry.".to_string(),
    };

    println!("{}\nAnswer is: {}",out_string,n)
}

fn main() {
    println!("\nWelcome to My Project Euler Rust Project!");
    println!("Type \"quit\" to exit.");
    loop {
        println!("\n\nWhich Project Euler Problem Should I Calculate?");
        let mut val = String::new();
        io::stdin().read_line(&mut val).expect("Failed to read line");

        let v = val.trim();
            
        if v == "q" || v == "quit" {
            break
        }

        if !v.chars().all(char::is_numeric) {
            println!("\nERROR: Must input an integer or a valid command.");
            continue
        }

        make_message(v);
    }
}