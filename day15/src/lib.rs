use std::collections::VecDeque;

use rectangle::{Rectangle, Rectangular, RectangularData, Direction};
use itertools::Itertools;


fn dir_from_char(ch: char) -> Option<Direction> {
    match ch {
        '^' => Some(Direction::Up),
        'v' => Some(Direction::Down),
        '<' => Some(Direction::Left),
        '>' => Some(Direction::Right),
        _   => None,
    }
}


trait Warehouse: RectangularData<char> + Rectangular {
    fn is_box(&self, coord: &(usize, usize)) -> bool {
        let Ok(ch) = self.get(coord) else { return false; };
        *ch == 'O' || *ch == '[' || *ch == ']'
    }

    fn is_side(&self, coord: &(usize, usize)) -> bool {
        let Ok(ch) = self.get(coord) else { return true; };
        *ch == '#'
    }

    fn push(&mut self, item_position: &mut (usize, usize), dir: Direction) -> bool {
        let new_position = self.next_coord(item_position, dir).unwrap();

        // If the new position is a side, don't move
        if self.is_side(&new_position) {
            return false;
        }

        // If the new position is a box, try to move the box
        if self.is_box(&new_position) {
            let mut box_position = new_position; // Discardable copy

            // If the box moves, then this item (robot or another box) can move
            if self.push(&mut box_position, dir) {
                self.swap(item_position, &new_position).unwrap();

                // Update the robot position
                *item_position = new_position;
                return true;
            }
            else {
                return false;
            }
        }

        //The new position must be empty, push
        self.swap(item_position, &new_position).unwrap();
        *item_position = new_position;
        true
    }

    fn push_wide(&mut self, item_position: &mut (usize, usize), dir: Direction) {
        // Pushing horizontally is easy (already handled)
        if matches!(dir, Direction::Left) || matches!(dir, Direction::Right) {
            self.push(item_position, dir);
            return;
        }

        // Pushing vertically is hard:
        let new_position = self.next_coord(item_position, dir).unwrap();

        if self.is_side(&new_position) {
            // If new position would be a side, then don't move
        }
        else if self.is_box(&new_position) {
            // If the new position is a box, we need figure out all the affected box locations
            // to push them all at once (if viable).
            // Viability check requires to keep track of the box front, as we update
            let mut push_positions: Vec<(usize, usize)> = vec!{*item_position}; // Will be used in reverse to update locations
            let mut box_front: VecDeque<(usize, usize)> = VecDeque::new(); // Queue used to progressively explore viability of the push

            // Update box positions and box front
            let wide_box = [new_position, self.get_other_side_of_the_box(&new_position)];
            push_positions.extend(wide_box.iter());
            box_front.extend(wide_box.iter());

            while !box_front.is_empty() {
                let after_front_position = self.next_coord(&box_front.pop_front().unwrap(), dir).unwrap();

                if self.is_side(&after_front_position) {
                    // If after front position is a side, then the whole thing falls apart, can't be pushed
                    return;
                }
                else if self.is_box(&after_front_position) {
                    // If this is a box, this might have already been put into the data structs, in which case, just skip this one
                    if push_positions.contains(&after_front_position) {
                        continue
                    }

                    // Normally, just update the front and pushing will happen at the end
                    let other_wide_box = [after_front_position, self.get_other_side_of_the_box(&after_front_position)];
                    // Update box positions and box front
                    push_positions.extend(other_wide_box.iter());
                    box_front.extend(other_wide_box.iter());
                }
                // else, pushing this one is fine, so let it be handled if everything is viable
            }

            // Pushing time, start from the end to avoid overlaps
            push_positions.iter().rev()
                .for_each(|element_position| {
                    self.swap(element_position, &self.next_coord(element_position, dir).unwrap()).unwrap();
                });
            // Update the robot position
            *item_position = new_position;
        }
        else {
            // Simple push with no obstacles
            self.swap(item_position, &new_position).unwrap();
            // Update the robot position
            *item_position = new_position;
        }
    }

