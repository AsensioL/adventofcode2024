#![recursion_limit = "256"]

pub fn magic(number: u64) -> impl IntoIterator<Item=u64> {
    if number == 0
    {
        vec!{1}.into_iter()
    }
    else
    {
        let txt = format!("{number}");
        let length = txt.len();

        if length % 2 == 0 {
            let lhs = txt[0..length/2].parse::<u64>().unwrap();
            let rhs = txt[length/2..length].parse::<u64>().unwrap();
            vec!{lhs, rhs}.into_iter()
        }
        else {
            vec!{number * 2024}.into_iter()
        }
    }
}

pub fn part1(input: &str) -> usize
{
    let numbers = input.split_ascii_whitespace()
        .map( |substr| substr.parse::<u64>().unwrap() )
        .collect::<Vec<_>>();

    numbers.into_iter()
        .flat_map( magic )
        .flat_map( magic )
        .flat_map( magic )
        .flat_map( magic )
        .flat_map( magic )

        .flat_map( magic )
        .flat_map( magic )
        .flat_map( magic )
        .flat_map( magic )
        .flat_map( magic )

        .flat_map( magic )
        .flat_map( magic )
        .flat_map( magic )
        .flat_map( magic )
        .flat_map( magic )

        .flat_map( magic )
        .flat_map( magic )
        .flat_map( magic )
        .flat_map( magic )
        .flat_map( magic )

        .flat_map( magic )
        .flat_map( magic )
        .flat_map( magic )
        .flat_map( magic )
        .flat_map( magic )
        .count()
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stones {
    number: u64,
    count: usize
}

impl Stones {
    pub fn blink(&mut self) -> Option<Self> {
        // Handle stone on the rules
        if self.number == 0
        {
            self.number = 1;
            None
        }
        else
        {
            let length = self.number.ilog10() + 1;

            if length % 2 == 0 {
                let base = 10_u64.pow(length/2);
                let lhs = self.number / base;
                let rhs = self.number % base;
                self.number = lhs;
                Some( Self { number: rhs, count: self.count } )
            }
            else {
                self.number *= 2024;
                None
            }
        }
    }
}

fn sort_and_group_stones(stones: &mut Vec<Stones>) {
    // Sort
    stones.sort_by_key(|stone| stone.number);


    // Add duplicated stone counts to the first one
    let len = stones.len();
    let mut head_idx = 0;
    for idx in 1..len {
        if stones[head_idx].number == stones[idx].number {
            stones[head_idx].count += stones[idx].count
        }
        else {
            head_idx = idx;
        }
    }

    // Remove duplicated stones (second and on)
    stones.dedup_by_key(|stone| stone.number);
}


pub fn part2(input: &str) -> u64
{
    let iterations = 75;

    let mut stones = input.split_ascii_whitespace()
        .map( |substr| substr.parse::<u64>().unwrap() )
        .map( |number| Stones { number, count: 1 })
        .collect::<Vec<_>>();

    for _iteration in 0..iterations {
        // Prepare overhead vector
        let mut added_stones= Vec::with_capacity(stones.len());

        // Blink stones
        stones.iter_mut()
            .for_each(|stone|
            {
                // Blink current stone
                let maybe_new_stone = stone.blink();

                // If this adds more stones, put them on a separate list and merge them later
                if let Some(new_stone) = maybe_new_stone {
                    added_stones.push(new_stone);
                }
            });

        // Join with overhead
        stones.extend(added_stones.into_iter());

        // Sort and group stones
        sort_and_group_stones(&mut stones);

        //println!("Iteration: {_iteration}, len: {}", stones.len());
        //println!("Iteration: {_iteration} -> {:?}", stones.iter().map(|s| (s.number, s.count)).collect::<Vec<_>>());
    }

    stones.into_iter()
        .map(|stone| stone.count as u64)
        .sum()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_sort_and_group_stones() {
        let mut input = vec![
            Stones{number: 1, count: 1},
            Stones{number: 2, count: 2},
            Stones{number: 1, count: 3},
            Stones{number: 3, count: 4}
        ];
        let output = vec![
            Stones{number: 1, count: 4},
            Stones{number: 2, count: 2},
            Stones{number: 3, count: 4}
        ];
        sort_and_group_stones(&mut input);
        assert_eq!(&input[..], &output[..]);
    }
}