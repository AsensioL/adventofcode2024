use std::collections::HashMap;
use std::str;
use itertools::Itertools;


pub fn is_buildable(target: &str, sorted_parts: &Vec<&str>) -> Vec<usize>
{
    let target = target.as_bytes();
    let sorted_parts = sorted_parts.iter().map(|word| word.as_bytes() ).collect::<Vec<_>>();

    let mut target_offset = 0;
    let mut remaining_length = target.len();

    let mut part_indices = vec![];
    let mut current_skip = 0;

    loop {
        // Find a good substring from the subset
        if let Some((idx, &interesting_part)) = sorted_parts.iter()
            .enumerate()
            .filter(|(_, &part)| part.len() <= remaining_length )
            .skip(current_skip)
            .find(|(_, &word)| word == &target[target_offset..(target_offset+word.len())])
        {
            current_skip = 0;
            let element_len = interesting_part.len();
            target_offset    += element_len;
            remaining_length -= element_len;
            part_indices.push(idx);
            continue
        }

        // If we built the target, then leave
        if remaining_length == 0 {
            break;
        }

        // Otherwise, pop the last index (if it exists)
        let Some(popped_idx) = part_indices.pop() else {
            break;
        };
        let failed_element_len = sorted_parts[popped_idx].len();
        target_offset    -= failed_element_len;
        remaining_length += failed_element_len;
        current_skip = popped_idx + 1;
    }

    part_indices
}

pub fn part1(input: &str) -> usize
{
    let (towels, combinations) = input.split("\n\n")
        .collect_tuple::<(&str, &str)>()
        .unwrap_or_else(|| {
            input.split("\r\n\r\n")
                .collect_tuple::<(&str, &str)>()
                .expect("Failed to split input")
        });

    let towels = towels.split(", ")
        //.map(|word| word.chars().collect::<Vec<_>>() )
        .sorted_by_key(|word| (word.len(), word.to_string() ) )
        .collect::<Vec<_>>();
    // println!("Towels {towels:?}");

    let combinations = combinations.lines()
        .collect::<Vec<&str>>();

    combinations.iter()
        .filter(|&&combination| is_buildable(combination, &towels) != vec![] )
        .count()
}

// Naive approach
pub fn number_of_combinations(target: &str, sorted_parts: &Vec<&str>) -> usize
{
    let target = target.as_bytes();
    let sorted_parts = sorted_parts.iter().map(|word| word.as_bytes() ).collect::<Vec<_>>();

    let mut target_offset = 0;
    let mut remaining_length = target.len();

    let mut part_indices = vec![];
    let mut current_skip = 0;

    let mut found_solutions = 0;

    loop {
        // Find a good substring from the subset

        if let Some((idx, &interesting_part)) = sorted_parts.iter()
            .enumerate()
            .filter(|(_, &part)| part.len() <= remaining_length )
            .skip(current_skip)
            .find(|(_, &word)| word == &target[target_offset..(target_offset+word.len())])
        {
            current_skip = 0;
            let element_len = interesting_part.len();
            target_offset    += element_len;
            remaining_length -= element_len;
            part_indices.push(idx);
            continue
        }

        // If we built the target, then count it as a solution but don't leave
        if remaining_length == 0 {
            found_solutions += 1;
            //println!("Solution found: {part_indices:?}");
        }

        // Otherwise, pop the last index (if it exists)
        let Some(popped_idx) = part_indices.pop() else {
            break;
        };
        let failed_element_len = sorted_parts[popped_idx].len();
        target_offset    -= failed_element_len;
        remaining_length += failed_element_len;
        current_skip = popped_idx + 1;
    }

    found_solutions
}

// Solid approach
pub fn number_of_combinations2(target: &str, sorted_parts: &Vec<&str>) -> usize
{
    let target = target.as_bytes();
    let sorted_parts = sorted_parts.iter().map(|word| word.as_bytes() ).collect::<Vec<_>>();

    let     target_len = target.len();
    let mut target_offset = target_len - 1;   // Start position of A (see below)
    let mut target_suboffset = target_offset; // Start position of B (see below)

    // Number of way to build a substring
    let mut map = HashMap::new();

    // Walk from the end of the string backwards, calculating (and caching) each substring AB
    // as following sum:
    //   number of times AB is in the parts +
    //   number of times A is in the parts  * number of ways to build B
    // do this for all possible ways to split a substring in 2 parts (A and B)

    while target_suboffset <= target_len {
        let subtarget = &target[target_offset..];
        let half_a = &target[target_offset..target_suboffset];
        let half_b = &target[target_suboffset..];

        if target_offset == target_suboffset {
            // Number of times AB is in the parts (half_a is empty)
            let appearances = sorted_parts.iter()
                .filter(|&&part| part == half_b )
                .count();
            //println!("Round {}: Found {appearances}", str::from_utf8(half_b).unwrap());
            map.insert(half_b, appearances);
        }
        else {
            // Number of times A is in the parts * Number of ways to build B
            let appearances_h1 = sorted_parts.iter()
                .filter(|&&part| part == half_a )
                .count();
            let appearances_h2 = *map.get(half_b).unwrap();
            let combinations = appearances_h1 * appearances_h2;

            //println!("Round {}: Found {combinations} -> ({appearances_h1} of {}) * ({appearances_h2} of {})", str::from_utf8(subtarget).unwrap(), str::from_utf8(half_a).unwrap(), str::from_utf8(half_b).unwrap());

            map.entry(subtarget)
                .and_modify(|e| *e += combinations)
                .or_insert(combinations);
        }

        // Change the AB split (ie: move the first char of B to the end of A)
        target_suboffset += 1;

        // If B is empty, make AB in char longer (quit if done)
        if target_suboffset >= target_len {

            if target_offset == 0 {
                break;
            }
            target_offset -= 1;
            target_suboffset = target_offset;
        }
    }
    *map.get(target).unwrap_or(&0)
}

pub fn part2(input: &str) -> usize
{
    let (towels, combinations) = input.split("\n\n")
        .collect_tuple::<(&str, &str)>()
        .unwrap_or_else(|| {
            input.split("\r\n\r\n")
                .collect_tuple::<(&str, &str)>()
                .expect("Failed to split input")
        });

    let towels = towels.split(", ")
        //.map(|word| word.chars().collect::<Vec<_>>() )
        .sorted_by_key(|word| (word.len(), word.to_string() ) )
        .collect::<Vec<_>>();

    let combinations = combinations.lines()
        .collect::<Vec<&str>>();

    combinations.iter()
        .filter(|&&combination| is_buildable(combination, &towels) != vec![] )
        .map(|&combination| number_of_combinations2(combination, &towels) )
        .sum()
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_is_buildable() {

        let parts = is_buildable("asdf", &vec!["a", "f", "sd"]);
        assert_eq!(parts, vec![0, 2, 1]);

        let parts = is_buildable("aasdf", &vec!["a", "f", "asd"]);
        assert_eq!(parts, vec![0, 2, 1]);

        let parts = is_buildable("faasd", &vec!["f", "fa", "asd"]);
        assert_eq!(parts, vec![1, 2]);
    }

    #[test]
    fn test_is_non_buildable() {

        let parts = is_buildable("asgdf", &vec!["a", "sd", "f"]);
        assert_eq!(parts, vec![]);
    }
}