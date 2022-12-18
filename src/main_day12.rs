use std::collections::HashMap;
// use std::collections::HashSet;
use num::integer::lcm;
use std::fs;

fn main() {
    let datastream: String = fs::read_to_string("input/day12.txt")
        .expect("Should have been able to read the file")
        .chars()
        .collect();

    // Heights as numbers
    let mut height_arr: Vec<Vec<u32>> = datastream
        .trim()
        .split("\n")
        .map(|line| line.chars().map(|c| c as u32).collect())
        .collect();

    // Too lazy to parse from file, hard coding these
    let start_pos = (20, 0);
    let end_pos = (20, 43);

    // height of special fields
    height_arr[start_pos.0][start_pos.1] = 'a' as u32;
    height_arr[end_pos.0][end_pos.1] = 'z' as u32;

    let board_height = height_arr.len();
    let board_width = height_arr[0].len();
    // number of fields on the board + 1 -> distance cannot be larger -> can be used as initial distance estimate on all fields
    let mut max_steps = (board_width * board_height + 1) as u32;

    let mut low_points: Vec<(usize, usize)> = Vec::new();

    // Part 2
    for i in 0..board_height {
        for j in 0..board_width {
            if height_arr[i][j] == 'a' as u32 {
                low_points.push((i, j));
            }
        }
    }
    // Part 1:
    // low_points = vec![start_pos];

    let mut n = 0;

    // Brute force is fast enough for part 2 (~ 1min). Reverse searching from E to nearest a would be the elegant solution, but requires too much new code.
    for start in low_points {
        let line = std::vec::from_elem(max_steps, board_width);
        let mut steps_arr: Vec<Vec<u32>> = std::vec::from_elem(line, board_height);

        // Start field requires no steps
        steps_arr[start.0][start.1] = 0;

        // Fields reachable in n steps. Will be updated in each step of the upcoming for loop
        let mut reachable_arr: Vec<Vec<bool>> = steps_arr
            .iter()
            .map(|row| row.iter().map(|x| *x < max_steps).collect())
            .collect();

        // Step counter
        let mut step = 0;

        // Iterate until goal is found (or max_steps is reached => unreachable)
        while reachable_arr[end_pos.0][end_pos.1] == false && step < max_steps {
            let mut new_reachable_arr = reachable_arr.clone();
            for i in 0..board_height {
                for j in 0..board_width {
                    // Check all fields adjacent to a reachable field
                    if reachable_arr[i][j] == true {
                        if i > 0 {
                            new_reachable_arr[i - 1][j] = height_arr[i - 1][j] - 1
                                <= height_arr[i][j]
                                || new_reachable_arr[i - 1][j];
                        }
                        if j > 0 {
                            new_reachable_arr[i][j - 1] = height_arr[i][j - 1] - 1
                                <= height_arr[i][j]
                                || new_reachable_arr[i][j - 1];
                        }
                        if i < board_height - 1 {
                            new_reachable_arr[i + 1][j] = height_arr[i + 1][j] - 1
                                <= height_arr[i][j]
                                || new_reachable_arr[i + 1][j];
                        }
                        if j < board_width - 1 {
                            new_reachable_arr[i][j + 1] = height_arr[i][j + 1] - 1
                                <= height_arr[i][j]
                                || new_reachable_arr[i][j + 1];
                        }
                    }
                }
            }
            // Update reachable field array and increase step count
            reachable_arr = new_reachable_arr;
            step += 1;
        }
        max_steps = step;
        n += 1;
        println!("{}, {}", max_steps, n);
    }

    println!("{}", max_steps);
}
