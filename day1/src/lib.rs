use std::collections::HashMap;

pub fn day1_part1_naive(text: & String) -> i32
{
    // Convenience modification
    let modified_text: String = text.replace("   ", " ");

    // Make single list with all numbers
    let numbers = modified_text
        .split(&[' ', '\n'])
        .map( |txt| txt.parse::<i32>().expect("Failed to parse") )
        .collect::<Vec<i32>>();

    // List of tuples with both numbers from each row
    let pairs = numbers.chunks_exact(2)
        .map( |chunk | (chunk[0], chunk[1]) )
        .collect::<Vec<_>>();

    // Transpose the list of tuples into a tuple of lists
    let (mut column1, mut column2): (Vec<i32>, Vec<i32>) = pairs.iter().cloned().unzip();

    // Sort them
    column1.sort();
    column2.sort();

    // Calculate sum of differences
    column1.iter().zip(column2)
                    .map( |(c1, c2)| (c1 - c2).abs() )
                    .sum()
}


pub fn day1_part1_no_string_copy(text: & String) -> i32
{
    // Make single list with all numbers
    let numbers = text.lines()
        .flat_map( |line| line.split("   ") )
        .map( |txt| txt.parse::<i32>().expect("Failed to parse") )
        .collect::<Vec<_>>();

    // List of tuples with both numbers from each row
    let pairs = numbers.chunks_exact(2)
        .map( |chunk | (chunk[0], chunk[1]) )
        .collect::<Vec<_>>();

    // Transpose the list of tuples into a tuple of lists
    let (mut column1, mut column2): (Vec<i32>, Vec<i32>) = pairs.iter().cloned().unzip();

    // Sort them
    column1.sort();
    column2.sort();

    // Calculate sum of differences
    column1.iter().zip(column2)
                    .map( |(c1, c2)| (c1 - c2).abs() )
                    .sum()
}

pub fn day1_part1(text: & String) -> i32
{
    // Get value pairs
    let pairs = text.lines()
        .map( |line| {
            let mut values = line.split("   ")
            .map( |val|
                val.parse::<i32>().expect("Failed to parse")
            );
            (values.next().unwrap(), values.next().unwrap())
        } )
        .collect::<Vec<_>>();


    // Transpose the list of tuples into a tuple of lists
    let (mut column1, mut column2): (Vec<i32>, Vec<i32>) = pairs.iter().cloned().unzip();

    // Sort them
    column1.sort();
    column2.sort();

    // Calculate sum of differences
    column1.iter().zip(column2)
                    .map( |(c1, c2)| (c1 - c2).abs() )
                    .sum()
}

pub fn day1_part2(text: & String) -> i32
{
    // Get value pairs
    let pairs = text.lines()
        .map( |line| {
            let mut values = line.split("   ")
            .map( |val|
                val.parse::<i32>().expect("Failed to parse")
            );
            (values.next().unwrap(), values.next().unwrap())
        } )
        .collect::<Vec<_>>();


    // Transpose the list of tuples into a tuple of lists
    let (column1, column2): (Vec<i32>, Vec<i32>) = pairs.iter().cloned().unzip();

    // Use a hash to count each number on the left and right columns
    let mut numbers_col1: HashMap<i32, i32> = HashMap::new();
    let mut numbers_col2: HashMap<i32, i32> = HashMap::new();

    // Count repetitions of the numbers in first column
    for v in column1 {
        numbers_col1.entry(v).and_modify(|count| *count += 1).or_insert(1);
    }

    // Count repetitions of the numbers in second column
    for v in column2 {
        numbers_col2.entry(v).and_modify(|count| *count += 1).or_insert(1);
    }

    // Instead of iterating on first row, first HashMap already knows how many times a number on first column appears
    numbers_col1.iter()
        .map( |(key, value)|
            key * value * numbers_col2.get(key).unwrap_or(&0) )
        .sum()
}

