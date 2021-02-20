// Problem: Find the minimal path sum from the left column to the right column by only moving up, down, and right in the provided file.

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

                // This is our exit condition
                if neighbor.1 == 79 {
                    return new_distance
                }                
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

    0u32
}

pub fn euler82() -> u64 {
    let mat = read_data();
    let g = matrix_to_graph(&mat,80);
    let mut attempts = vec![];
    for i in 0..80 {
        attempts.push(dijkstra((i,0),&mat,&g))
    }
    *attempts.iter().min().unwrap() as u64
}

pub fn euler82_example() {
    println!("\nProblem: Find the minimal path sum from the left column to the right column by only moving up, down, and right in the provided file.");
    println!("\n\nThe solution to this is essentially the same as for Problem 81, only the implementation of Dijkstra's algorithm needs to be lightly modified. We include the ability to specify a starting point and detect a slightly different exit condition.");
    let s = "
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

                
                // This is our exit condition
                if neighbor.1 == 79 {
                    return new_distance
                }                
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

    0u32
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler82());
}


#[test]
fn test82() {
    assert_eq!(euler82(),260324)
}
