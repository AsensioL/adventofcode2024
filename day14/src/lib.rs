use std::collections::HashSet;
use regex::Regex;
use itertools::Itertools;

pub fn part1(input: &str, width: i32, height: i32) -> i32
{
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let iterations = 100;
    let half_width = width / 2;
    let half_height = height / 2;

    re.captures_iter(input)
        .map( |caps| caps.iter()
            .skip(1)
            .map( |c| c.unwrap().as_str().parse::<i32>().unwrap() )
            .collect_tuple::<(i32, i32, i32, i32)>()
            .unwrap()
        )
        .map(|(x,y, vx, vy)| {
            ((((x + iterations * vx) %  width) +  width) %  width,
             (((y + iterations * vy) % height) + height) % height)
        })
        .fold([0_i32;4], |mut c, (x, y)| {
            if      x < half_width && y < half_height { c[0] += 1; return c; }
            else if x > half_width && y < half_height { c[1] += 1; return c; }
            else if x < half_width && y > half_height { c[2] += 1; return c; }
            else if x > half_width && y > half_height { c[3] += 1; return c; }
            c
        })
        .iter()
        .product()
}

pub fn part2(input: &str, width: i32, height: i32) -> i32
{
    // Build regexp
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    // Parse data
    let parsed_data = re.captures_iter(input)
        .map( |caps| caps.iter()
            .skip(1)
            .map( |c| c.unwrap().as_str().parse::<i32>().unwrap() )
            .collect_tuple::<(i32, i32, i32, i32)>()
            .unwrap()
        )
        .collect::<Vec<_>>();

    // For each number of iterations between 0 and 10k
    let iterations_for_image = (0..10000)
        .map(|iterations| {
            // Calculate the positions of all robots and put them into a HashSet
            let positions = parsed_data.iter()
                .map(|(x,y, vx, vy)| {
                    ((((x + iterations * vx) %  width) +  width) %  width,
                    (((y + iterations * vy) % height) + height) % height)
                })
                .collect::<HashSet<(i32, i32)>>();

            // Calculate how many robots have another robot around them
            let number_of_close_neighbors = positions.iter()
                .filter(|&robot_coord| {
                    neighbor_coords(robot_coord).iter()
                        .any(|c| positions.contains(c) )
                })
                .count();
            (iterations, number_of_close_neighbors) // Return iterations and neighbors
        })
        // Find item with most neighbors, and return how many iterations it took to get there
        .max_by_key(|(_, neighbors)| *neighbors )
        .map(|(iterations, _)| iterations ) //
        .unwrap();

    print_robots(parsed_data, width, height, iterations_for_image);

    iterations_for_image
}

fn neighbor_coords(coord: &(i32, i32)) -> [(i32, i32); 8] {
    let (x, y) = coord;
    [
        ((*x + 1), (*y + 1)),
        ((*x + 1), (*y    )),
        ((*x + 1), (*y - 1)),
        ((*x    ), (*y - 1)),
        ((*x - 1), (*y - 1)),
        ((*x - 1), (*y    )),
        ((*x - 1), (*y + 1)),
        ((*x    ), (*y + 1)),
    ]
}

fn print_robots(robot_data: Vec<(i32, i32, i32, i32)>, width: i32, height:i32, iterations: i32) {
    // Calculate the positions the
    let image_positions = robot_data.iter()
    .map(|(x,y, vx, vy)| {
        ((((x + iterations * vx) %  width) +  width) %  width,
        (((y + iterations * vy) % height) + height) % height)
    })
    .collect::<HashSet<(i32, i32)>>();

    let mut output = String::new();

    for y in 0..height {
        for x in 0..width {
            output += if image_positions.contains( &(x, y) ) { "#" } else { " " };
        }
        output += "\n";
    }

    println!("{output}");
}
