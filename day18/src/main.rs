use std::fs;

use day18::*;

static INPUT_FILE_PATH: &str = "data/input.txt";
static SAMPLE_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

#[allow(dead_code)]
enum InputType{
    Sample,
    File
}

fn main() {
    // Input choice --> CHANGE THIS ENUM HERE <--
    let input_type = InputType::File;

    let (input, width, height, fall_count) = match input_type {
        InputType::Sample => {(
            SAMPLE_INPUT.to_string(),
            /* width      */ 7,
            /* height     */ 7,
            /* fall_count */ 12)},
        InputType::File   => {(
            fs::read_to_string(INPUT_FILE_PATH)
                .expect("Should have been able to read the file"),
            /* width      */ 71,
            /* height     */ 71,
            /* fall_count */ 1024)}
    };

    let part1_solution = part1(&input, width, height, fall_count);
    println!("Part 1, solution: {part1_solution}");

    let part2_solution = part2(&input, width, height,fall_count);
    println!("Part 2, solution: {part2_solution}");
}

