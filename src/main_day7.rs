use std::collections::HashMap;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input/day7.txt").expect("Should have been able to read the file");

    let mut direct_size: HashMap<String, u32> = HashMap::new();
    let mut total_size: HashMap<String, u32> = HashMap::new();
    let mut current_dir: Vec<String> = Vec::new();

    // Part 1
    for line in contents.trim().split("\n") {
        if line.eq("$ cd ..") {
            current_dir.pop();
        } else if line.starts_with("$ cd ") {
            current_dir.push(line.chars().skip(5).collect::<String>());
        } else if line.starts_with(|c: char| c.is_digit(10)) {
            let size = line.split(" ").nth(0).unwrap().parse::<u32>().unwrap();
            // let current_dir_str = ;
            *direct_size.entry(current_dir.join("/")).or_insert(0) += size;
        }
    }

    for (key, val) in direct_size.iter() {
        let mut dir_vec: Vec<String> = key.split("/").map(|x| x.to_string()).collect();
        while !dir_vec.is_empty() {
            // println!("{:?} {}", dir_vec.join("/"), *val);
            *total_size.entry(dir_vec.join("/")).or_insert(0) += *val;
            dir_vec.pop();
        }
    }

    // Part 1
    let sum_of_small_dirs =
        total_size.iter().fold(
            0,
            |acc, (_key, val)| if *val <= 100000 { acc + *val } else { acc },
        );
    println!("Total size of small directories: {:?}", sum_of_small_dirs);

    // Part 2
    let used_space = total_size["/"];
    let free_space = 70000000 - used_space;
    let required_space = 30000000 - free_space;
    let removed_dir_size = total_size.iter().fold(used_space, |acc, (_key, val)| {
        if *val >= required_space && *val <= acc {
            *val
        } else {
            acc
        }
    });
    println!("Size of removed directory: {}", removed_dir_size);
}
