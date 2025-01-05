use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Invalid direction character")
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Guard {
    x: i32,
    y: i32,
    dir: Direction
}

impl Guard {
    fn from_raw(x: usize, y: usize, dir: char) -> Self {
        Self {x: x as i32, y: y as i32, dir: Direction::from_char(dir)}
    }

    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn next_position(&self) -> Self {
        let x_delta = match self.dir {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0
        };
        let y_delta = match self.dir {
            Direction::Down => 1,
            Direction::Up => -1,
            _ => 0
        };
        Self{x: self.x + x_delta, y: self.y + y_delta, dir: self.dir}
    }

    fn step(&mut self) {
        let x_delta = match self.dir {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0
        };
        let y_delta = match self.dir {
            Direction::Down => 1,
            Direction::Up => -1,
            _ => 0
        };
        self.x  += x_delta;
        self.y  += y_delta;
    }

    fn turn(&mut self) {
        self.dir = match self.dir {
            Direction::Up    => Direction::Right,
            Direction::Down  => Direction::Left,
            Direction::Left  => Direction::Up,
            Direction::Right => Direction::Down
        };
    }

    fn is_colliding(&self, obstacles: &[(i32, i32)]) -> bool {
        obstacles.iter()
            .any( |(x, y)| self.x == *x && self.y == *y )
    }

    fn is_out_of_bounds(&self, width: usize, height: usize) -> bool {
        let width  = width  as i32;
        let height = height as i32;

        !(0..width).contains(&self.x) || !(0..height).contains(&self.y)
    }
}

pub fn part1(input: &str) -> usize
{
    let char_table = input.lines()
        .map( |line|  line.chars().collect::<Vec<char>>() )
        .collect::<Vec<Vec<char>>>();

    let height = char_table.len();
    let width = char_table[0].len();

    let mut guard_finder = None;
    let mut obstacles: Vec<(i32, i32)> = vec!();

    for (line_idx , col_idx, ch) in char_table.iter()
        .enumerate()
        .flat_map( |(line_idx, line)|
        {
            line.iter()
                .enumerate()
                .map( move |(col_idx, ch)| (line_idx, col_idx, ch) )
        })
    {
        if "^>v<".find(*ch).is_some() {
            guard_finder = Some(Guard::from_raw(col_idx, line_idx, *ch));
        }
        else if *ch == '#' {
            obstacles.push((col_idx as i32, line_idx as i32));
        }
    }

    let mut guard = guard_finder.expect("Did not find initial guard position");

    let mut covered_positions: HashSet<(i32, i32)> = HashSet::new();

    // Update the guard position
    while !guard.is_out_of_bounds(width, height) {
        covered_positions.insert(guard.position());

        if guard.next_position().is_colliding(&obstacles) {
            guard.turn();
        }
        else {
            guard.step();
        }
    }

    covered_positions.len()
}

pub fn part2(input: &str) -> usize
{
    let char_table = input.lines()
        .map( |line|  line.chars().collect::<Vec<char>>() )
        .collect::<Vec<Vec<char>>>();

    let height = char_table.len();
    let width = char_table[0].len();

    let mut guard_finder = None;
    let mut obstacles: Vec<(i32, i32)> = vec!();

    for (line_idx , col_idx, ch) in char_table.iter()
        .enumerate()
        .flat_map( |(line_idx, line)|
        {
            line.iter()
                .enumerate()
                .map( move |(col_idx, ch)| (line_idx, col_idx, ch) )
        })
    {
        if "^>v<".find(*ch).is_some() {
            guard_finder = Some(Guard::from_raw(col_idx, line_idx, *ch));
        }
        else if *ch == '#' {
            obstacles.push((col_idx as i32, line_idx as i32));
        }
    }

    let mut guard = guard_finder.expect("Did not find initial guard position");
    let initial_guard = guard;

    let mut covered_positions: HashSet<(i32, i32)> = HashSet::new();

    // Find out which positions are covered by the guard, because it does not
    // make sense to put obstacles in a place that it wouldn't go
    while !guard.is_out_of_bounds(width, height) {
        covered_positions.insert(guard.position());

        if guard.next_position().is_colliding(&obstacles) {
            guard.turn();
        }
        else {
            guard.step();
        }
    }

    // Remove the guard's initial position
    covered_positions.remove(&initial_guard.position());

    // Test every other position for loop-forming obstacles, and count
    covered_positions.iter()
        //.enumerate()
        //.map( |(idx, v)|
        //{
        //    println!("Working on item: {idx}, placing obstacle in {v:?}");
        //    v
        //})
        .map( |(x, y)|
        {
            let mut guard = initial_guard;
            let mut previous_states: HashSet<Guard> = HashSet::new();
            let mut new_obstacles: Vec<(i32, i32)> = obstacles.clone();
            new_obstacles.push((*x, *y));

            while !guard.is_out_of_bounds(width, height) {

                // Check if this is a repeated state
                if previous_states.contains(&guard) {
                    return true; // IS LOOP
                }
                else {
                    previous_states.insert(guard);
                }

                // Update the guard position
                if guard.next_position().is_colliding(&new_obstacles) {
                    guard.turn();
                }
                else {
                    guard.step();
                }
            }
            false // IS NOT LOOP
        })
        .filter( |b| *b)
        .count()
}