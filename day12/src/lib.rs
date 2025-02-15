use std::collections::HashSet;

use rectangle::{Rectangle, RectangleEdge, Rectangular, RectangularData};

pub fn part1(input: &str) -> usize
{
    let r: Rectangle<char> = Rectangle::from_char_str(input).unwrap();

    let mut global_visited = HashSet::new();

    let mut result: usize = 0;

    // Iterate over each coordinate
    for init_coord in r.iter_coord()
    {
        // Continue if this coord is already visited
        if global_visited.contains(&init_coord) {
            continue;
        }

        // Take first char and build a FILO
        let c = *r.get(init_coord).unwrap();
        let mut filo = vec!{init_coord};
        let mut current_group = HashSet::new();

        // Iterate on the FILO until it is empty, to build the current group
        while let Some(new_coord) = filo.pop() {
            // Continue if already visited
            if global_visited.contains(&new_coord) {
                continue;
            }
            global_visited.insert(new_coord);
            current_group.insert(new_coord);

            // Extend the FILO with adjacent matching coordinates that have not been visited
            filo.extend(
                r.adjacent_coordinates_matching(&new_coord, &c)
                    .into_iter()
                    .flatten()
                    .filter(|adjancent_coord| !current_group.contains(adjancent_coord))
            );
        }

        // println!("Char `{c}` has {} cells: {:?}", current_group.len(), current_group);

        // Calculate area and perimeter
        let area: usize = current_group.len();
        let perimeter: usize = current_group.iter()
            .map(|coord: &(usize, usize)| {
                let adjacent_cells = r.adjacent_coordinates(coord).into_iter().flatten().collect::<Vec<_>>();
                let beyond_limit_cells = 4 - adjacent_cells.len();
                let other_cells = adjacent_cells.into_iter()
                    .filter(|coord| *r.get(*coord).unwrap() != c )
                    .count();
                beyond_limit_cells + other_cells
            })
            .sum();
        // println!("Perimeter: {perimeter}, Area: {area}");
        result += area * perimeter;
    }

    result
}

pub fn part2(input: &str) -> usize
{
    let r: Rectangle<char> = Rectangle::from_char_str(input).unwrap();

    let mut global_visited = HashSet::new();

    let mut result: usize = 0;

    // Iterate over each coordinate
    for init_coord in r.iter_coord()
    {
        // Continue if this coord is already visited
        if global_visited.contains(&init_coord) {
            continue;
        }

        // Take first char and build a FILO
        let c = *r.get(init_coord).unwrap();
        let mut filo = vec!{init_coord};
        let mut current_group = HashSet::new();

        // Iterate on the FILO until it is empty, to build the current group
        while let Some(new_coord) = filo.pop() {
            // Continue if already visited
            if global_visited.contains(&new_coord) {
                continue;
            }
            global_visited.insert(new_coord);
            current_group.insert(new_coord);

            // Extend the FILO with adjacent matching coordinates that have not been visited
            filo.extend(
                r.adjacent_coordinates_matching(&new_coord, &c)
                    .into_iter()
                    .flatten()
                    .filter(|adjancent_coord| !current_group.contains(adjancent_coord))
            );
        }

        // Collect all the edges into a set
        let edge_set: HashSet<_> = current_group.iter()
            .flat_map( |c| r.adjacent_coordinates(c).into_iter().zip(r.edges(c).into_iter() )) // Adjacent coords and edges are aligned
            .filter_map(|(maybe_coord, edge)| {

                // If coord is None, this is beyond the rectangle, so it is an edge
                let Some(coord) = maybe_coord else {
                    return Some(edge);
                };

                // If the adjacent cell value is different from this one, then it is an edge. Otherwise, it isn't.
                if r.get(coord).unwrap() != &c {
                    Some(edge)
                }
                else {
                    None
                }
            })
            .collect();

        // It is not enough to separate edges by horizontal and vertical because this fail
        // when counting edges for will fail to count in cases like this:
        //   AAABBA
        //   AAABBA
        //   ABBAAA
        //   ABBAAA
        //   AAAAAA

        // Count the number of sides by removing contiguous edges in the same row/column
        let mut top_edges: Vec<_> = edge_set.iter().copied()
            .filter(RectangleEdge::is_top)
            .collect();
        top_edges.sort_by_key(|edge| (edge.row(), edge.column()) );
        let top_sides = 1 + top_edges.windows(2)
            .filter(|pair| (pair[0].column() + 1) != pair[1].column() || pair[0].row() != pair[1].row() )
            .count();

        let mut bottom_edges: Vec<_> = edge_set.iter().copied()
            .filter(RectangleEdge::is_bottom)
            .collect();
        bottom_edges.sort_by_key(|edge| (edge.row(), edge.column()) );
        let bottom_sides = 1 + bottom_edges.windows(2)
            .filter(|pair| (pair[0].column() + 1) != pair[1].column() || pair[0].row() != pair[1].row() )
            .count();

        let mut left_edges: Vec<_> = edge_set.iter().copied()
            .filter(RectangleEdge::is_left)
            .collect();
        left_edges.sort_by_key(|edge| (edge.column(), edge.row()) );
        let left_sides =  1 + left_edges.windows(2)
            .filter(|pair| (pair[0].row() + 1) != pair[1].row() || pair[0].column() != pair[1].column() )
            .count();

        let mut right_edges: Vec<_> = edge_set.iter().copied()
            .filter(RectangleEdge::is_right)
            .collect();
        right_edges.sort_by_key(|edge| (edge.column(), edge.row()) );
        let right_sides =  1 + right_edges.windows(2)
            .filter(|pair| (pair[0].row() + 1) != pair[1].row() || pair[0].column() != pair[1].column() )
            .count();

        let total_sides = top_sides + bottom_sides + left_sides + right_sides;

        // println!("Current group `{c}` has size: {}, total sides: {total_sides} = ({top_sides} + {bottom_sides} + {left_sides} + {right_sides})", current_group.len());

        result += total_sides * current_group.len();
    }

    result
}
