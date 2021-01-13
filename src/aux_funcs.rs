pub fn int_to_digits(n: u64) -> Vec<u64> {
    let mut digits = Vec::new();
    let mut num = n;
    while num != 0 {
        let q = num/10;
        let r = num%10;
        digits.insert(0,r as u64);
        num = q;
    }
    return digits;
}

pub fn gcd(a: u64, b: u64) -> u64 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    return x;
}

pub fn pow_mod(n: u64, e: u64, m: u64) -> u64 {
    if e == 0 {
        return 1;
    }
    let mut out = n;
    for _ in 0..e-1 {
        out *= n;
        out = out % m;
    }
    out
}

// 64-bit primality test
// First checks small possible factors then switches to deterministic Miller-Rabin
pub fn is_prime(n: u64) -> bool {

    if n <= 1 {
        return false;
    }

    // Check small prime factors to quickly eliminate 80% of possible composite inputs
    for p in [2,3,5,7,11,13].iter() {
        if n == *p {
            return true;
        }
        if n % *p == 0 {
            //println!("{}|{}",*p,n);
            return false;
        }
    }

    let witnesses = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    
    let mut d = (n-1)/2;
    let mut r = 1;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }
    //println!("{} = 2^{} * {} + 1 {}",n,r,d,n == 2u64.pow(r)*d+1);
    
    'outer: for w in witnesses.iter() {
        let mut x = pow_mod(*w,d,n);
        
        if x == 1 || x == n-1 {
            continue 'outer;
        }
        
        for _ in 0..r-1 {
            x = pow_mod(x,2,n);
            
            if x == n-1 {
                 continue 'outer;               
            }
        }
        println!("{} eliminated by witness {}",n,*w);
        return false;
    }
    true
}