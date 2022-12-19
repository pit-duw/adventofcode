// use std::collections::HashMap;
// use std::collections::HashSet;
// use num::integer::lcm;
use std::fs;

fn is_num(input: &str) -> bool {
    !input.contains("[")
}

fn str_to_num(input: &str) -> u32 {
    input.parse::<u32>().unwrap()
}

fn get_nth_elem(input: &str, n: usize) -> String {
    let substr: String = input.chars().skip(1).take(input.len()-2).collect();
    let mut top_level_comma = 0;
    let mut new_substr: String = "".to_string();
    // We cannot just split at all ",", because then we would also split all inner lists.
    // Therefore we replace all "," in the outermost list with ";" then split the string at ";"
    for c in substr.chars(){
        let mut newchar = c;
        if c == '[' {
            top_level_comma += 1;
        } else if c == ']' {
            top_level_comma -= 1;
        } else if c == ',' && top_level_comma == 0 {
            newchar = ';';
        }
        new_substr.push(newchar);
    }
    // n-th element or empty string if it does not exist
    new_substr.split(";").nth(n).unwrap_or("").to_string()
}


// Possible outputs: 1 => is ordered, -1 => is not ordered, 0 is equal
fn is_ordered(left: &str, right: &str) -> i32 {
    // println!("left {}\nright{}", left, right);
    // Comparison in case of two numbers 
    if is_num(left) && is_num(right) {
        let left_num = str_to_num(left);
        let right_num = str_to_num(right);
        if left_num < right_num {
            return 1;
        } else if left_num > right_num {
            return -1;
        } else {
            return 0;
        }
    // Comparison if left is number and right is a list
    } else if is_num(left) && !is_num(right) {
        let new_right = get_nth_elem(right, 0);
        if new_right.is_empty() {
            -1
        } else {
            let order_temp = is_ordered(left, &new_right);
            // If the number and the first element of the list are equal we need to check whether the list has more elements left 
            if order_temp == 0 && !(get_nth_elem(right, 1).is_empty()) {
                return 1
            } else {
                order_temp
            }
        }
    // Comparison if right is a number and left is a list
    } else if !is_num(left) && is_num(right) {
        let new_left = get_nth_elem(left, 0);
        if new_left.is_empty() {
            1
        } else {
            let order_temp = is_ordered(&new_left, right);
            // If the number and the first element of the list are equal we need to check whether the list has more elements left 
            if order_temp == 0 && !(get_nth_elem(left, 1).is_empty()) {
                return -1
            } else {
                order_temp
            }
        }
    // Comparison if both sides are still lists
    } else {
        let mut order = 0;
        let mut n = 0;
        // Iterate until one sublist results in an order
        while order == 0{
            // Get next sublist on each side and compare them 
            let next_left = get_nth_elem(left, n);
            let next_right = get_nth_elem(right, n);
            if next_left.is_empty() && next_right.is_empty() {
                return 0;
            } else if next_left.is_empty() {
                return 1;
            } else if next_right.is_empty() {
                return -1;
            } else {
                order = is_ordered(&next_left, &next_right);
            }
            n += 1;
        }
        return order
    }
}



fn main() {
    let datastream: String = fs::read_to_string("input/day13.txt")
        .expect("Should have been able to read the file")
        .chars()
        .collect();

    // Part 1
    let pairs: Vec<(&str, &str)> = datastream.trim()
        .split("\n\n")
        .map(|part| (part.split("\n").nth(0).unwrap(), part.split("\n").nth(1).unwrap()))
        .collect();
 
    let mut sum = 0;
    
    for (i, pair) in pairs.iter().enumerate() {
        if is_ordered(pair.0, pair.1)  == 1{
            sum += i+1;
            // println!("{}", i+1);
        }
    }
    println!("{}", sum);

    // Part 2
    // We dont need to sort the whole list, figuring out how many elements are before each divider packet is sufficient.
    // Therefore we just compare each packet to the divider packets and sum how many are smaller
    let mut packets: Vec<&str> = Vec::new();
    for pair in pairs {
        packets.push(pair.0);
        packets.push(pair.1);
    }
    let divider1 = "[[2]]";
    let divider2 = "[[6]]";
    
    let mut decoder1 = 1;
    // Start at 2 to account for the smaller divider packet without explicit comparison
    let mut decoder2 = 2;
    for packet in packets{
        if is_ordered(packet, divider1) == 1{
            decoder1 += 1;
        }

        if is_ordered(packet, divider2) == 1{
            decoder2 += 1;
        }
    }
    println!("{}", decoder1 * decoder2);

}
