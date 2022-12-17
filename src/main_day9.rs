use std::collections::HashSet;
use std::fs;

// Direction from letter
fn str_to_move(move_str: &str) -> (i32, i32) {
    match move_str {
        "L" => (-1, 0),
        "R" => (1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => (0, 0),
    }
}

fn follow(head_pos: (i32, i32), mut tail_pos: (i32, i32)) -> (i32, i32) {

    // Check whether the head is more than 1 away from tail in any direction
    if (head_pos.0 - tail_pos.0).abs() > 1 || (head_pos.1 - tail_pos.1).abs() > 1 {
        // If not in the same row, move tail 1 closer in x direction
        if head_pos.0 > tail_pos.0 {
            tail_pos.0 += 1;
        } else if head_pos.0 < tail_pos.0 {
            tail_pos.0 -= 1;
        }
        // If not in the same column, move tail 1 closer in y direction
        if head_pos.1 > tail_pos.1 {
            tail_pos.1 += 1;
        } else if head_pos.1 < tail_pos.1 {
            tail_pos.1 -= 1;
        }
    }

    tail_pos
}


fn main() {
    let contents = fs::read_to_string("./input/day9.txt").expect("Should have been able to read the file");

    // Part 1: KNOT_NUMBER = 2, Part 2: KNOT_NUMBER = 10
    const KNOT_NUMBER: usize = 10;

    let mut rope: Vec<(i32, i32)> = Vec::new();

    for _i in 0..KNOT_NUMBER{
        rope.push((0,0));
    }

    // Set of visited positions
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();

    // Get movements as a vector of tuples, where each element contanis the direction and the number of times to move in that direction
    let moves: Vec<((i32, i32), u32)> = contents
        .trim()
        .split("\n")
        .map(|line| {
            (
                str_to_move(line.split(" ").nth(0).unwrap()),
                line.split(" ").nth(1).unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();

    // Iterate over all moves
    for (dir, times) in moves {
        // Repeat as many times as 
        for _i in 0..times {
            // Move head
            rope[0].0 += dir.0;
            rope[0].1 += dir.1;
            for i in 1..KNOT_NUMBER{
                rope[i] = follow(rope[i-1], rope[i]);
            }
            // Add current tail position to the set of visited positions
            tail_visited.insert(rope[KNOT_NUMBER-1]);
        }
    }
    
    // Total number of visited fields is the length of the set
    println!("Total number of fields visited: {}", tail_visited.len());

}
