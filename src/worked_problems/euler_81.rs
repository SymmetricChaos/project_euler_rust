// Problem: Find the minimal path sum from the top left to the bottom right by only moving right and down in the provided file.

/*

*/
use std::{
    fs,
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};


// The sum will not overflow a u32
fn read_data() -> Vec<Vec<u32>> {
    let s = fs::read_to_string("files\\Euler81Doc.txt").unwrap();
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

// Instead of a Vertex type we will just use a pair (usize,usize) since we can get the information directly from the matrix

// This KnownVertex type will store a vertex that has been visisted before along with a distance through it
// Since the BinaryHeap that Rust provides is a max-heap the order relation is reversed to make it act as a min-heap.


#[derive(Eq)]
struct KnownVertex {
    name: (usize,usize),
    distance: u32,
}

impl PartialEq for KnownVertex {
    fn eq(&self, other: &Self) -> bool {
        other.distance.eq(&self.distance)
    }
}

impl Ord for KnownVertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for KnownVertex {
    fn partial_cmp(&self, other: &KnownVertex) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


// Returns a table of adjacencies
// Every position is linked to a vector that contains the adjacent positions and the distance to them
fn matrix_to_graph(mat: &Vec<Vec<u32>>, lim: usize) -> HashMap<(usize,usize),Vec<((usize,usize),u32)>> {
    let mut adjacency = HashMap::new();
    for i in 0..lim {
        for j in 0..lim {
            let v = (i,j);
            adjacency.insert(v,vec![]);
            if i+1 < lim {
                adjacency.get_mut(&v).unwrap().push( ((i+1,j),mat[i+1][j]) )
            }
            if j+1 < lim {
                adjacency.get_mut(&v).unwrap().push( ((i,j+1),mat[i][j+1]) )
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

fn dijkstra(adjacency_list: &HashMap<(usize,usize),Vec<((usize,usize),u32)>>) -> u32 {
    // Relate every vertex to a distance
    let mut distances = HashMap::new();
    // Track which verticies have been visited
    let mut visited = HashSet::new();
    // A heap to act as a priority queue
    let mut to_visit = BinaryHeap::new();

    
    distances.insert((0,0), 4445);

    to_visit.push(KnownVertex {
        name: (0,0),
        distance: 4445,
    });

    // The "while let" syntax means this will break if the destructure fails because we pop a None
    // The BinaryHeap ensures that we always work on the KnownVertex which has the shortest path from the starting point
    while let Some(KnownVertex { name, distance }) = to_visit.pop() {
        if !visited.insert(name) {
            // Already visited this node
            continue;
        }

        // The "if let" syntax skips this block if the assignment fails
        if let Some(neighbors) = adjacency_list.get(&name) {
            // If we find a shorter path to this neighbor than is known (or if no path is known)
            // then we update the distances HashMap and push this neighbor onto the to_visit BinaryHeap as a KnownVertex
            for (neighbor, cost) in neighbors {
                let new_distance = distance + cost;
                let is_shorter = distances
                    .get(&neighbor)
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    // Once we visit the target we're done
                    if *neighbor == (79,79) {
                        return new_distance
                    }
                    distances.insert(*neighbor, new_distance);
                    to_visit.push(KnownVertex {
                        name: *neighbor,
                        distance: new_distance,
                    });
                }
            }
        }
    }

    // We can never get here and this tells the copiler that.
    unreachable!()
}



pub fn euler81() -> u64 {
    let mat = read_data();
    let g = matrix_to_graph(&mat,80);
    dijkstra(&g) as u64
}

pub fn euler81_example() {
    println!("\nProblem: Find the minimal path sum from the top left to the bottom right by only moving right and down in the provided file.");
    println!("\n\nThis problem along with Problems 82 and 83 are all variations on the same theme, traversing a grid to find the path with the lowest sum. This is similar in presentation to Problem 67, however more complicated since the method of going in reverse is harder to apply. Traversing a grid to find the lowest sum is the same as traversing a graph to find the lowest sum, which can be done efficiently Dijsktra's algorithm. The code below is just for that algorithm and the adjacency matrix. To make it work the KnownVertex type needs to be implemented with a reversed Ord and PartialOrd to make the BinaryHeap act as a min-heap.");
    let s = "
fn matrix_to_graph(mat: &Vec<Vec<u32>>, lim: usize) -> HashMap<(usize,usize),Vec<((usize,usize),u32)>> {
    let mut adjacency = HashMap::new();
    for i in 0..lim {
        for j in 0..lim {
            let v = (i,j);
            adjacency.insert(v,vec![]);
            if i+1 < lim {
                adjacency.get_mut(&v).unwrap().push( ((i+1,j),mat[i+1][j]) )
            }
            if j+1 < lim {
                adjacency.get_mut(&v).unwrap().push( ((i,j+1),mat[i][j+1]) )
            }
            
        }
    }
    adjacency
}

fn dijkstra(adjacency_list: &HashMap<(usize,usize),Vec<((usize,usize),u32)>>) -> u32 {
    // Relate every vertex to a distance
    let mut distances = HashMap::new();
    // Track which verticies have been visited
    let mut visited = HashSet::new();
    // A heap to act as a priority queue
    let mut to_visit = BinaryHeap::new();

    
    distances.insert((0,0), 4445);

    to_visit.push(KnownVertex {
        name: (0,0),
        distance: 4445,
    });

    // The \"while let\" syntax means this will break if the destructure fails because we pop a None
    // The BinaryHeap ensures that we always work on the KnownVertex which has the shortest path from the starting point
    while let Some(KnownVertex { name, distance }) = to_visit.pop() {
        if !visited.insert(name) {
            // Already visited this node
            continue;
        }

        // Once we visit the target we're done
        if visited.contains(&(79,79)) {
            break
        }

        // The \"if let\" syntax skips this block if the assignment fails
        if let Some(neighbors) = adjacency_list.get(&name) {
            // If we find a shorter path to this neighbor than is known (or if no path is known)
            // then we update the distances HashMap and push this neighbor onto the to_visit BinaryHeap as a KnownVertex
            for (neighbor, cost) in neighbors {
                let new_distance = distance + cost;
                let is_shorter = distances
                    .get(&neighbor)
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    distances.insert(*neighbor, new_distance);
                    to_visit.push(KnownVertex {
                        name: *neighbor,
                        distance: new_distance,
                    });
                }
            }
        }
    }

    distances[&(79,79)]
}

pub fn euler81() -> u64 {
    let mat = read_data();
    let g = matrix_to_graph(&mat,80);
    dijkstra(&g) as u64
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler81());
}


#[test]
fn test81() {
    assert_eq!(euler81(),427337)
}
