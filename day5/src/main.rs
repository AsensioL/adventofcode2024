use std::fs;

use day5::*;

static INPUT_FILE_PATH: &str = "data/input.txt";
static SAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

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

