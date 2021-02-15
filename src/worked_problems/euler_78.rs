// Problem: Find the least value of n for which the number of partitions of n is divisible by one million.
/*
*/


struct AllIntegers {
    ctr: i64,
    parity: u8,
} 

impl Iterator for AllIntegers {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        if self.parity == 0 {
            self.parity = 1;
            return Some(self.ctr)
        } else {
            self.parity = 0;
            self.ctr += 1;
            return Some(-(self.ctr-1))
        }
    }
}

fn gen_pentagonal(n: i64) -> i64 {
    (3*n*n-n)/2
}

pub fn euler78() -> u64 {
    let mut known_partitions = vec![1,1];
    let mut n = 2;

    let out = loop {
        let mut ctr = AllIntegers{ctr: 1, parity: 0};
        let mut p = gen_pentagonal(ctr.next().unwrap());
        let mut sum = 0;
        let mut sign = [1,1,-1,-1].iter().cycle();
        while p <= n {
            sum += known_partitions[(n-p) as usize]*sign.next().unwrap();
            p = gen_pentagonal(ctr.next().unwrap());
        }
        sum = sum % 1_000_000;
        if sum == 0 {
            break n
        }
        known_partitions.push(sum);
        n += 1
    };

    out as u64
}

pub fn euler78_example() {
    println!("\nProblem: Find the least value of n for which the number of partitions of n is divisible by one million.");
    println!("\n\nBorrowing the technique used for Problem 76 quickly gives an answer to this with the small chage that we reduce every sum modulo one million to avoiding having to work with enormous numbers. We can justify this on the fact that (a%m + b%m)%m = (a+b)%m.");
    let s = "
pub fn euler78() -> u64 {
    let mut known_partitions = vec![1,1];
    let mut n = 2;

    let out = loop {
        let mut ctr = AllIntegers{ctr: 1, parity: 0};
        let mut p = gen_pentagonal(ctr.next().unwrap());
        let mut sum = 0;
        let mut sign = [1,1,-1,-1].iter().cycle();
        while p <= n {
            sum += known_partitions[(n-p) as usize]*sign.next().unwrap();
            p = gen_pentagonal(ctr.next().unwrap());
        }
        sum = sum % 1_000_000;
        if sum == 0 {
            break n
        }
        known_partitions.push(sum);
        n += 1
    };

    out as u64
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler78());
}


#[test]
fn test78() {
    assert_eq!(euler78(),55374)
}