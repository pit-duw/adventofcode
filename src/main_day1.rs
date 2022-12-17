use std::fs;

fn sum_from_string(s: &str) -> i32 {
    let output = s
        .split("\n")
        .map(|substring| substring.parse::<i32>().unwrap_or(0))
        .sum();
    println!("{}", output);
    output
}

fn main() {
    let contents = fs::read_to_string("../../Downloads/input.txt")
        .expect("Should have been able to read the file");

    let mut cal_carried: Vec<i32> = contents.split("\n\n").map(|s| sum_from_string(s)).collect();
    cal_carried.sort_by(|a, b| b.cmp(a));
    let cal_sum = cal_carried[0] + cal_carried[1] + cal_carried[2];

    println!("Top 3 cal total: {}", cal_sum);
}
