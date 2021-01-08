//Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.

fn decimal_part_len(n: u64) -> u64 {
    let mut digits = Vec::new();
    let mut partials = Vec::new();
    partials.push(10);

    loop {
        // This section is the long division alorithm
        let p = partials.last().unwrap();
        let new_digit = p / n;
        let new_partial = (p % n)*10;
        digits.push(new_digit);

        // Here detect if we the cycle has begun to repeat or if the decimal expansion has
        // terminated. In any case we have to push the new partial value onto the vector.
        if partials.contains(&new_partial) {
            partials.push(new_partial);
            break;
        }
        if new_partial == 0 {
            partials.push(new_partial);
            break;
        }
        partials.push(new_partial);
    }

    // We always end with the last digit but the cycle may not start with the first digit
    // so we have to get the position where the cycle begins. Note for terminating decimals
    // there is no cycle and this is represented by the cycle both ending and starting with
    // the last digit, giving a cycle length of zero.
    let end = digits.len() as u64;
    let start = partials.iter().position(|x| x == partials.last().unwrap()).unwrap() as u64;
    return end-start as u64;
}

pub fn euler26() -> u64 {
    let mut longest = 0;
    let mut best = 1;
    for i in 2..1000 {
        if decimal_part_len(i) > longest {
            longest = decimal_part_len(i);
            best = i
        }
    }
    return best;
}