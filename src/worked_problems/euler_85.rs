// Problem: Although there exists no rectangular grid that contains exactly two million rectangles, find the area of the grid with the nearest solution.
/*
This has to have a combinatorial solution

For a 1xN grid we get triangular numbers
1x1 -> 1
1x2 -> 3
1x3 -> 6
1x4 -> 10
1x5 -> 15

This gives us a trivial upper bound on both dimension of 2000.

For every 1xA rectangle in a MxN grid there are corresponding rectangles of dimension 1xA through MxA

*/


pub fn euler85() -> u64 {
    let mut atri = 0i32;
    let mut closest = 2_000_000;
    let mut out = 0;
    for a in 1..=2000 {
        atri += a;
        let mut btri = 0i32;
        for b in 1..=2000 {
            btri += b;
            let rects = atri*btri;
            if (2_000_000-rects).abs() < closest {
                closest = (2_000_000-rects).abs();
                out = a*b
            }
            if rects > 2_000_000 {
                break
            }
        }
    }
    out as u64
}

pub fn euler85_example() {
    println!("\nProblem: Although there exists no rectangular grid that contains exactly two million rectangles, find the area of the grid with the nearest solution.");
    println!("\n\nFor a 1xN grid the number of rectangles is 1, 3, 6, 10, 15... the triangular numbers. So there is one such rectangle for each triangular number. In an MxN grid there are T(M) rectangles for each of the T(N) thin rectangle. So we can find the answer by multiplying pairs of triangular numbers.");
    let s = "
pub fn euler85() -> u64 {
    let mut atri = 0i32;
    let mut closest = 2_000_000;
    let mut out = 0;
    for a in 1..=2000 {
        atri += a;
        let mut btri = 0i32;
        for b in 1..=2000 {
            btri += b;
            let rects = atri*btri;
            if (2_000_000-rects).abs() < closest {
                closest = (2_000_000-rects).abs();
                out = a*b
            }
            if rects > 2_000_000 {
                break
            }
        }
    }
    out as u64
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler85());
}

#[test]
fn test85() {
    assert_eq!(euler85(),2772)
}
