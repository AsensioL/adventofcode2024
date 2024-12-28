use std::fs;

use day3::*;

static _SAMPLE_INPUT_1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
static _SAMPLE_INPUT_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() {
    // Sample input
    //let input = _SAMPLE_INPUT_1.to_string();
    //let input = _SAMPLE_INPUT_2.to_string();

    // Real/File input
    let file_path = "data/day3_part1.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let part1_solution = day3_part1(&input);
    println!("Part 1, solution: {part1_solution}");

    let part2_solution = day3_part2(&input);
    println!("Part 2, solution: {part2_solution}");
}