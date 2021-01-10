pub fn int_to_digits(n: u64) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut num = n;
    while num != 0 {
        let q = num/10;
        let r = num%10;
        digits.insert(0,r as u8);
        num = q;
    }
    return digits;
}