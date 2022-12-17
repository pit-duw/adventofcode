use std::fs;
// use std::collections::HashSet;

// Part 2
fn is_full_overlap(s: String) -> i32 {
    let string1 = s.split(",").nth(0).unwrap();
    let string2 = s.split(",").nth(1).unwrap();

    let start1 = string1.split("-").nth(0).unwrap().parse::<i32>().unwrap();
    let end1 = string1.split("-").nth(1).unwrap().parse::<i32>().unwrap();
    let start2 = string2.split("-").nth(0).unwrap().parse::<i32>().unwrap();
    let end2 = string2.split("-").nth(1).unwrap().parse::<i32>().unwrap();

    // Part 1
    // if (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1) {
    //     1
    // } else {
    //     0
    // }

    // Part 2
    if (start1 <= start2 && end1 >= start2)
        || (start2 <= start1 && end2 >= start1)
        || (start1 <= end2 && end1 >= end2)
        || (start2 <= end1 && end2 >= end1)
    {
        1
    } else {
        0
    }
}

fn main() {
    let contents =
        fs::read_to_string("./input/day4.txt").expect("Should have been able to read the file");

    // Part 1
    let total_overlaps: i32 = contents
        .trim()
        .split("\n")
        .map(|s| is_full_overlap(s.to_string()))
        .sum();

    println!("Total prio: {}", total_overlaps);
}
