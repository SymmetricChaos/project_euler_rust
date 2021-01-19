// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

fn factoradic(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut f = 1;
    let mut v = Vec::new();
    loop {
        if n == 0 {
            break;
        }
        v.push(n%f as u64);
        n /= f;
        f += 1
    };
    v
}

pub fn euler24() -> u64 {
    let mut v = factoradic(999_999);
    v.reverse();
    let mut elems = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut out = 0;
    let mut ctr = 10;
    for pos in v {
        ctr -= 1;
        out += elems.remove(pos as usize) * 10u64.pow(ctr);
    }
    out
}

#[test]
fn test24() {
    assert_eq!(euler24(),2783915460)
}