    fn get_other_side_of_the_box(&mut self, item_location: &(usize, usize)) -> (usize, usize) {
        // Get the character
        let c = self.get(item_location).expect("Should have been a valid coordinate");
        let (row, col) = *item_location;

        // Calculate the other coordinate accordingly
        let new_coord = match c {
            '[' => (row, col + 1),
            ']' => (row, col - 1),
            _ => panic!("Character {c} is not a box")
        };

        // Validation
        let other_c = *self.get(&new_coord).expect("Calculated coordinate should be inside rectangle");
        match c {
            '[' => assert!( other_c == ']' ),
            ']' => assert!( other_c == '[' ),
            _ => unreachable!()
        };

        // Return calculated coordinate
        new_coord
    }
}

// This is the lazy way
impl Warehouse for Rectangle<char> { }

pub fn part1(input: &str) -> usize
{
    let (warehouse, instructions) = input.split("\n\n")
        .collect_tuple::<(&str, &str)>()
        .unwrap_or_else(|| {
            input.split("\r\n\r\n")
                .collect_tuple::<(&str, &str)>()
                .expect("Failed to split input")
        });

    let mut warehouse = Rectangle::from_char_str(warehouse)
        .map_err( |error|  panic!("Failed to parse rectangle. Reason: {error}") )
        .unwrap();

    let mut robot_position = warehouse.iter_coord()
        .find(|c| *warehouse.get(c).unwrap() == '@' )
        .unwrap_or_else(|| panic!("Failed to locate initial position") );

    let instructions = instructions.lines()
        .flat_map(|line| line.chars() )
        .collect::<String>();

    // Before
    // println!("{}\n", warehouse.to_string());

    instructions.chars()
        .map(|c| dir_from_char(c).unwrap_or_else(|| panic!("Invalid direction character: {c}")))
        .for_each(|dir| {
            warehouse.push(&mut robot_position, dir);

            // Step
            // println!("{}\n", warehouse.to_string());
        });

    // After
    // println!("{}", warehouse.to_string());

    warehouse.iter_coord()
        .filter(|&coord| *warehouse.get(&coord).unwrap() == 'O' )
        .map(|c| 100 * c.0 + c.1 )
        .sum()
}


pub fn part2(input: &str) -> usize
{
    let (warehouse, instructions) = input.split("\n\n")
        .collect_tuple::<(&str, &str)>()
        .unwrap_or_else(|| {
            input.split("\r\n\r\n")
                .collect_tuple::<(&str, &str)>()
                .expect("Failed to split input")
        });

    let wide_warehouse = warehouse.lines()
        .flat_map(|l| l.chars().chain(Some('\n')) )
        .map(|c| {
            match c {
                '#' => "##".to_string(),
                'O' => "[]".to_string(),
                '.' => "..".to_string(),
                '@' => "@.".to_string(),
                _   => c.to_string(),
            }
        })
        .collect::<String>();

    let mut warehouse = Rectangle::from_char_str(&wide_warehouse)
        .map_err( |error|  panic!("Failed to parse rectangle. Reason: {error}") )
        .unwrap();

    let mut robot_position = warehouse.iter_coord()
        .find(|c| *warehouse.get(c).unwrap() == '@' )
        .unwrap_or_else(|| panic!("Failed to locate initial position") );

    let instructions = instructions.lines()
        .flat_map(|line| line.chars() )
        .collect::<String>();

    // Before
    // println!("{}\n", warehouse.to_string());

    instructions.chars()
        .map(|c| dir_from_char(c).unwrap_or_else(|| panic!("Invalid direction character: {c}")))
        .for_each(|dir| {
            warehouse.push_wide(&mut robot_position, dir);

            // Step
            // println!("{}\n", warehouse.to_string());
        });


    // After
    // println!("{}", warehouse.to_string());

    warehouse.iter_coord()
        .filter(|&coord| *warehouse.get(&coord).unwrap() == '[' )
        .map(|c| 100 * c.0 + c.1 )
        .sum()
}