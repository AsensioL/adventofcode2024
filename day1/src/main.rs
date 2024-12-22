//use std::env;
use std::fs;

use day1::*;

static _SAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

fn main() {
    // Sample input
    //let input = _SAMPLE_INPUT.to_string();

    // Real/File input
    let file_path = "data/day1_part1.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let part1_solution = day1_part1(&input);
    println!("Part 1, solution: {part1_solution}");

    let part2_solution = day1_part2(&input);
    println!("Part 2, solution: {part2_solution}");

}

