use std::fs;

use day12::*;

static INPUT_FILE_PATH: &str = "data/input.txt";
// static SAMPLE_INPUT: &str = "AAAA
// BBCD
// BBCC
// EEEC";

// static SAMPLE_INPUT: &str = "OOOOO
// OXOXO
// OOOOO
// OXOXO
// OOOOO";

// static SAMPLE_INPUT: &str = "AAAAAA
// AAABBA
// AAABBA
// ABBAAA
// ABBAAA
// AAAAAA";

static SAMPLE_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

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

