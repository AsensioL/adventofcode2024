use std::fs;

use day9::*;

static INPUT_FILE_PATH: &str = "data/input.txt";
static SAMPLE_INPUT: &str = "2333133121414131402";

#[allow(dead_code)]
enum InputType{
    Sample,
    File
}

fn main() {
    // Input choice --> CHANGE THIS ENUM HERE <--
    let input_type = InputType::File;

    let input = match input_type {
        InputType::Sample => SAMPLE_INPUT.to_string(),
        InputType::File   => fs::read_to_string(INPUT_FILE_PATH)
                                .expect("Should have been able to read the file")
    };

    let part1_solution = part1(&input);
    println!("Part 1, solution: {part1_solution}");

    let part2_solution = part2(&input);
    println!("Part 2, solution: {part2_solution}");

}

