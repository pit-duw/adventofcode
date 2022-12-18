// use std::cmp;
use std::collections::HashMap;
// use std::collections::HashSet;
use num::integer::lcm;
use std::fs;

fn main() {
    let datastream: String = fs::read_to_string("input/day11.txt")
        .expect("Should have been able to read the file")
        .chars()
        .collect();

    // Part 1: 3, Part 2: 1
    const WORRY_DIVISOR: u64 = 1;
    const ROUNDS: u64 = 10000;

    let monkeys: Vec<&str> = datastream.trim().split("\n\n").collect();

    let mut monkey_inventories: HashMap<usize, Vec<u64>> = HashMap::new();
    let mut monkey_operation: HashMap<usize, (u64, u64)> = HashMap::new();
    let mut monkey_test: HashMap<usize, u64> = HashMap::new();
    let mut monkey_targets: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut monkey_inspect_count: HashMap<usize, u64> = HashMap::new();

    // Initial conditions and behavior
    for (id, monkey) in monkeys.into_iter().enumerate() {
        monkey_inventories.insert(id, Vec::new());
        let lines: Vec<&str> = monkey.split("\n").collect();
        // println!("{}", lines[1]);
        let inventory: Vec<u64> = lines[1]
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        for item in inventory {
            monkey_inventories.entry(id).and_modify(|v| v.push(item));
        }
        let op = match lines[2]
            .split(" = old ")
            .nth(1)
            .unwrap()
            .chars()
            .nth(0)
            .unwrap()
        {
            '*' => 0,
            '+' => 1,
            _ => 2,
        };
        let num: u64 = lines[2]
            .split(" = old ")
            .nth(1)
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap_or(0);
        monkey_operation.insert(id, (op, num));
        monkey_test.insert(
            id,
            lines[3]
                .split("divisible by ")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap(),
        );
        monkey_targets.insert(
            id,
            (
                lines[4]
                    .split("throw to monkey ")
                    .nth(1)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
                lines[5]
                    .split("throw to monkey ")
                    .nth(1)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            ),
        );
        monkey_inspect_count.insert(id, 0);
    }
    let lowest_common_multiple = monkey_test.values().fold(1, |acc, val| lcm(acc, *val));

    // Iterate over the twenty rounds
    for _rnd in 0..ROUNDS {
        for id in 0..monkey_inventories.len() {
            let mut moves: Vec<(u64, usize)> = Vec::new();
            for item in monkey_inventories.get_mut(&id).unwrap() {
                let mut num = monkey_operation.get(&id).unwrap().1;
                // handle "old * old" operation
                if num == 0 {
                    num = *item;
                }
                //
                if monkey_operation.get(&id).unwrap().0 == 0 {
                    *item *= num;
                    *item /= WORRY_DIVISOR;
                    *item %= lowest_common_multiple;
                }
                //
                if monkey_operation.get(&id).unwrap().0 == 1 {
                    *item += num;
                    *item /= WORRY_DIVISOR;
                    *item %= lowest_common_multiple;
                }
                if *item % monkey_test.get(&id).unwrap() == 0 {
                    moves.push((*item, monkey_targets.get(&id).unwrap().0));
                } else {
                    moves.push((*item, monkey_targets.get(&id).unwrap().1));
                }
            }
            *monkey_inspect_count.get_mut(&id).unwrap() += moves.len() as u64;
            for mv in moves {
                monkey_inventories.get_mut(&mv.1).unwrap().push(mv.0);
                let idx = monkey_inventories
                    .get_mut(&id)
                    .unwrap()
                    .iter()
                    .position(|x| *x == mv.0)
                    .unwrap();
                monkey_inventories.get_mut(&id).unwrap().remove(idx);
                // println!("Moved item {} from {} to {}.", mv.0, id, mv.1);
            }
        }
    }
    let mut inspect_counts: Vec<u64> = monkey_inspect_count.values().map(|x| *x).collect();
    inspect_counts.sort_by(|a, b| b.cmp(a));
    println!("{:?}", inspect_counts);
    println!(
        "Monkey business: {:?}",
        inspect_counts[0] * inspect_counts[1]
    );
}
