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

fn blink_n_times(initial_number: u64, times: usize) -> Vec<u64> {
    let mut stones = Vec::with_capacity(128);
    stones.push(Stone{iterations_left: times, number: initial_number});

    loop {
        // Take the first stone the needs blinking or break
        let maybe_stone = stones.iter_mut()
            .filter(|stone| stone.iterations_left > 0 )
            .next();

        let Some(stone) = maybe_stone else {
            break;
        };

        // Blink it
        let maybe_new_stone = stone.blink();

        // Handle new stone (if present) by putting it at the end
        if let Some(new_stone) = maybe_new_stone {
            stones.push(new_stone);
        }
    }

    // Return stone numbers
    stones.into_iter()
        .map(|stone| stone.number )
        .collect::<Vec<_>>()
}

pub fn part2(input: &str) -> u64
{
    let iterations = 75;

    let mut numbers = input.split_ascii_whitespace()
        .map( |substr| substr.parse::<u64>().unwrap() )
        .map( |number| Stone { iterations_left: iterations, number })
        .collect::<Vec<_>>();

    let mut total: u64 = 0;


    // Prepopulation step
    let prepopulation_end: u64 = 100;
    let prepopulation_shortcut_blinks: usize = 25;
    let shortcut = (0..prepopulation_end)
        .map(|number| blink_n_times(number, prepopulation_shortcut_blinks) )
        .collect::<Vec<_>>();
    println!("Prepopulation done");


    loop {
        // Look at the stone at the back and blink it
        let stone = numbers.last_mut().unwrap();

        // Try to take the shortcut
        if stone.number < (shortcut.len() as u64) && stone.iterations_left >= prepopulation_shortcut_blinks{
            let number = stone.number as usize;
            let iterations_left = stone.iterations_left - prepopulation_shortcut_blinks;
            numbers.pop();

            // If the stones are out of iterations, count them instead
            if iterations_left == 0 {
                total += shortcut[number].len() as u64;
            }
            else { // Otherwise, replace them with the shortcut
                let mut new_stones = shortcut[number].iter()
                    .map(|&number| Stone{iterations_left, number} )
                    .collect::<Vec<Stone>>();
                numbers.append(&mut new_stones);
            }
        }
        else
        { // Otherwise take the normal approach
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
        }

        if numbers.is_empty() {
            break;
        }
        //else {
        //    let temp = numbers.iter()
        //        .map(|el| el.number)
        //        .collect::<Vec<_>>();
        //    println!("{:?}, IT-left: {}, total: {}", temp, numbers.last().unwrap().iterations_left, total);
        //}
    }

    total
}
