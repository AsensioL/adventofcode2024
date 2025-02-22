use std::fs;

use day14::*;

static INPUT_FILE_PATH: &str = "data/input.txt";
static INPUT_FILE_WIDTH: i32 = 101;
static INPUT_FILE_HEIGHT: i32 = 103;

// static SAMPLE_INPUT: &str = "p=0,4 v=3,-3
// p=6,3 v=-1,-3
// p=10,3 v=-1,2
// p=2,0 v=2,-1
// p=0,0 v=1,3
// p=3,0 v=-2,-2
// p=7,6 v=-1,-3
// p=3,0 v=-1,-2
// p=9,3 v=2,3
// p=7,3 v=-1,2
// p=2,4 v=2,-3
// p=9,5 v=-3,-3";
static SAMPLE_INPUT: &str = "p=2,2 v=3,1
p=2,3 v=1,1
p=2,4 v=-1,-1
p=2,5 v=2,-1
p=5,2 v=1,3
p=5,3 v=-2,-2
p=5,4 v=-1,-3
p=5,5 v=-1,-2
p=3,2 v=2,3
p=3,5 v=-1,2
p=4,2 v=2,-3
p=4,5 v=-3,-3";
static SAMPLE_WIDTH: i32 = 11;
static SAMPLE_HEIGHT: i32 = 7;

#[allow(dead_code)]
enum InputType{
    Sample,
    File
}

fn main() {
    // Input choice --> CHANGE THIS ENUM HERE <--
    let input_type = InputType::File;

    let (input, width, height) = match input_type {
        InputType::Sample => (SAMPLE_INPUT.to_string(), SAMPLE_WIDTH, SAMPLE_HEIGHT),
        InputType::File   => (fs::read_to_string(INPUT_FILE_PATH)
                                .expect("Should have been able to read the file"),
                                INPUT_FILE_WIDTH, INPUT_FILE_HEIGHT)
    };

    let part1_solution = part1(&input, width, height);
    println!("Part 1, solution: {part1_solution}");

    let part2_solution = part2(&input, width, height);
    println!("Part 2, solution: {part2_solution}");
}

