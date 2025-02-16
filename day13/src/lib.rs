use regex::Regex;
use itertools::Itertools;
use std::cmp::min;

pub fn part1(input: &str) -> i32
{
    let re = Regex::new(r".+X\+(\d+), Y\+(\d+)\n.+X\+(\d+), Y\+(\d+)\n.+X=(\d+), Y=(\d+)").unwrap();

    let total_cost: i32 = re.captures_iter(input)
        .map( |caps| caps.iter()
            .skip(1)
            .map( |c| c.unwrap().as_str().parse::<i32>().unwrap() )
            .collect_tuple::<(i32, i32, i32, i32, i32, i32)>()
            .unwrap()
        )
        .filter_map(|(a_x, a_y, b_x, b_y, p_x, p_y)| {

            // Calculate the determinant of the matrix
            let determinant = a_x * b_y - a_y * b_x;

            // If the determinant is 0, the (a_x, a_y) and (b_x, b_y) are linearly dependent
            if determinant != 0 {
                // Solve using matrix algebra
                // Note: If these solutions have decimals, they are not valid.
                let n_press_a = (p_x * b_y - p_y * b_x) / determinant;
                let n_press_b = (a_x * p_y - a_y * p_x) / determinant;

                // Verify that the solution is correct (it might not be if there are decimals)
                if (a_x * n_press_a + b_x * n_press_b == p_x) && (a_y * n_press_a + b_y * n_press_b == p_y) {
                    Some( n_press_a * 3 + n_press_b )
                }
                else {
                    None
                }
            }
            else {
                // A and B are proportional, find if they work (none, one, both) and which one is cheaper
                let mut n_press_a = None;
                let mut n_press_b = None;

                if (p_x % a_x == 0) && (((p_x / a_x)*a_y) == p_y) {
                    n_press_a = Some( p_x / a_x );
                }
                if (p_x % b_x == 0) && (((p_x / b_x)*b_y) == p_y) {
                    n_press_b = Some( p_x / b_x );
                }

                match (n_press_a, n_press_b) {
                    (Some(a), Some(b)) => Some( min( 3*a, b ) ), // both work, take cheapest
                    (Some(a), None   ) => Some( 3*a ), // only a works
                    (None,    Some(b)) => Some( b ), // only b works
                    (None,    None   ) => None // none works
                }
            }
        } )
        .sum();

    total_cost
}

pub fn part2(input: &str) -> i64
{
    let re = Regex::new(r".+X\+(\d+), Y\+(\d+)\n.+X\+(\d+), Y\+(\d+)\n.+X=(\d+), Y=(\d+)").unwrap();

    let total_cost: i64 = re.captures_iter(input)
        .map( |caps| caps.iter()
            .skip(1)
            .map( |c| c.unwrap().as_str().parse::<i64>().unwrap() )
            .collect_tuple::<(i64, i64, i64, i64, i64, i64)>()
            .unwrap()
        )
        .map(|(a_x, a_y, b_x, b_y, p_x, p_y)| (a_x, a_y, b_x, b_y, p_x + 10000000000000, p_y + 10000000000000) ) // Add coordinates
        .filter_map(|(a_x, a_y, b_x, b_y, p_x, p_y)| {

            // Calculate the determinant of the matrix
            let determinant = a_x * b_y - a_y * b_x;

            // If the determinant is 0, the (a_x, a_y) and (b_x, b_y) are linearly dependent
            if determinant != 0 {
                // Solve using matrix algebra
                // Note: If these solutions have decimals, they are not valid.
                let n_press_a = (p_x * b_y - p_y * b_x) / determinant;
                let n_press_b = (a_x * p_y - a_y * p_x) / determinant;

                // Verify that the solution is correct (it might not be if there are decimals)
                if (a_x * n_press_a + b_x * n_press_b == p_x) && (a_y * n_press_a + b_y * n_press_b == p_y) {
                    Some( n_press_a * 3 + n_press_b )
                }
                else {
                    None
                }
            }
            else {
                // A and B are proportional, find if they work (none, one, both) and which one is cheaper
                let mut n_press_a = None;
                let mut n_press_b = None;

                if (p_x % a_x == 0) && (((p_x / a_x)*a_y) == p_y) {
                    n_press_a = Some( p_x / a_x );
                }
                if (p_x % b_x == 0) && (((p_x / b_x)*b_y) == p_y) {
                    n_press_b = Some( p_x / b_x );
                }

                match (n_press_a, n_press_b) {
                    (Some(a), Some(b)) => Some( min( 3*a, b ) ), // both work, take cheapest
                    (Some(a), None   ) => Some( 3*a ), // only a works
                    (None,    Some(b)) => Some( b ), // only b works
                    (None,    None   ) => None // none works
                }
            }
        } )
        .sum();

    total_cost
}