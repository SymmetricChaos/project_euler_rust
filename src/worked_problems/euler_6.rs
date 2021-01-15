// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

pub fn euler6() -> u32 {
    let mut sum = 0;
    let mut sq_sum = 0;
    for n in 1..101 {
        sum += n;
        sq_sum += n*n;
    }
    sum *= sum;
    return sum-sq_sum;
}