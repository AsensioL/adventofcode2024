use std::fs;

use day17::*;

static INPUT_FILE_PATH: &str = "data/input.txt";
static SAMPLE_INPUT_1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
static SAMPLE_INPUT_2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

#[allow(dead_code)]
enum InputType{
    Sample1,
    Sample2,
    File
}

fn main() {
    // Input choice --> CHANGE THIS ENUM HERE <--
    let input_type = InputType::File;

    let input = match input_type {
        InputType::Sample1 => SAMPLE_INPUT_1.to_string(),
        InputType::Sample2 => SAMPLE_INPUT_2.to_string(),
        InputType::File   => fs::read_to_string(INPUT_FILE_PATH)
                                .expect("Should have been able to read the file")
    };

    let part1_solution = part1(&input);
    println!("Part 1, solution: {part1_solution}");

    let part2_solution = part2(&input);
    println!("Part 2, solution: {part2_solution}");
}

