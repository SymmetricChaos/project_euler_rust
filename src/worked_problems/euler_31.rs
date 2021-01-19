// How many different ways can £2 be made using any number of coins?

fn restricted_partition(coins: &[u64], pos: usize, cur: u64) -> u64 {
    let mut total = 0;
    for (n,c) in coins.iter().enumerate() {
        if n < pos {
            // Skip any coins smaller than the one currently being used
            continue;
        }
        if *c > cur {
            // Do nothing if the coin type is too big
        } else if *c == cur {
            // If we reach exactly zero add one to the total
            total += 1;
        } else {
            // Otherwise continue to recur
            total += restricted_partition(coins,n,cur-c);
        }
    }
    total
}

pub fn euler31() -> u64 {
    let coins = [1,2,5,10,20,50,100,200];
    restricted_partition(&coins,0,200u64)
}

pub fn euler31_example() {
    println!("\nProblem: How many different ways can £2 be made using any number of coins? The coins are 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).");
    println!("\n\nThis is easy to do via recursion. The list of coins is an array and it passed as by reference for speed.");
    let s = "
fn restricted_partition(coins: &[u64], pos: usize, cur: u64) -> u64 {
    let mut total = 0;
    for (n,c) in coins.iter().enumerate() {
        if n < pos {
            // Skip any coins smaller than the one currently being used
            continue;
        }
        if *c > cur {
            // Do nothing if the coin type is too big
        } else if *c == cur {
            // If we reach exactly zero add one to the total
            total += 1;
        } else {
            // Otherwise continue to recur
            total += restricted_partition(coins,n,cur-c);
        }
    }
    total
}

pub fn euler31() -> u64 {
    let coins = [1,2,5,10,20,50,100,200];
    restricted_partition(&coins,0,200u64)
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler31());
}

#[test]
fn test31() {
    assert_eq!(euler31(),73682)
}