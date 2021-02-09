// Problem: Using the numbers 1 to 10, and depending on arrangements, it is possible to form 16- and 17-digit strings. What is the maximum 16-digit string for a "magic" 5-gon ring?

/*
Due to the details of the problem we know that 10 must be on one of the outer parts of the ring.
We do not need to seperately create solutions that are just rotations of each other as the rules make these equivalent.
These two facts mean we can always start with 10.
*/


    // Fill first group starting with 10, a, b
    // Mark the sum 10 + a + b as 'ring_const'
    // Fill the second row with c, b, (c+b-ring_const)
    // If that step fails try different values of c
    // If a working value is found continue
    // If a working value is not found go back to the first group
    // If the second group has been fill the third group with d, (c+b-ring_const), (d+(c+b-ring_const)-ring_const)
    // So on and so forth.
    // Last row needs to check only the outer number and sum since the middle and last are already filled
    // There are only six free variables
    // We select six without repetition and without replacement from a set of nine options
    // There are thus only 60480 possibilities to consider
    


use itertools::Itertools;

fn fill_5_gon() -> Vec<Vec<Vec<u8>>> {

    let mut out = Vec::new();
    let perms = (1..10).permutations(6);

    for p in perms {
        // Track the already used numbers, we always use 10 to start with
        let mut used = vec![10];

        // Build row1
        let a = p[0];
        let b = p[1];
        let row1 =  vec![10,a,b];
        let ring_const: u8 = row1.iter().sum();
        used.push(a);
        used.push(b);

        // Build row2
        let c = p[2];
        used.push(c);
        let d = match ring_const.checked_sub(c+b) {
            Some(num) => num,
            None => continue,
        };
        if used.contains(&d) || d < 1 || d > 10 {
            continue
        }
        used.push(d);
        let row2 =  vec![c,b,d];

        // Build row3
        let e = p[3];
        if used.contains(&e) {
            continue
        }
        used.push(e);
        let f = match ring_const.checked_sub(e+d) {
            Some(num) => num,
            None => continue,
        };
        if used.contains(&f) || f < 1 || f > 10 {
            continue
        }
        used.push(f);
        let row3 =  vec![e,d,f];

        // Build row4
        let g = p[4];
        if used.contains(&g) {
            continue
        }
        used.push(g);
        let h = match ring_const.checked_sub(g+f) {
            Some(num) => num,
            None => continue,
        };
        if used.contains(&h) || h < 1 || h > 10 {
            continue
        }
        used.push(h);
        let row4 = vec![g,f,h];

        // Build row5
        let i = p[5];
        if used.contains(&i) {
            continue
        }
        let row5 =  vec![i,h,a];
        if row5.iter().sum::<u8>() != ring_const {
            continue
        }

        out.push(vec![row1,row2,row3,row4,row5]);
    }
    out
}

fn solution_to_num(solution: &Vec<Vec<u8>>) -> u64 {
    let firsts = solution.iter().map(|x| x[0]).collect::<Vec<u8>>();
    let lowest = firsts.iter().min().unwrap();
    let mut s = solution.clone();
    while s[0][0] != *lowest {
        let t = s.remove(0);
        s.push(t)
    }
    let mut digits = Vec::new();
    for i in 0..5 {
        for j in 0..3 {
            digits.push((s[i][j]).to_string())
        }
    }
    let d = digits.join("");
    d.parse::<u64>().unwrap()
}

pub fn euler68() -> u64 {
    let solutions = fill_5_gon();
    let mut biggest = 0;
    for s in solutions {
        let num = solution_to_num(&s);
        if num > biggest {
            biggest = num
        }
    }
    biggest
}

pub fn euler68_example() {
    println!("\nProblem: Using the numbers 1 to 10, and depending on arrangements, it is possible to form 16- and 17-digit strings. What is the maximum 16-digit string for a \"magic\" 5-gon ring?");
    println!("\n\nThis seems like a complicated problem but there are actually a lot of constraints provided by the problem and I encourage reading the whole page. Since we need a 16-digit string the 10 must appear on the outside. Next observe that we can fill in the first two spaces of that row with whatever we like. The sum of those terms gives us the constant for the whole ring. Knowing this constant and that each row overlaps with the next there is only one choice to be made for each later row. So with only 6 choices to make and only 9 options from there are just 60480 possiblities to check. With a little algebra the problem can then quickly be solved.");
    let s = "
use itertools::Itertools;

fn fill_5_gon() -> Vec<Vec<Vec<u8>>> {

    let mut out = Vec::new();
    let perms = (1..10).permutations(6);

    for p in perms {
        // Track the already used numbers, we always use 10 to start with
        let mut used = vec![10];

        // Build row1
        let a = p[0];
        let b = p[1];
        let row1 =  vec![10,a,b];
        let ring_const: u8 = row1.iter().sum();
        used.push(a);
        used.push(b);

        // Build row2
        let c = p[2];
        used.push(c);
        let d = match ring_const.checked_sub(c+b) {
            Some(num) => num,
            None => continue,
        };
        if used.contains(&d) || d < 1 || d > 10 {
            continue
        }
        used.push(d);
        let row2 =  vec![c,b,d];

        // Build row3
        let e = p[3];
        if used.contains(&e) {
            continue
        }
        used.push(e);
        let f = match ring_const.checked_sub(e+d) {
            Some(num) => num,
            None => continue,
        };
        if used.contains(&f) || f < 1 || f > 10 {
            continue
        }
        used.push(f);
        let row3 =  vec![e,d,f];

        // Build row4
        let g = p[4];
        if used.contains(&g) {
            continue
        }
        used.push(g);
        let h = match ring_const.checked_sub(g+f) {
            Some(num) => num,
            None => continue,
        };
        if used.contains(&h) || h < 1 || h > 10 {
            continue
        }
        used.push(h);
        let row4 = vec![g,f,h];

        // Build row5
        let i = p[5];
        if used.contains(&i) {
            continue
        }
        let row5 =  vec![i,h,a];
        if row5.iter().sum::<u8>() != ring_const {
            continue
        }

        out.push(vec![row1,row2,row3,row4,row5]);
    }
    out
}

fn solution_to_num(solution: &Vec<Vec<u8>>) -> u64 {
    let firsts = solution.iter().map(|x| x[0]).collect::<Vec<u8>>();
    let lowest = firsts.iter().min().unwrap();
    let mut s = solution.clone();
    while s[0][0] != *lowest {
        let t = s.remove(0);
        s.push(t)
    }
    let mut digits = Vec::new();
    for i in 0..5 {
        for j in 0..3 {
            digits.push((s[i][j]).to_string())
        }
    }
    let d = digits.join(\"\");
    d.parse::<u64>().unwrap()
}

pub fn euler68() -> u64 {
    let solutions = fill_5_gon();
    let mut biggest = 0;
    for s in solutions {
        let num = solution_to_num(&s);
        if num > biggest {
            biggest = num
        }
    }
    biggest
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler68());
}


#[test]
fn test68() {
    assert_eq!(euler68(),6531031914842725)
}