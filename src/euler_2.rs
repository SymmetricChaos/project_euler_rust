// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.

// Struct with the two terms of a Fibonnacci sequence and a small integer
// to denote how many intermediate values to ignore
struct SkipFib {
    a: u32,
    b: u32,
    skips: u8,
}

// Give the struc the ability to iterate
impl Iterator for SkipFib {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        for _i in 0..self.skips+1 {
            let new = self.a + self.b;
            self.a = self.b;
            self.b = new;
        }
        //emit Some(number) or None if the iterator terminates
        Some(self.a)
    }
}

// Start at (0,1) and skip two terms before emitting a new value
// This saves comutation of checking if each term is even since 
// the Fibonnacci sequence has partity 0,1,1,0,1,1,0,1,1...
pub fn euler2() -> u32 {
    let mut f = SkipFib {a: 0, b: 1, skips: 2};
    let mut out = 0;
    let mut cur = 0;
    while cur < 4_000_000 {
        out += cur;
        cur = match f.next() {
            Some(number) => number,
            None => 0
        }
    }
    return out;
}