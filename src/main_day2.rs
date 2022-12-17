use std::fs;


const HANDS_OPP: [char; 3] = ['A', 'B', 'C'];
const HANDS_SELF: [char; 3] = ['X', 'Y', 'Z'];

fn score_from_string(s: &str) -> i32 {
    let mut score_line = 1;
    // s.to_string().retain(|c| !c.is_whitespace());
    // println!("{}", s.chars().nth(0).unwrap());
    let opp_pos = HANDS_OPP.iter()
                                  .position(|&x| x == s.chars().nth(0).unwrap())
                                  .unwrap() as i32;
    let self_pos = HANDS_SELF.iter()
                                    .position(|&x| x == s.chars().nth(2).unwrap())
                                    .unwrap() as i32;

    // Part 2
    score_line += self_pos*3;
    if self_pos == 0{
        score_line += (opp_pos+2)%3;
    } else if self_pos == 1{
        score_line += opp_pos;
    } else {
        score_line += (opp_pos+1)%3;
    }

    // Part 1
    // score_line += self_pos;
    // if self_pos == opp_pos{
    //     score_line += 3;
    // } else if self_pos == (opp_pos+1)%3{
    //     score_line += 6;
    // }

    println!("{}", score_line);
    score_line
}

fn main() {

    let contents = fs::read_to_string("./input/day2.txt")
        .expect("Should have been able to read the file");

    let score: i32 = contents.trim()
                             .split("\n")
                             .map(|s| score_from_string(s))
                             .sum();

    println!("Score total: {}", score);
}