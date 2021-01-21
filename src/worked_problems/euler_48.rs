// Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.

pub fn euler48() -> u64 {
    let m = 10_000_000_000;
    let mut out = 0;
    for n in 1..1000 {
        let mut a = 1;
        for _ in 0..n {
            a = (a*n) % m;
        }
        out += a;
    }
    out % m
}

pub fn euler48_example() {
    println!("\nProblem: Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.");
    println!("\n\nThere is no need for a big integer library here. We can simply discard the higher order digits using the modulus (remainder) operation.");
    let s = "
pub fn euler48() -> u64 {
    let m = 10_000_000_000;
    let mut out = 0;
    for n in 1..1000 {
        let mut a = 1;
        for _ in 0..n {
            a = (a*n) % m;
        }
        out += a;
    }
    out % m
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler48());
}

#[test]
fn test48() {
    assert_eq!(euler48(),9110846700)
}