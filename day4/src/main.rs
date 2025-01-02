use std::fs;

use day4::*;

static INPUT_FILE_PATH: &str = "data/input.txt";
static SAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

#[allow(dead_code)]
enum InputType{
    SAMPLE,
    FILE
}

fn main() {
    // Input choice --> CHANGE THIS ENUM HERE <--
    let input_type = InputType::FILE;


    let input = match input_type {
        InputType::SAMPLE => SAMPLE_INPUT.to_string(),
        InputType::FILE   => fs::read_to_string(INPUT_FILE_PATH)
                                .expect("Should have been able to read the file")
    };

    let part1_solution = part1(&input);
    println!("Part 1, solution: {part1_solution}");

    let part2_solution = part2(&input);
    println!("Part 2, solution: {part2_solution}");

}

