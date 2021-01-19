// What is the largest prime factor of the number 600851475143 ?

pub fn euler3() -> u64 {
    let mut m = 600851475143;
    let mut ctr = 2;

    let out = loop {
        if ctr > m/2 {
            break m;
        }
        while m % ctr == 0 {
            m /= ctr;
        }
        ctr += 1;
    };
    out
}

pub fn euler3_example() {
    println!("\nProblem: What is the largest prime factor of the number 600851475143");
    println!("\n\nSince the number in question is \"small\" it is sufficient to simply test factors one by one.");
    let s = "pub fn euler3() -> u64 {
    let mut m = 600851475143;
    let mut ctr = 2;

    let out = loop {
        if ctr > m/2 {
            break m;
        }
        while m % ctr == 0 {
            m /= ctr;
        }
        ctr += 1;
    };
    out
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler3());
}


#[test]
fn test3() {
    assert_eq!(euler3(),6857)
}