// Problem: Find the minimal path sum from the top left to the bottom right by only moving right and down in the provided file.

/*

*/
use std::fs;
use std::cmp::min;
use std::collections::{HashMap,BinaryHeap};

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


/*
// Simple recursive depth first search works for a small matrix
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
*/

// Returns a table of adjacencies
fn matrix_to_graph(mat: &Vec<Vec<u32>>, lim: usize) -> HashMap<(usize,usize),Vec<u32>> {
    let mut adjacency = HashMap::new();
    for i in 0..lim {
        for j in 0..lim {
            adjacency.insert((i,j),vec![]);
            if i+1 < lim {
                adjacency.get_mut(&(i,j)).unwrap().push(mat[i+1][j])
            }
            if j+1 < lim {
                adjacency.get_mut(&(i,j)).unwrap().push(mat[i][j+1])
            }
            
        }
    }
    adjacency
}

// Dijkstra's algorithm should be much faster for a large one
fn dijkstra() {

}




pub fn euler81() -> u64 {
    let mat = read_data();
    let mat_test = vec![
                    vec![131,673,234,103,18],
                    vec![201,96,342,965,150],
                    vec![630,803,746,422,111],
                    vec![537,699,497,121,956],
                    vec![805,732,524,37,331]
                    ];
    let g = matrix_to_graph(&mat_test,5);
    for r in g {
        println!("{:?}",r)
    }
    0u64
}

pub fn euler81_example() {
    println!("\nProblem: Find the minimal path sum from the top left to the bottom right by only moving right and down in the provided file.");
    println!("\n\nThis problem along with Problems 82 and 83 are all variations on the same theme, traversing a grid to find the path with the lowest sum. This is similar in presentation to Problem 67 but is probably better addressed as if finding a route. An ordinary search simply will not work on an 80x80 grid so instead a more efficient method is needed. I looked up information on implemented Dijsktra's algorithm.");
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