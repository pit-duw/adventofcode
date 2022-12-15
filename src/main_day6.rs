use std::fs;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {

    let datastream: Vec<char> = fs::read_to_string("input/day6.txt")
    .expect("Should have been able to read the file").chars().collect();
    
    const MARKER_LENGTH: usize = 14;    // Part 1: 4, Part 2: 14

    let mut chars = VecDeque::new();
    for i in 0..MARKER_LENGTH {
        chars.push_back(datastream[i]);
    }
    for i in 4..datastream.len()-3 {
        let temp_set: HashSet<char> = chars.iter().map(|x| *x).collect();
        println!("{:?}", temp_set);
        if temp_set.len() == MARKER_LENGTH {
            println!("{}", i);
            break;
        } else {
            chars.pop_front();
            chars.push_back(datastream[i]);
        }
    }
}
