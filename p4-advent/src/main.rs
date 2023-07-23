use std::{num::ParseIntError, env::args};

fn main() -> Result<(), ParseIntError> {
    let input = args().collect::<Vec<String>>();

    let parsed_input : i32 = input[1].parse()?;

    assert!(parsed_input > 0);

    let position = get_spiral_position(parsed_input);
    let dist = manhattan_dist(position, (0, 0));
    
    println!("Position on spiral: {:?}", position);
    println!("Manhattan Distance to origin: {}", dist);

    Ok(())
}

type Point = (i32, i32);

/// Given a number
/// calculates the point (x, y) on a right-handed ccw spiral 
/// relative to the origin 1 at (0,0)
fn get_spiral_position(input: i32) -> Point {
    let lsqr = (input as f32).sqrt() as i32 + 1;
    let square_diff = lsqr.pow(2) - (lsqr - 1).pow(2);
    let diff = lsqr.pow(2) - input;

    let gt_half_min = diff.min(square_diff / 2);
    let lt_half_max = (diff - square_diff / 2).max(0);

    match lsqr % 2 {
        0 => (
            (1 - lsqr / 2) + gt_half_min,
            lsqr / 2 - lt_half_max
        ),
        1 => (
            (lsqr - 1) / 2 - gt_half_min,
            (lsqr - 1) / -2 + lt_half_max
        ),
        _ => panic!("What number could this be??")
    }
}

/// Calculate the Manhattan Distance between two points
/// https://en.wikipedia.org/wiki/Taxicab_geometry
fn manhattan_dist(p: Point, q: Point) -> i32 {
    let (p1, p2) = p;
    let (q1, q2) = q;

    (p1 - q1).abs() + (p2 - q2).abs()
}