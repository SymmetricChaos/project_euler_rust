// By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers.
// Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

fn aliquot_sum(n:u64) -> u64 {
    if n == 0 {
        return 0u64;
    }
    let lim = (n as f64).sqrt().floor() as u64;
    let mut out = 1;
    for f in 2..lim+1 {
        if n % f == 0 {
            if f != n/f {
                out += f + n/f;
            } else {
                out += f;
            }
        }
    }
    out
}


// Get all the abundant numbers less than 28124
fn abudant_numbers() -> Vec<u64> {
    let mut v = Vec::new();
    for n in 1..28124 {
        if aliquot_sum(n) > n {
            v.push(n);
        }
    }
    v
}


/*
fn is_ab_sum(n: u64, abundants: &Vec<u64>) -> bool {
    for a in abundants {
        if n >= *a {
            if abundants.contains(&(n-*a)) {
                return true;
            }
        } else {
            return false;
        }
    }
    return false;
}
*/


pub fn euler23() -> u64 {
    let mut out = 0u64;
    let abundants = abudant_numbers();

    /*
    //For some reason this loop is extremely slow probably due to the subtractions and nested
    //if statements in is_ab_sum function
    for n in 1..28124 {
        if !is_ab_sum(n,&abundants) {
            //println!("{}",n);
            out += n;
        }
    }
    */

    let mut arr: [u64; 28124] = [0; 28124];
    for i in 0..28123 {
        arr[i] = i as u64
    }
    for a in &abundants {
        for b in &abundants {
            if a+b > 28123 {
                break;
            } else {
                arr[(a+b) as usize] = 0;
            }
        }
    }
    for a in &arr {
        out += a
    }
    out
}

pub fn euler23_example() {
    println!("\nProblem: By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers. Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.");
    println!("\n\n");
    let s = "
fn aliquot_sum(n:u64) -> u64 {
    if n == 0 {
        return 0u64;
    }
    let lim = (n as f64).sqrt().floor() as u64;
    let mut out = 1;
    for f in 2..lim+1 {
        if n % f == 0 {
            if f != n/f {
                out += f + n/f;
            } else {
                out += f;
            }
        }
    }
    out
}

// Get all the abundant numbers less than 28124
fn abudant_numbers() -> Vec<u64> {
    let mut v = Vec::new();
    for n in 1..28124 {
        if aliquot_sum(n) > n {
            v.push(n);
        }
    }
    v
}

pub fn euler23() -> u64 {
    let mut out = 0u64;
    let abundants = abudant_numbers();    
    let mut arr: [u64; 28124] = [0; 28124];
    for i in 0..28123 {
        arr[i] = i as u64
    }
    for a in &abundants {
        for b in &abundants {
            if a+b > 28123 {
                break;
            } else {
                arr[(a+b) as usize] = 0;
            }
        }
    }
    for a in &arr {
        out += a
    }
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler23());
}

#[test]
fn test23() {
    assert_eq!(euler23(),4179871)
}