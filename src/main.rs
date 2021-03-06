use std::io;

mod rationals;
mod aux_funcs;
mod worked_problems;

fn make_message(input: &str) {
    match input {
        "1" => worked_problems::euler_1::euler1_example(),
        "2" => worked_problems::euler_2::euler2_example(),
        "3" => worked_problems::euler_3::euler3_example(),
        "4" => worked_problems::euler_4::euler4_example(),
        "5" => worked_problems::euler_5::euler5_example(),
        "6" => worked_problems::euler_6::euler6_example(),
        "7" => worked_problems::euler_7::euler7_example(),
        "8" => worked_problems::euler_8::euler8_example(),
        "9" => worked_problems::euler_9::euler9_example(),
        "10" => worked_problems::euler_10::euler10_example(),
        "11" => worked_problems::euler_11::euler11_example(),
        "12" => worked_problems::euler_12::euler12_example(),
        "13" => worked_problems::euler_13::euler13_example(),
        "14" => worked_problems::euler_14::euler14_example(),
        "15" => worked_problems::euler_15::euler15_example(),
        "16" => worked_problems::euler_16::euler16_example(),
        "17" => worked_problems::euler_17::euler17_example(),
        "18" => worked_problems::euler_18::euler18_example(),
        "19" => worked_problems::euler_19::euler19_example(),
        "20" => worked_problems::euler_20::euler20_example(),
        "21" => worked_problems::euler_21::euler21_example(),
        "22" => worked_problems::euler_22::euler22_example(),
        "23" => worked_problems::euler_23::euler23_example(),
        "24" => worked_problems::euler_24::euler24_example(),
        "25" => worked_problems::euler_25::euler25_example(),
        "26" => worked_problems::euler_26::euler26_example(),
        "27" => worked_problems::euler_27::euler27_example(),
        "28" => worked_problems::euler_28::euler28_example(),
        "29" => worked_problems::euler_29::euler29_example(),
        "30" => worked_problems::euler_30::euler30_example(),
        "31" => worked_problems::euler_31::euler31_example(),
        "32" => worked_problems::euler_32::euler32_example(),
        "33" => worked_problems::euler_33::euler33_example(),
        "34" => worked_problems::euler_34::euler34_example(),
        "35" => worked_problems::euler_35::euler35_example(),
        "36" => worked_problems::euler_36::euler36_example(),
        "37" => worked_problems::euler_37::euler37_example(),
        "38" => worked_problems::euler_38::euler38_example(),
        "39" => worked_problems::euler_39::euler39_example(),
        "40" => worked_problems::euler_40::euler40_example(),
        "41" => worked_problems::euler_41::euler41_example(),
        "42" => worked_problems::euler_42::euler42_example(),
        "43" => worked_problems::euler_43::euler43_example(),
        "44" => worked_problems::euler_44::euler44_example(),
        "45" => worked_problems::euler_45::euler45_example(),
        "46" => worked_problems::euler_46::euler46_example(),
        "47" => worked_problems::euler_47::euler47_example(),
        "48" => worked_problems::euler_48::euler48_example(),
        "49" => worked_problems::euler_49::euler49_example(),
        "50" => worked_problems::euler_50::euler50_example(),
        "51" => worked_problems::euler_51::euler51_example(),
        "52" => worked_problems::euler_52::euler52_example(),
        "53" => worked_problems::euler_53::euler53_example(),
        "54" => worked_problems::euler_54::euler54_example(),
        "55" => worked_problems::euler_55::euler55_example(),
        "56" => worked_problems::euler_56::euler56_example(),
        "57" => worked_problems::euler_57::euler57_example(),
        "58" => worked_problems::euler_58::euler58_example(),
        "59" => worked_problems::euler_59::euler59_example(),
        "60" => worked_problems::euler_60::euler60_example(),
        "61" => worked_problems::euler_61::euler61_example(),
        "62" => worked_problems::euler_62::euler62_example(),
        "63" => worked_problems::euler_63::euler63_example(),
        "64" => worked_problems::euler_64::euler64_example(),
        "65" => worked_problems::euler_65::euler65_example(),
        "66" => worked_problems::euler_66::euler66_example(),
        "67" => worked_problems::euler_67::euler67_example(),
        "68" => worked_problems::euler_68::euler68_example(),
        "69" => worked_problems::euler_69::euler69_example(),
        "70" => worked_problems::euler_70::euler70_example(),
        "71" => worked_problems::euler_71::euler71_example(),
        "72" => worked_problems::euler_72::euler72_example(),
        "73" => worked_problems::euler_73::euler73_example(),
        "74" => worked_problems::euler_74::euler74_example(),
        "75" => worked_problems::euler_75::euler75_example(),
        "76" => worked_problems::euler_76::euler76_example(),
        "77" => worked_problems::euler_77::euler77_example(),
        "78" => worked_problems::euler_78::euler78_example(),
        "79" => worked_problems::euler_79::euler79_example(),
        "80" => worked_problems::euler_80::euler80_example(),
        "81" => worked_problems::euler_81::euler81_example(),
        "82" => worked_problems::euler_82::euler82_example(),
        "83" => worked_problems::euler_83::euler83_example(),
        "84" => worked_problems::euler_84::euler84_example(),
        "85" => worked_problems::euler_85::euler85_example(),
        "86" => worked_problems::euler_86::euler86_example(),
        "87" => worked_problems::euler_87::euler87_example(),
        "88" => worked_problems::euler_88::euler88_example(),
        "89" => worked_problems::euler_89::euler89_example(),
        "90" => worked_problems::euler_90::euler90_example(),
        "91" => worked_problems::euler_91::euler91_example(),
        _ => println!("Haven't gotten to that one yet, sorry."),
    };
}

fn main() {
    println!("\nWelcome to My Project Euler Rust Project!\nhttps://github.com/SymmetricChaos/project_euler_rust\nProblems 1 to 90 are available\n\nType \"quit\" to exit.");
    loop {
        println!("\n\nWhich Project Euler Problem Should I Show You?");
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
        println!("\n\n\nSee Problem At: https://projecteuler.net/problem={}",v);
        make_message(v);
    }
}