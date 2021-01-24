// Problem: How many, not necessarily distinct, values of nCr for 1 <= n <= 100, are greater than one-million?

/*
Binomial coefficients have a varietyy of properties that we can exploit.
First: Each row is symmetric so knowing half of the row is the same as knowing all of the row
Second: Each row is strictly increasing up until the middle element

     1
    1 1
   1 2 1
  1 3 3 1
 1 4 6 4 1
*/


pub fn euler53() -> u64 {
    // Actually just a partial row
    let mut out = 0;
    let mut row = vec![0,1,0];
    for row_len in 1..101 {
        let mut next_row = vec![0];
        let mut lim = 0;
        for pos in 0..row.len()-1 {
            let coef = row[pos]+row[pos+1];
            next_row.push(coef);
            if coef > 1_000_000 {
                lim = pos;
                break
            }
        };
        next_row.push(0);
        row = next_row;
        if lim != 0 {
            // Need to use row_len+1 because we're always building and measuring the next row
            out += (row_len+1)-lim*2
        }
    }
    out as u64
}

pub fn euler53_example() {
    println!("\nProblem: How many, not necessarily distinct, values of nCr for 1 <= n <= 100, are greater than one-million?");
    println!("\n\nBinomial coefficients have a varietyy of properties that we can exploit. Each row is symmetric so knowing half of the row is the same as knowing all of the row. Since the values increase until the middle we also only need to work out coefficients up until they pass one million in each row, the rest can be found logically.");
    let s = "
pub fn euler53() -> u64 {
    // Actually just a partial row
    let mut out = 0;
    let mut row = vec![0,1,0];
    for row_len in 1..101 {
        let mut next_row = vec![0];
        let mut lim = 0;
        for pos in 0..row.len()-1 {
            let coef = row[pos]+row[pos+1];
            next_row.push(coef);
            if coef > 1_000_000 {
                lim = pos;
                break
            }
        };
        next_row.push(0);
        row = next_row;
        if lim != 0 {
            // Need to use row_len+1 because we're always building and measuring the next row
            out += (row_len+1)-lim*2
        }
    }
    out as u64
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler53());
}

#[test]
fn test53() {
    assert_eq!(euler53(),4075)
}