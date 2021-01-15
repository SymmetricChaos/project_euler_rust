// For which value of p ≤ 1000, is the number of integer right triangles with perimeter p maximised?

use std::collections::HashMap;

pub fn euler39() -> u64 {

    let mut hmap = HashMap::<u64,u64>::new();
    for a in 1..1000 {
        for b in 1..1000-a {
            for c in  1..1000-a-b {
                if a+b+c > 1000 {
                    continue
                }
                if a*a + b*b == c*c {
                    if !hmap.contains_key(&(a+b+c)) {
                        hmap.insert(a+b+c,1);
                    } else {
                        *hmap.get_mut(&(a+b+c)).unwrap() += 1;
                    }
                }
            }
        }
    }
    let mut out = 0;
    let mut biggest = 0;
    for (k,v) in hmap.iter() {
        if *v > biggest {
            biggest = *v;
            out = *k;
        }
    }
    //println!("{:?}",hmap);
    out
}