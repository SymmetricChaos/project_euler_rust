// Problem: Find the minimal path sum from the top left to the bottom right by only moving right and down in the provided file.

/*

*/
use std::{
    fs,
    cell::Cell,
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    hash::{Hash, Hasher},
};


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

//https://codereview.stackexchange.com/questions/202677/dijkstras-algorithm-in-rust

// This vertex structure will make it possible to keep track of the distance through each node
// Cell is needed in order to have a shared mutable reference. We may need to change the distance through that node many times.
// Since the BinaryHeap that Rust provides is a max-heap the order relation is reversed to make it act as a min-heap.
// The Hash trait is necessary for using a vertex as a key in a HashMap or HashSet.
#[derive(Eq)]
struct Vertex<'a> {
    name: &'a str,
    distance: Cell<usize>,
    score: u32,
}

impl<'a> Vertex<'a> {
    fn new(i: usize, j: usize, score: u32) -> Vertex<'a> {
        Vertex {
            name: format!("({},{})",i,j),
            distance: Cell::new(usize::max_value()),
            score,
        }
    }
}

impl<'a> Hash for Vertex<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<'a> Ord for Vertex<'a> {
    fn cmp(&self, other: &Vertex<'a>) -> Ordering {
        other.distance.get().cmp(&self.distance.get())
    }
}

impl<'a> PartialOrd for Vertex<'a> {
    fn partial_cmp(&self, other: &Vertex<'a>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Vertex<'a> {
    fn eq(&self, other: &Vertex<'a>) -> bool {
        self.name == other.name
    }
}

// Returns a table of adjacencies
// Every position is linked to a vector that contains the adjacent positions and the distance to them
fn matrix_to_graph(mat: &Vec<Vec<u32>>, lim: usize) -> HashMap<Vertex,Vec<Vertex>> {
    let mut adjacency = HashMap::new();
    for i in 0..lim {
        for j in 0..lim {
            let v = Vertex::new(i,j,mat[i][j]);
            adjacency.insert(v,vec![]);
            if i+1 < lim {
                adjacency.get_mut(&v).unwrap().push( Vertex::new(i+1,j,mat[i+1][j]) )
            }
            if j+1 < lim {
                adjacency.get_mut(&v).unwrap().push( Vertex::new(i,j+1,mat[i][j+1]) )
            }
            
        }
    }
    adjacency
}



// Overview of what we need to do
// The verticies should be held in a BinaryHeap (the Rust implementation of a priority queue)
// To work properly it must be possible to sort the verticies
// We set the vertex [0,0] as the current position to start
// For each unvisisted neighbor check the total distance to them by going through the current position
// If we find a faster way to that neighbor than we previously knew set that distance as the new one
// From the BinaryHeap select the vertex with the shortest total distance
// If that vertex is [79,79] we're done otherwise it becomes the current position


// Dijkstra's algorithm should be much faster for a large matrix
fn dijkstra(adjacency_map: HashMap<Vertex,Vec<Vertex>>) {
    let mut to_visit = BinaryHeap::new();
    let mut visited = HashSet::<Vertex>::new();
    let mut distances = HashMap::<Vertex,usize>::new();
    adjacency_map.keys().for_each(|p| to_visit.push(p));



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