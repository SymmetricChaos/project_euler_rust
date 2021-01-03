// How many routes are there through a 20×20 grid starting in the top left corner and only being able to move to the right and down?

// This recursive algorithm works but is impractically slow
fn paths(x: u64, y: u64) -> u64 {
    match x == 0 || y == 0 {
        true => 1,
        false => paths(x-1,y) + paths(x,y-1),
    }
}

// 

pub fn euler15() -> u64 {
    
}
