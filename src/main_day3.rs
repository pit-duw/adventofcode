use std::collections::HashSet;
use std::fs;

fn prio_from_char(c: char) -> u8 {
    if c.is_uppercase() {
        c as u8 - 38
    } else {
        c as u8 - 96
    }
}

// Part 1
fn prio_from_string(s: &str) -> i32 {
    let mut first_compartment = HashSet::new();
    for c in s.chars().take(s.len() / 2) {
        first_compartment.insert(c);
    }
    for c in s.chars().skip(s.len() / 2) {
        if first_compartment.contains(&c) {
            return prio_from_char(c) as i32;
        }
    }

    0
}

// Part 2
fn prio_from_string2(s: String) -> i32 {
    let mut first_pack = HashSet::new();
    let mut second_pack = HashSet::new();
    let mut third_pack = HashSet::new();
    for c in s.split("\n").nth(0).unwrap().chars() {
        first_pack.insert(c);
    }
    for c in s.split("\n").nth(1).unwrap().chars() {
        second_pack.insert(c);
    }
    for c in s.split("\n").nth(2).unwrap().chars() {
        third_pack.insert(c);
    }

    let intersection1: HashSet<char> = first_pack.intersection(&second_pack).map(|x| *x).collect();
    let intersection2: Vec<char> = intersection1
        .intersection(&third_pack)
        .map(|x| *x)
        .collect();
    prio_from_char(intersection2[0]) as i32
}

fn main() {
    let contents =
        fs::read_to_string("./input/day3.txt").expect("Should have been able to read the file");

    // Part 1
    let total_prio: i32 = contents.split("\n").map(|s| prio_from_string(s)).sum();

    // Part 2
    let String_vec: Vec<&str> = contents.trim().split("\n").collect();

    let group_vec: Vec<String> = String_vec.chunks(3).map(|s| s.join("\n")).collect();

    let total_prio2: i32 = group_vec
        .iter()
        .map(|s| prio_from_string2(s.to_string()))
        .sum();

    // cal_carried.sort_by(|a, b| b.cmp(a));
    // let cal_sum = cal_carried[0]+cal_carried[1]+cal_carried[2];

    println!("Total prio: {}", total_prio2);
}
