use std::collections::HashMap;

use itertools::{self, Itertools};
use rectangle::{Rectangle, RectangularData};
use petgraph::{algo::astar, graph::UnGraph, prelude::StableUnGraph, stable_graph::NodeIndex};


pub fn part1(input: &str, width: usize, height: usize, fall_count: usize) -> usize
{
    // Parse input
    let positions = input.lines()
        .map(|line|
        {
            line.split(',')
                .map(|n| n.parse::<usize>().unwrap() )
                .collect_tuple::<(usize, usize)>()
                .map(|t| (t.1, t.0) ) // Flip input because we use (row, col) instead of (x, y)
                .unwrap()
        })
        .collect::<Vec<_>>();
    //println!("{positions:?}");

    // Update with fallen bytes
    let mut region = Rectangle::from_repeated_element('.', height, width);
    positions.iter()
        .take(fall_count)
        .for_each(|coord|
    {
        region.set(coord, &'#').unwrap();
    });
    //println!("{}", region.to_string());

    // Build graph
    let mut graph = UnGraph::new_undirected();
    let mut node_map = HashMap::new();

    // Add nodes to the graph
    for (coord, &c) in region.iter_coord_and_data() {
        if c == '.' {
                let node_idx = graph.add_node(coord);
                node_map.insert(coord, node_idx);
        }
    }

    // Add edges to the graph
    for &a_node_idx in node_map.values() {
        let a_coord = graph.node_weight(a_node_idx).unwrap();
        for b_coord in region.adjacent_coordinates_matching(a_coord, &'.').iter().flatten() {
            let b_node_idx = *node_map.get(b_coord).unwrap();
            graph.add_edge(a_node_idx, b_node_idx, 1);
        }
    }

    // Locate start and end nodes
    let goal_coord = (height - 1, width - 1);
    let start_node_idx = *node_map.get(&(0, 0)).unwrap();
    let goal_node_idx  = *node_map.get(&goal_coord).unwrap();

    // Run the A* algorithm
    let (cost, _path) = astar(
        &graph,
        start_node_idx,
        |node_idx| node_idx == goal_node_idx,
        |e| *e.weight(),
        |node_idx| {
            let node_coord = graph.node_weight(node_idx).unwrap();
            let delta = (height - node_coord.0, width - node_coord.1);
            delta.0 + delta.1
        }).expect("Expected A* to work");

    // Optionally print the correct path
    // let path_coord = _path.iter()
    //     .map(|node_idx| *graph.node_weight(*node_idx).unwrap())
    //     .collect::<Vec<_>>();
    // let result_string = region.to_string_with_changes(|coord, c| {
    //     if c == '.' && path_coord.contains(&coord) {
    //         'O'
    //     } else {
    //         c
    //     }
    // });
    // println!("{result_string}");

    cost
}

pub fn part2(input: &str, width: usize, height: usize, initial_fall_count: usize) -> String
{

    // Parse input
    let positions = input.lines()
        .map(|line|
        {
            line.split(',')
                .map(|n| n.parse::<usize>().unwrap() )
                .collect_tuple::<(usize, usize)>()
                .map(|t| (t.1, t.0) ) // Flip input because we use (row, col) instead of (x, y)
                .unwrap()
        })
        .collect::<Vec<_>>();
    //println!("{positions:?}");

    // Update with fallen bytes
    let mut region = Rectangle::from_repeated_element('.', height, width);
    positions.iter()
        .take(initial_fall_count)
        .for_each(|coord|
    {
        region.set(coord, &'#').unwrap();
    });
    //println!("{}", region.to_string());

    // Build graph
    let mut graph = StableUnGraph::with_capacity(width*height, width*height*4);
    let mut node_map: HashMap<(usize, usize), NodeIndex<u16>> = HashMap::new();

    // Add nodes to the graph
    for (coord, &c) in region.iter_coord_and_data() {
        if c == '.' {
                let node_idx = graph.add_node(coord);
                node_map.insert(coord, node_idx);
        }
    }

    // Add edges to the graph
    for &a_node_idx in node_map.values() {
        let a_coord = graph.node_weight(a_node_idx).unwrap();
        for b_coord in region.adjacent_coordinates_matching(a_coord, &'.').iter().flatten() {
            let b_node_idx = *node_map.get(b_coord).unwrap();
            graph.add_edge(a_node_idx, b_node_idx, 1);
        }
    }

    // Locate start and end nodes
    let goal_coord = (height - 1, width - 1);
    let start_node_idx = *node_map.get(&(0, 0)).unwrap();
    let goal_node_idx  = *node_map.get(&goal_coord).unwrap();

    // Run the A* algorithm initially
    let (_, mut path) = astar(
        &graph,
        start_node_idx,
        |node_idx| node_idx == goal_node_idx,
        |e| *e.weight(),
        |node_idx| {
            let node_coord = graph.node_weight(node_idx).unwrap();
            let delta = (height - node_coord.0, width - node_coord.1);
            delta.0 + delta.1
        }).expect("Expected A* to work");

    let mut path_coord = path.iter()
        .map(|node_idx| *graph.node_weight(*node_idx).unwrap())
        .collect::<Vec<_>>();

    // For each additionally fallen byte...
    for new_coord in positions.iter().skip(initial_fall_count) {

        // Remove the node
        let new_node_idx = node_map.get(new_coord).unwrap();
        graph.remove_node(*new_node_idx);

        // Only re-run A* if the fallen byte lands on the optimal path
        if path_coord.contains(new_coord) {
            let result = astar(
                &graph,
                start_node_idx,
                |node_idx| node_idx == goal_node_idx,
                |e| *e.weight(),
                |node_idx| {
                    let node_coord = graph.node_weight(node_idx).unwrap();
                    let delta = (height - node_coord.0, width - node_coord.1);
                    delta.0 + delta.1
                });

            // If A* does not find a solution, the path just broke
            if result.is_none() {
                // Optionally update the region to show the breaking-byte (drawing only)
                // region.set(new_coord, &'@').unwrap();
                // println!("{}", region.to_string());
                return format!("{},{}", new_coord.1, new_coord.0); // Print reversed because we have been used reversed coordinates
            }
            else {
                // Take the new path as the new optimal path
                (_, path) = result.unwrap();
                path_coord = path.iter()
                    .map(|node_idx| *graph.node_weight(*node_idx).unwrap())
                    .collect::<Vec<_>>();

                // Update the region (drawing only)
                region.set(new_coord, &'#').unwrap();
            }
        }
    }

    "error".to_string()
}
