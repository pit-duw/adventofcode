use std::collections::HashSet;
use std::fs;
use std::cmp;

fn score_from_tree(tree: (usize, usize), tree_vec: &Vec<Vec<i32>>) -> i32{
    let mut west = 0;
    let mut north  = 0;
    let mut east = 0;
    let mut south  = 0;

    let tree_val = tree_vec[tree.0][tree.1];

    for i in tree.0+1..tree_vec.len(){
        south += 1;
        if !(tree_val > tree_vec[i][tree.1]) {
            break;
        }
    }

    for i in (0..tree.0).rev(){
        north += 1;
        if !(tree_val > tree_vec[i][tree.1]) {
            break;
        }
    }

    for i in tree.1+1..tree_vec[0].len(){
        east += 1;
        if !(tree_val > tree_vec[tree.0][i]) {
            break;
        }
    }

    for i in (0..tree.1).rev(){
        west += 1;
        if !(tree_val > tree_vec[tree.0][i]) {
            break;
        }
    }

    return west*north*east*south;
}

fn main() {
    let datastream: String = fs::read_to_string("input/day7.txt")
        .expect("Should have been able to read the file")
        .chars()
        .collect();

    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();

    let tree_vec: Vec<Vec<i32>> = datastream
        .trim()
        .split("\n")
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect();


    for i in 0..tree_vec.len(){
        let mut tallest_in_row = -1;
        for j in 0..tree_vec[0].len(){
            if tree_vec[i][j] > tallest_in_row {
                tallest_in_row = tree_vec[i][j];
                visible_trees.insert((i,j));
            }
        }
    }

    for i in 0..tree_vec[0].len(){
        let mut tallest_in_col = -1;
        for j in 0..tree_vec.len(){
            if tree_vec[j][i] > tallest_in_col {
                tallest_in_col= tree_vec[j][i];
                visible_trees.insert((j,i));
            }
        }
    }

    for i in 0..tree_vec.len(){
        let mut tallest_in_row = -1;
        for j in (0..tree_vec[0].len()).rev(){
            if tree_vec[i][j] > tallest_in_row {
                tallest_in_row = tree_vec[i][j];
                visible_trees.insert((i,j));
            }
        }
    }

    for i in 0..tree_vec[0].len(){
        let mut tallest_in_col = -1;
        for j in (0..tree_vec.len()).rev(){
            if tree_vec[j][i] > tallest_in_col {
                tallest_in_col= tree_vec[j][i];
                visible_trees.insert((j,i));
            }
        }
    }

    let mut best_score = 0;
    for tree in visible_trees.iter(){
        println!("Best score: {}, tree [{},{}]", best_score, tree.0, tree.1);
        best_score = cmp::max(best_score, score_from_tree(*tree, &tree_vec));
    }

    // Part 1
    println!("Number of visible trees: {:?}", visible_trees.len());
    // Part 2
    println!("Best score: {}", best_score);

}
