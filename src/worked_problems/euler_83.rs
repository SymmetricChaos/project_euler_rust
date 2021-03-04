// Problem: Find the minimal path sum from the top left to the bottom right by moving left, right, up, and down in the provided file.

/*
See Problem 81
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
            if i > 0 {
                adjacency.get_mut(&v).unwrap().push( ((i-1,j),mat[i-1][j]) )
            }
            if j+1 < lim {
                adjacency.get_mut(&v).unwrap().push( ((i,j+1),mat[i][j+1]) )
            }
            if j > 0 {
                adjacency.get_mut(&v).unwrap().push( ((i,j-1),mat[i][j-1]) )
            }
        }
    }
    adjacency
}



fn dijkstra(start: (usize,usize), mat: &Vec<Vec<u32>>, adjacency_list: &HashMap<(usize,usize),Vec<((usize,usize),u32)>>) -> u32 {
    // Relate every vertex to a distance
    let mut distances = HashMap::new();
    // Track which verticies have been visited
    let mut visited = HashSet::new();
    // A heap to act as a priority queue
    let mut to_visit = BinaryHeap::new();

    distances.insert(start, mat[start.0][start.1]);

    to_visit.push(KnownVertex {
        name: start,
        distance: mat[start.0][start.1],
    });

    while let Some(KnownVertex { name, distance }) = to_visit.pop() {
        if !visited.insert(name) {
            // Already visited this node
            continue;
        }

        if let Some(neighbors) = adjacency_list.get(&name) {
            for (neighbor, cost) in neighbors {
                let new_distance = distance + cost;
                let is_shorter = distances
                    .get(&neighbor)
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    // This is our exit condition
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

pub fn euler83() -> u64 {
    let mat = read_data();
    let g = matrix_to_graph(&mat,80);
    dijkstra((0,0),&mat,&g) as u64
}

pub fn euler83_example() {
    println!("\nProblem: Find the minimal path sum from the top left to the bottom right by moving left, right, up, and down in the provided file.");
    println!("\n\nThe solution to this is almost identical to Problem 81. The only needed change is including all four directions in the adjacency map.");
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
            if i > 0 {
                adjacency.get_mut(&v).unwrap().push( ((i-1,j),mat[i-1][j]) )
            }
            if j+1 < lim {
                adjacency.get_mut(&v).unwrap().push( ((i,j+1),mat[i][j+1]) )
            }
            if j > 0 {
                adjacency.get_mut(&v).unwrap().push( ((i,j-1),mat[i][j-1]) )
            }
        }
    }
    adjacency
}

fn dijkstra(start: (usize,usize), mat: &Vec<Vec<u32>>, adjacency_list: &HashMap<(usize,usize),Vec<((usize,usize),u32)>>) -> u32 {
    // Relate every vertex to a distance
    let mut distances = HashMap::new();
    // Track which verticies have been visited
    let mut visited = HashSet::new();
    // A heap to act as a priority queue
    let mut to_visit = BinaryHeap::new();

    distances.insert(start, mat[start.0][start.1]);

    to_visit.push(KnownVertex {
        name: start,
        distance: mat[start.0][start.1],
    });

    while let Some(KnownVertex { name, distance }) = to_visit.pop() {
        if !visited.insert(name) {
            // Already visited this node
            continue;
        }

        if let Some(neighbors) = adjacency_list.get(&name) {
            for (neighbor, cost) in neighbors {
                let new_distance = distance + cost;
                let is_shorter = distances
                    .get(&neighbor)
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    // This is our exit condition
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

pub fn euler83() -> u64 {
    let mat = read_data();
    let g = matrix_to_graph(&mat,80);
    dijkstra((0,0),&mat,&g) as u64
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler83());
}


#[test]
fn test83() {
    assert_eq!(euler83(),425185)
}
