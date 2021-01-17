// Find the pair of pentagonal numbers, Pj and Pk, for which their sum and difference are pentagonal and D = |Pk − Pj| is minimised; what is the value of D?

use num::integer::Roots;

fn is_pentagonal(n: u64) -> bool {
    let a = ((24*n+1).sqrt()+1)/6;
    if (3*a*a-a)/2 == n {
        return true
    }
    false
}

pub fn euler44() -> u64 {
    let mut out = 0;
    let mut pentagonal = vec![1];
    let mut n = 1;
    'outer: loop {
        n += 1;
        let p = (3*n*n-n)/2;        
        //println!("{}",p);
        for pent in &pentagonal {
            let sum = p+pent;
            let dif = p-pent;
            if pentagonal.contains(&dif) && is_pentagonal(sum) {
                println!("{} + {} = {}",p,pent,sum);
                println!("{} - {} = {}",p,pent,dif);
                out = dif;
                break 'outer
            }
        }
        pentagonal.push(p);
    }
    out
}