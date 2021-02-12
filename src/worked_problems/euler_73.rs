// Problem: How many fractions lie between 1/3 and 1/2 in the sorted set of reduced proper fractions for d ≤ 12,000?

/*
*/

pub fn euler73() -> u64 {
    let n = 12_000u16;

    let mut a = 1;
    let mut b = 3;
    let mut c = 1;
    let mut d = 2;

    loop {
        if b + d > n {
            break
        }
        c += a;
        d += b;
    }

    let mut ctr = 0;

    while (a,b) != (1,2) {
        ctr += 1;
        let k = (n+b)/d;
        let ct = c;
        let dt = d;
        c = (k*ct)-a;
        d = (k*dt)-b;
        a = ct;
        b = dt;
    }
    
    ctr-1
}


pub fn euler73_example() {
    println!("\nProblem: How many fractions lie between 1/3 and 1/2 in the sorted set of reduced proper fractions for d ≤ 12,000?");
    println!("\n\nThere aren't that many possibilities to loop through so this can be done by brute force. For a little efficiency we use the technique from problem 71 to immediately begin at 1/3 without computing the values up to it.");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler73());
}


#[test]
fn test73() {
    assert_eq!(euler73(),7295372)
}