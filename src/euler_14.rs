// Which starting number, under one million, produces the longest Collatz chain?

use std::collections::HashMap;

fn collatz_step(n: u64) -> u64 {
    if n % 2 == 0 {
        return n/2;
    } else {
        return 3*n+1;
    }
}

pub fn euler14() -> u64 {
    let mut collatz = HashMap::<u64,u64>::new();
    let mut out = 0;
    let mut length = 0;
    collatz.insert(1,0);
    for i in 2..1_000_000 {
        let mut val = i as u64;
        let mut ctr = 0;
        while val != 1 {
            val = collatz_step(val);
            if collatz.contains_key(&val) {
                ctr += collatz[&val];
                break;
            } else {
                ctr += 1;
            }
        }
        collatz.insert(i,ctr);
        if ctr > length {
            length = ctr;
            out = i;
        }
    }
    return out;
}