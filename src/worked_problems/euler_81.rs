// Problem: Find the minimal path sum from the top left to the bottom right by only moving right and down in the provided file.

/*

*/
use std::fs;
use std::cmp::min;
use std::collections::VecDeque;

// The sum will not overflow a u32
fn read_data() -> Vec<Vec<u32>> {
    let s = fs::read_to_string("Euler81Doc.txt").unwrap();
    let rows = s.split("\r\n");
    let mut vec = Vec::new();
    for r in rows {
        let elems = r.split(",");
        let v = elems.map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        vec.push(v);
    }
    vec
}

// WE CAN SOLVE IT AS A MAZE
// LOOK UP Dijkstra

// Simple depth first search works for a small matrix
fn depth_first_search(mat: &Vec<Vec<u32>>, lim: u32, pos: &[usize]) -> u32 {
    let cur = mat[pos[0]][pos[1]];
    let mut choices = vec![];
    if pos[0] + 1 < lim as usize {
        choices.push(cur + depth_first_search(mat, lim, &[pos[0]+1, pos[1]]))
    }
    if pos[1] + 1 < lim as usize {
        choices.push(cur + depth_first_search(mat, lim, &[pos[0], pos[1]+1]))
    }
    if choices.len() == 0 {
        return cur
    }
    return *choices.iter().min().unwrap()
}

// Dijkstra's algorithm should be much faster for a large one
fn best_first_search() {

}


pub fn euler81() -> u64 {
    let mat = read_data();
    let mat2 = vec![vec![131,673,234,103,18],
                    vec![201,96,342,965,150],
                    vec![630,803,746,422,111],
                    vec![537,699,497,121,956],
                    vec![805,732,524,37,331]];
    println!("{}",depth_first_search(&mat, 80, &[0,0]));
    0u64
}

pub fn euler81_example() {
    println!("\nProblem: Find the minimal path sum from the top left to the bottom right by only moving right and down in the provided file.");
    println!("\n\nThis problem along with Problems 82 and 83 are all variations on the same theme, traversing a grid to find the path with the lowest sum. This is similar in presentation to Problem 67 but is probably better addressed as solving a maze.");
    let s = "
";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler81());
}

/*
#[test]
fn test81() {
    assert_eq!(euler81(),)
}
*/