use std::fs;

fn main() {
    let mut stacks: Vec<Vec<char>> = vec![
        vec!['B', 'P', 'N', 'Q', 'H', 'D', 'R', 'T'],
        vec!['W', 'G', 'B', 'J', 'T', 'V'],
        vec!['N', 'R', 'H', 'D', 'S', 'V', 'M', 'Q'],
        vec!['P', 'Z', 'N', 'M', 'C'],
        vec!['D', 'Z', 'B'],
        vec!['V', 'C', 'W', 'Z'],
        vec!['G', 'Z', 'N', 'C', 'V', 'Q', 'L', 'S'],
        vec!['L', 'G', 'J', 'M', 'D', 'N', 'V'],
        vec!['T', 'P', 'M', 'F', 'Z', 'C', 'G'],
    ]; 

    let contents = fs::read_to_string("input/day5.txt")
    .expect("Should have been able to read the file");

    let moves: Vec<&str> = contents.trim().split("\n").collect();
    for mv in moves {
        let parts: Vec<usize> = mv.split(" ").map(|x| x.parse().unwrap_or(0)).collect();
        println!("{}", mv);
        // Part 1
        // for _i in 0..parts[1]{
        //     let temp = stacks[parts[3]-1].pop().unwrap();
        //     stacks[parts[5]-1].push(temp);
        // }

        // Part 2
        let mut temp: Vec<char> = Vec::new();
        for _i in 0..parts[1]{
            temp.push(stacks[parts[3]-1].pop().unwrap());
        }
        for _i in 0..parts[1]{
            stacks[parts[5]-1].push(temp.pop().unwrap());
        }


        println!("{}", stacks[7].len());
    }

    for i in 0..9 {
        println!("{}", stacks[i].pop().unwrap()); 
    }

}
