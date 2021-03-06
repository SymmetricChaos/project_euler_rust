// What is the value of the first triangle number to have over five hundred divisors?

fn num_factors(n: u32) -> u32 {
    let mut total = 2;
    let temp = n as f64;
    let lim = temp.sqrt().floor() as u32;
    
    for f in 2..lim {
       if n % f == 0 {
           if n / f != f {
                total += 2;
           } else {
               total += 1;
           }
       }
    }
    total
}

pub fn euler12() -> u32 {
    let mut triangle = 0;
    let mut ctr = 1;

    loop {
        triangle += ctr;
        if num_factors(triangle) > 500 {
            break;
        }
        ctr += 1;
    }
    triangle
}

pub fn euler12_example() {
    println!("\nProblem: What is the value of the first triangle number to have over five hundred divisors?");
    println!("\n\nTriangle numbers are easy to generate so counting factors the major challenge.");
    let s = "
fn num_factors(n: u32) -> u32 {
    let mut total = 2;
    let temp = n as f64;
    let lim = temp.sqrt().floor() as u32;
    
    for f in 2..lim {
        if n % f == 0 {
            if n / f != f {
                total += 2;
            } else {
                total += 1;
            }
        }
    }
    total
}

pub fn euler12() -> u32 {
    let mut triangle = 0;
    let mut ctr = 1;

    loop {
        triangle += ctr;
        if num_factors(triangle) > 500 {
            break;
        }
        ctr += 1;
    }
    triangle
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler12());
}

#[test]
fn test12() {
    assert_eq!(euler12(),76576500)
}