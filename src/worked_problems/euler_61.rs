// Problem: Find the sum of the only ordered set of six cyclic 4-digit numbers for which each polygonal type: triangle, square, pentagonal, hexagonal, heptagonal, and octagonal, is represented by a different number in the set.
/*

*/

use std::collections::HashMap;

fn figurate(s: u16, n: u16) -> u16 {
    if s == 3 {
        return n*(n+1) / 2
    }
    ( n*n*(s-2)-n*(s-4) ) / 2
}

fn all_four_digit_figurate(s: u16) -> Vec<u16> {
    let mut ctr = 0;
    let mut out = Vec::<u16>::new();
    loop {
        ctr += 1;
        let f = figurate(s,ctr);
        if f > 9999 {
            break
        }
        // Trim these out b/c they can't qualify
        if (f % 100) / 10 == 0 {
            continue
        }
        if f > 999 {
            out.push(f)
        }
    }
    out
}

fn start_map(nums: &Vec<u16>) -> HashMap<u16,Vec<u16>> {
    let mut map: HashMap<u16,Vec<u16>> = HashMap::new();
    for s in nums.iter() {
        let start = s/100;
        if map.contains_key(&start) {
            map.get_mut(&start).unwrap().push(*s)
        } else {
            map.insert(start,vec![*s]);
        }
    }
    map
}

pub fn euler61() -> u64 {
    let tri = all_four_digit_figurate(3);
    let sqr = all_four_digit_figurate(4);
    let pen = all_four_digit_figurate(5);
    let hex = all_four_digit_figurate(6);
    let sep = all_four_digit_figurate(7);
    let oct = all_four_digit_figurate(8);
    
    let tri_map = start_map(&tri);
    let sqr_map = start_map(&sqr);
    let pen_map = start_map(&pen);
    let hex_map = start_map(&hex);
    let sep_map = start_map(&sep);
    let oct_map = start_map(&oct);


    println!("{:?}\n\n{:?}\n\n{:?}\n\n{:?}\n\n{:?}\n\n{:?}",tri,sqr,pen,hex,sep,oct);


    //let sets = [&sqr,&pen,&hex,&sep,&oct];


    0u64
}

pub fn euler61_example() {
    println!("\nProblem: Find the sum of the only ordered set of six cyclic 4-digit numbers for which each polygonal type: triangle, square, pentagonal, hexagonal, heptagonal, and octagonal, is represented by a different number in the set.");
    println!("\n\n");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler61());
}


#[test]
fn test61() {
    assert_eq!(euler61(),28684)
}
