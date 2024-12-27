//use std::env;
use std::fs;

use day2::*;

static _SAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

fn main() {
    // Sample input
    //let input = _SAMPLE_INPUT.to_string();

    // Real/File input
    let file_path = "data/day2_part1.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let part1_solution = day2_part1(&input);
    println!("Part 1, solution: {part1_solution}");

    let part2_solution = day2_part2(&input);
    println!("Part 2, solution: {part2_solution}");

}

