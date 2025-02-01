#![recursion_limit = "256"]

use std::{thread};

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


#[derive(Debug, Clone, Copy)]
pub struct Stone {
    iterations_left: usize,
    number: u64
}

impl Stone {
    pub fn blink(&mut self) -> Option<Self> {
        // Decrement remaining iterations
        self.iterations_left -= 1;

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
                Some( Stone { iterations_left: self.iterations_left, number: rhs } )
            }
            else {
                self.number *= 2024;
                None
            }
        }
    }
}


pub fn part2(input: &str) -> u64
{
    let iterations = 55;

    let numbers = input.split_ascii_whitespace()
        .map( |substr| substr.parse::<u64>().unwrap() )
        .map( |number| Stone { iterations_left: iterations, number })
        .collect::<Vec<_>>();

    let handles = numbers.into_iter()
        .map(|initial_stone|
        {
            thread::spawn(move || -> u64
            {
                let mut numbers = Vec::with_capacity(128);
                numbers.push(initial_stone);

                let mut total = 0_u64;

                loop {
                    // Look at the stone at the back and blink it
                    let stone = numbers.last_mut().unwrap();
                    let maybe_new_stone = stone.blink();

                    // If the stone is out of iterations, count it and pop it
                    if stone.iterations_left == 0 {
                        total += 1;
                        numbers.pop();
                    }

                    // Handle new stone (if present) to count it or add it to the stone list
                    if let Some(new_stone) = maybe_new_stone {
                        if new_stone.iterations_left == 0 {
                            total += 1;
                        }
                        else {
                            numbers.push(new_stone);
                        }
                    }

                    if numbers.is_empty() {
                        break;
                    }
                }
                total
            })
        })
    .collect::<Vec<_>>();

    println!("Threads started");

    handles.into_iter()
        .map(|h| h.join() )
        .map(|r| r.expect("Error") )
        .sum()
}
