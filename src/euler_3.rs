// What is the largest prime factor of the number 600851475143 ?

pub fn euler3() -> u64 {
    let mut m = 600851475143;
    let mut ctr = 2;

    // this assigns to 'out' whatever value the loop returns
    let out = loop {
        if ctr > m/2 {
            // this breaks the loop and returns the value of m
            break m;
        }
        while m % ctr == 0 {
            m /= ctr;
        }
        ctr += 1;
    };
    return out;
}