// Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?

/*
73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450
*/

use std::fs;

pub fn euler8() -> u64 {
    let mut biggest_num = 0u64;
    let s = fs::read_to_string("Euler8Doc.txt").unwrap();
    for i in 0..988 {
        let substring = &s[i..i+13];
        if substring.contains("0") {
            // we can ignore any substring that contains zero
        } else {
            // we must use u64 because the maximum value that might
            // occur is 9^13 which is greaterthan any u32
            let mut temp = 1u64;
            for c in substring.chars() {
                let t = c.to_digit(10).unwrap();
                temp *= t as u64;
            }
            if temp > biggest_num {
                biggest_num = temp;
            }
        }
    }
    biggest_num
}

pub fn euler8_example() -> u64 {
    println!("\nFind the thirteen adjacent digits in the 1000-digit number provided that have the greatest product. What is the value of this product?");
    println!("\nThe main challenge here is reading the provided file.");
    let s = "
use std::fs;

pub fn euler8() -> u64 {
    let mut biggest_num = 0u64;
    let s = fs::read_to_string(\"Euler8Doc.txt\").unwrap();
    for i in 0..988 {
        let substring = &s[i..i+13];
        if substring.contains(\"0\") {
            // we can ignore any substring that contains zero
        } else {
            // we must use u64 because the maximum value that might
            // occur is 9^13 which is greaterthan any u32
            let mut temp = 1u64;
            for c in substring.chars() {
                let t = c.to_digit(10).unwrap();
                temp *= t as u64;
            }
            if temp > biggest_num {
                biggest_num = temp;
            }
        }
    }
    biggest_num
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler8());
    0u64
}

#[test]
fn test8() {
    assert_eq!(euler8(),23514624000)
}
