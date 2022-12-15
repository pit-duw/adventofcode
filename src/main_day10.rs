use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;
use std::cmp;


fn main() {
    let datastream: String = fs::read_to_string("input/day10.txt")
        .expect("Should have been able to read the file")
        .chars()
        .collect();

    let commands: Vec<(String, i32)> = datastream.trim()
                                          .split("\n")
                                          .map(|line| (line.split(" ").nth(0).unwrap().to_string(), line.split(" ").nth(1).unwrap_or("0").parse::<i32>().unwrap() ))
                                          .collect();

    let mut signal_strength: i32 = 0;
    let mut register: i32 = 1;
    let mut sub_cycle: i32 = 0;
    let mut cycle: i32 = 20;
    let mut current_line: String = String::new();
    let mut crt_subcycle: i32 = 0;
    let mut repeat: i32 = 0;

    for com in commands {
        if com.0 == "noop" {
            repeat = 1;
        } else {
            repeat = 2;
        }
        for _i in 0..repeat{
            sub_cycle += 1;
            if crt_subcycle==register || crt_subcycle==register+1 || crt_subcycle==register-1{
                current_line.push_str("#");
            } else {
                current_line.push_str(".");
            }
            crt_subcycle += 1;
            if sub_cycle >= 20 {
                signal_strength += cycle*register;
                // println!("cycle: {}, register: {}, subcycle: {}, com.0: {}, com.1: {}, cycle*reg: {}", cycle, register, sub_cycle, com.0, com.1, cycle*register);
                sub_cycle -= 40;
                cycle += 40;
            }
            if crt_subcycle==40 {
                crt_subcycle -= 40;
                println!("{}", current_line);
                current_line = "".to_string();
            }
        } 
        register += com.1;
    }
    println!("{}", signal_strength);
}
