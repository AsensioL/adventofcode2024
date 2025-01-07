use regex::Regex;

pub fn part1(input: &str) -> i64
{
    // Parse input
    let re = Regex::new(r"(\d+): ([\d ]+)").unwrap();
    let rows = input.lines()
        .map( |line|
        {
            re.captures(line)
                .map( |c| (c.get(1).unwrap().as_str(), c.get(2).unwrap().as_str()) )
                .map( |(total_str, numbers_str)|
                (
                    total_str.parse::<i64>().unwrap_or_else(|_| {println!("Failed to unwrap `{total_str}`"); panic!()}),
                    numbers_str.split(' ')
                        .map( |num_str| num_str.parse::<i64>().unwrap() )
                        .collect::<Vec<i64>>()
                ))
                .unwrap()
        })
        .collect::<Vec<_>>();

    // Verify parsed input
    //rows.into_iter().for_each( |(n, v)| println!("Total is {n}, numbers are {v:?}"));

    rows.into_iter()
        .filter( |(total, numbers)|
        {
            // Put the first number in a vector
            // For every remaining number, take the existing vector (twice)
            // Add and multiply the new number and put back in a vector
            // Repeat for all the numbers
            numbers.iter()
                .fold( vec![], |prev_vec, &new_number|
                {
                    if prev_vec.is_empty() {
                        return vec![new_number];
                    }
                    prev_vec.iter()
                        .map(|v| v + new_number )
                        .chain(prev_vec.iter().map( |v| v * new_number ))
                        .collect::<Vec<_>>()
                })
                .into_iter()
                .any( |v| v == *total)
        })
        .map( |(total, _)| total )
        .sum()
}

pub fn part2(input: &str) -> i64
{
    // Parse input
    let re = Regex::new(r"(\d+): ([\d ]+)").unwrap();
    let rows = input.lines()
        .map( |line|
        {
            re.captures(line)
                .map( |c| (c.get(1).unwrap().as_str(), c.get(2).unwrap().as_str()) )
                .map( |(total_str, numbers_str)|
                (
                    total_str.parse::<i64>().unwrap_or_else(|_| {println!("Failed to unwrap `{total_str}`"); panic!()}),
                    numbers_str.split(' ')
                        .map( |num_str| num_str.parse::<i64>().unwrap() )
                        .collect::<Vec<i64>>()
                ))
                .unwrap()
        })
        .collect::<Vec<_>>();

    // Verify parsed input
    //rows.into_iter().for_each( |(n, v)| println!("Total is {n}, numbers are {v:?}"));

    rows.into_iter()
        .filter( |(total, numbers)|
        {
            // Put the first number in a vector
            // For every remaining number, take the existing vector (twice)
            // Add and multiply the new number and put back in a vector
            // Repeat for all the numbers
            numbers.iter()
                .fold( vec![], |prev_vec, &new_number|
                {
                    if prev_vec.is_empty() {
                        return vec![new_number];
                    }
                    prev_vec.iter()
                        .map(|v| v + new_number )
                        .chain(prev_vec.iter().map( |&v| v * new_number ))
                        .chain(prev_vec.iter().map( |&v| combine_numbers(v, new_number) ))
                        .collect::<Vec<_>>()
                })
                .into_iter()
                .any( |v| v == *total)
        })
        .map( |(total, _)| total )
        .sum()
}

// Count the number of zeros of second number (n_zeros), then shift left (ie:
// multiply by 10^n_zeros) the left number and add the second number.
// See tests below
fn combine_numbers(v1: i64, v2: i64) -> i64 {
    v1 * (10_i64).pow(1 + v2.ilog10()) + v2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_few_examples() {
        let result = combine_numbers(2, 2);
        assert_eq!(result, 22);

        let result = combine_numbers(20, 2);
        assert_eq!(result, 202);

        let result = combine_numbers(20, 20);
        assert_eq!(result, 2020);

        let result = combine_numbers(99999, 88888);
        assert_eq!(result, 9999988888);
    }
}