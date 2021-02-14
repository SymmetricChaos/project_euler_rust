// Problem: How many different ways can one hundred be written as a sum of at least two positive integers?
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

pub fn euler76() -> u64 {
    let mut known_partitions = [0i64;101];
    known_partitions[0] = 1;
    known_partitions[1] = 1;

    for n in 2..=100 {
        let mut ctr = AllIntegers{ctr: 1, parity: 0};
        let mut p = gen_pentagonal(ctr.next().unwrap());
        let mut sum = 0;
        let mut sign = [1,1,-1,-1].iter().cycle();
        while p <= n {
            sum += known_partitions[(n-p) as usize]*sign.next().unwrap();
            p = gen_pentagonal(ctr.next().unwrap());
        }
        known_partitions[n as usize] = sum
    }

    // We need to subtract 1 because the partitions we calculated include the number itself as a partition
    (known_partitions[100]-1) as u64
}


pub fn euler76_example() {
    println!("\nProblem: How many different ways can one hundred be written as a sum of at least two positive integers?");
    println!("\n\nThis could possibly be done using brute force to calculate partitions of each integer and then memoizing the results. However Euler's Pentagonal Number Theorem gives us a much more efficient option where we don't need to brute force calculate any partitions except for 0 and 1, which both have a single partition.");
    let s = "
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

pub fn euler76() -> u64 {
    let mut known_partitions = [0i64;101];
    known_partitions[0] = 1;
    known_partitions[1] = 1;

    for n in 2..=100 {
        let mut ctr = AllIntegers{ctr: 1, parity: 0};
        let mut p = gen_pentagonal(ctr.next().unwrap());
        let mut sum = 0;
        let mut sign = [1,1,-1,-1].iter().cycle();
        while p <= n {
            sum += known_partitions[(n-p) as usize]*sign.next().unwrap();
            p = gen_pentagonal(ctr.next().unwrap());
        }
        known_partitions[n as usize] = sum
    }

    // We need to subtract 1 because the partitions we calculated include the number itself as a partition
    (known_partitions[100]-1) as u64
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler76());
}


#[test]
fn test76() {
    assert_eq!(euler76(),190569291)
}