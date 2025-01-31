use std::collections::HashSet;

trait Rectangle {
    fn adjacent_coordinates_matching(&self, coord: &(usize, usize), number: u32) -> std::vec::IntoIter<(usize, usize)>;
}

impl Rectangle for Vec<Vec<u32>> {
    fn adjacent_coordinates_matching(&self, coord: &(usize, usize), number: u32) -> std::vec::IntoIter<(usize, usize)> {
        let height = self.len();
        let width = self[0].len();

        let mut adjacent = Vec::with_capacity(4);

        if (1..height+1).contains(&coord.0    ) && (0..width  ).contains(&coord.1    ) {
            let c = (coord.0-1, coord.1  );
            let n = self[c.0][c.1];
            if number == n { adjacent.push( c ); }
        }
        if (0..height  ).contains(&(coord.0+1)) && (0..width  ).contains(&coord.1    ) {
            let c = (coord.0+1, coord.1  );
            let n = self[c.0][c.1];
            if number == n { adjacent.push( c ); }
        }
        if (0..height  ).contains(&coord.0    ) && (1..width+1).contains(&coord.1    ) {
            let c = (coord.0  , coord.1-1);
            let n = self[c.0][c.1];
            if number == n { adjacent.push( c ); }
        }
        if (0..height  ).contains(&coord.0    ) && (0..width  ).contains(&(coord.1+1)) {
            let c = (coord.0  , coord.1+1);
            let n = self[c.0][c.1];
            if number == n { adjacent.push( c ); }
        }

        adjacent.into_iter()
    }
}

pub fn part1(input: &str) -> usize
{
    // Parse input into a matrix
    let number_matrix: Vec<Vec<u32>> = input.lines()
        .map( |l|
        {
            l.chars()
                .map( |c| c.to_digit(10).unwrap() )
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    // Iterate over the coordinate of each trail head (num = 0)
    number_matrix.iter()
        .enumerate() // index is row, item is Vec<u32>
        .flat_map( |(row, vec_of_nums)|
        {
            vec_of_nums.iter()
                .enumerate() // index is column, item is number
                .filter( |(_, num)| **num == 0 )
                .map( move |(col, _)| (row, col) )
        })
        .map( |c|
        {
            // Calculate the rank of each trailhead
            number_matrix.adjacent_coordinates_matching(&c, 1)
                .flat_map( |c| number_matrix.adjacent_coordinates_matching(&c, 2) )
                .flat_map( |c| number_matrix.adjacent_coordinates_matching(&c, 3) )
                .flat_map( |c| number_matrix.adjacent_coordinates_matching(&c, 4) )
                .flat_map( |c| number_matrix.adjacent_coordinates_matching(&c, 5) )
                .flat_map( |c| number_matrix.adjacent_coordinates_matching(&c, 6) )
                .flat_map( |c| number_matrix.adjacent_coordinates_matching(&c, 7) )
                .flat_map( |c| number_matrix.adjacent_coordinates_matching(&c, 8) )
                .flat_map( |c| number_matrix.adjacent_coordinates_matching(&c, 9) )
                .collect::<HashSet<(usize, usize)>>()
                .len()
        })
        .sum()
}

pub fn part2(input: &str) -> usize
{
    // Parse input into a matrix
    let number_matrix: Vec<Vec<u32>> = input.lines()
        .map( |l|
        {
            l.chars()
                .map( |c| c.to_digit(10).unwrap() )
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    // Iterate over the coordinate of each trail head (num = 0)
    number_matrix.iter()
        .enumerate() // index is row, item is Vec<u32>
        .flat_map( |(row, vec_of_nums)|
        {
            vec_of_nums.iter()
                .enumerate() // index is column, item is number
                .filter( |(_, num)| **num == 0 )
                .map( move |(col, _)| (row, col) )
        })
        .map( |c|
        {
            // Calculate the rank of each trailhead
            number_matrix.adjacent_coordinates_matching(&c, 1)
                .flat_map( |c| number_matrix.adjacent_coordinates_matching(&c, 2) )
                .flat_map( |c| number_matrix.adjacent_coordinates_matching(&c, 3) )
                .flat_map( |c| number_matrix.adjacent_coordinates_matching(&c, 4) )
                .flat_map( |c| number_matrix.adjacent_coordinates_matching(&c, 5) )
                .flat_map( |c| number_matrix.adjacent_coordinates_matching(&c, 6) )
                .flat_map( |c| number_matrix.adjacent_coordinates_matching(&c, 7) )
                .flat_map( |c| number_matrix.adjacent_coordinates_matching(&c, 8) )
                .flat_map( |c| number_matrix.adjacent_coordinates_matching(&c, 9) )
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let data: Vec<Vec<u32>> = vec!{ vec!{1,2,3}, vec!{4, 5, 6}, vec!{7, 8, 9} };

        assert_eq!(
            data.adjacent_coordinates(&(1, 1)).collect::<Vec<(usize, usize)>>(),
            vec!{ (0, 1), (2, 1), (1, 0), (1, 2) }
        );
        assert_eq!(
            data.adjacent_coordinates(&(0, 0)).collect::<Vec<(usize, usize)>>(),
            vec!{ (1, 0), (0, 1) }
        );
        assert_eq!(
            data.adjacent_coordinates(&(0, 1)).collect::<Vec<(usize, usize)>>(),
            vec!{ (1, 1), (0, 0), (0, 2) }
        );
        assert_eq!(
            data.adjacent_coordinates(&(1, 0)).collect::<Vec<(usize, usize)>>(),
            vec!{ (0, 0), (2, 0), (1, 1) }
        );
        assert_eq!(
            data.adjacent_coordinates(&(2, 2)).collect::<Vec<(usize, usize)>>(),
            vec!{ (1, 2), (2, 1) }
        );
        assert_eq!(
            data.adjacent_coordinates(&(2, 1)).collect::<Vec<(usize, usize)>>(),
            vec!{ (1, 1), (2, 0), (2, 2) }
        );
        assert_eq!(
            data.adjacent_coordinates(&(1, 2)).collect::<Vec<(usize, usize)>>(),
            vec!{ (0, 2), (2, 2), (1, 1) }
        );
    }
}
