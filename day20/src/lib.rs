use std::collections::{HashMap, HashSet};

use rectangle::{Rectangle, Rectangular, RectangularData};
use petgraph::{algo::dijkstra, graph::{NodeIndex, UnGraph}};

type Coord = (usize, usize);

fn rectangle_to_graph(rectangle: &Rectangle<char>, start_position: Coord) -> (UnGraph<Coord, ()>, HashMap<Coord, NodeIndex>){
    // Start a graph
    let mut graph = UnGraph::new_undirected();

    // Start a "seen node map"
    let mut node_map = HashMap::new();
    let mut processed_coords = HashSet::new();

    // Start a queue with our initial element and add it to the graph and to the "seen nodes map"
    let mut queue=  vec!(start_position);
    let start_node_idx = graph.add_node(start_position);
    node_map.insert(start_position, start_node_idx);
    processed_coords.insert(start_position);

    // Pop elements from the queue
    while let Some(coord) = queue.pop()
    {
        // Get NodeIdx and mark as processed
        let current_node_idx = *node_map.get(&coord).expect("Couldn't find coord in node map");
        processed_coords.insert(coord);

        // println!("Popped coord {coord:?}");

        // Get its unprocessed adjacent cells
        for adjacent_coord in rectangle.adjacent_coordinates(&coord).into_iter()
            .flatten()
            .filter(|adj_coord| *rectangle.get(adj_coord).unwrap() != '#' )
            .filter( |adj_coord| !processed_coords.contains(adj_coord) )
        {
            // println!("Processing adjacent cell {adjacent_coord:?}");

            // If it has not been processed, it might already exist on the graph
            let new_node_idx = *node_map.entry(adjacent_coord)
                .or_insert_with( || graph.add_node(adjacent_coord) );

            // Add edge from the node being processed to the current adjacent (use update to avoid duplication)
            graph.update_edge(current_node_idx, new_node_idx, ());

            // Queue adjacent for later processing
            queue.push(adjacent_coord);
        }
    }

    (graph, node_map)
}

pub fn part1(input: &str) -> usize
{
    let racetrack = Rectangle::from_char_str(input).unwrap();

    let start_position = racetrack.iter_coord_and_data()
        .find(|(_, &c)| c == 'S' )
        .map(|(coord, _)| coord)
        .unwrap();
    let end_position = racetrack.iter_coord_and_data()
        .find(|(_, &c)| c == 'E' )
        .map(|(coord, _)| coord)
        .unwrap();

    // println!("{}", racetrack.to_string());
    // println!("{start_position:?}");
    // println!("{end_position:?}");

    let (graph, node_map) = rectangle_to_graph(&racetrack, start_position);

    let start_position_idx = *node_map.get(&start_position).expect("Could not find NodeIndex for start_position");
    let end_position_idx = *node_map.get(&end_position).expect("Could not find NodeIndex for end_position");

    let distances_from_start = dijkstra(&graph, start_position_idx, None, |_| 1_usize);
    let distances_from_end = dijkstra(&graph, end_position_idx, None, |_| 1_usize);

    let shortest_path_without_shortcuts = *distances_from_start.get(&end_position_idx)
        .expect("Dijkstra did not find a path from start to end");
    // println!("Shortest distance without shortcuts: {}", shortest_path_without_shortcuts);

    let mut shortcut_map: HashMap<_, Vec<Coord>> = HashMap::new();

    // Iterate over every wall coordinate
    for wall_coord in racetrack.iter_coord_and_data()
        .filter(|(_, &c)| c == '#' )
        .map(|(coord, _)| coord )
    {
        // Get wall coordinates that have tracks around them
        let tracks_adjacent_to_wall = racetrack.adjacent_coordinates(&wall_coord).into_iter()
            .flatten()
            .filter(|coord| *racetrack.get(coord).unwrap() != '#' )
            .collect::<Vec<_>>();

        // If the wall has less than 2 tracks around, removing the wall would be a dead end, so skip it
        if tracks_adjacent_to_wall.len() < 2 {
            continue;
        }

        // From all the tracks adjacent to this wall find the shortest distance to the start and to the end
        let shortest_distance_to_start = tracks_adjacent_to_wall.iter()
            .map(|coord| node_map.get(coord).unwrap() )
            .map( |coord_idx| distances_from_start.get(coord_idx).unwrap() )
            .min()
            .unwrap();
        let shortest_distance_to_end = tracks_adjacent_to_wall.iter()
            .map(|coord| node_map.get(coord).unwrap() )
            .map( |coord_idx| distances_from_end.get(coord_idx).unwrap() )
            .min()
            .unwrap();

        // Calculate shortcut distance. Add 2 for the unaccounted steps to jump into and out-of the wall coordinate
        let shortest_path_with_shortcut = shortest_distance_to_start + 2 + shortest_distance_to_end;

        // Store the wall removed and how many picoseconds it saves (this is only useful for verification with the example)
        if shortest_path_with_shortcut < shortest_path_without_shortcuts {
            shortcut_map.entry(shortest_path_without_shortcuts - shortest_path_with_shortcut)
                .and_modify(|coord_list| coord_list.push(wall_coord) )
                .or_insert_with(|| vec![wall_coord] );

        }
    }

    // Turn the map into a list for sorting
    let mut shortcut_list = shortcut_map.iter()
        .collect::<Vec<_>>();
    shortcut_list.sort_by_key(|entry| *entry.0 );

    // Print all the savings, like in the example (for verification)
    // for (savings, coord_list) in shortcut_list.iter() {
    //     println!("There are {} cheats that save {} picoseconds", coord_list.len(), *savings);
    // }

    // Return how many shortcuts save at least 100 picoseconds
    shortcut_list.iter()
        .fold( 0, |total, &(savings, shortcuts)| total + if *savings >= 100 { shortcuts.len() } else { 0 } )
}

/// Measure how many steps in the horizontal direction plus steps in the vertical
/// direction it takes to go from a to b.
fn taxicab_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let delta_rows = if a.0 <= b.0 { b.0 - a.0 } else { a.0 - b.0 };
    let delta_cols = if a.1 <= b.1 { b.1 - a.1 } else { a.1 - b.1 };
    delta_rows + delta_cols
}

pub fn part2(input: &str) -> usize
{
    let racetrack = Rectangle::from_char_str(input).unwrap();

    let start_position = racetrack.iter_coord_and_data()
        .find(|(_, &c)| c == 'S' )
        .map(|(coord, _)| coord)
        .unwrap();
    let end_position = racetrack.iter_coord_and_data()
        .find(|(_, &c)| c == 'E' )
        .map(|(coord, _)| coord)
        .unwrap();

    let (graph, node_map) = rectangle_to_graph(&racetrack, start_position);

    let start_position_idx = *node_map.get(&start_position).expect("Could not find NodeIndex for start_position");
    let end_position_idx = *node_map.get(&end_position).expect("Could not find NodeIndex for end_position");

    let distances_from_start = dijkstra(&graph, start_position_idx, None, |_| 1_usize);
    let distances_from_end = dijkstra(&graph, end_position_idx, None, |_| 1_usize);

    let shortest_path_without_shortcuts = *distances_from_start.get(&end_position_idx)
        .expect("Dijkstra did not find a path from start to end");

    // Map of saved distance -> vector of (entry, exit) indices
    let mut shortcut_map: HashMap<_, Vec<(NodeIndex,NodeIndex)>> = HashMap::new();

    // Iterate over every racetrack coordinate to enter the cheat mode
    for racetrack_cheat_enter_coord in racetrack.iter_coord_and_data()
        .filter(|(_, &c)| c != '#' )
        .map(|(coord, _)| coord )
    {
        // Get distance to from start to this racetrack_cheat_enter_coord
        let racetrack_cheat_enter_coord_idx = *node_map.get(&racetrack_cheat_enter_coord).unwrap();
        let distance_to_start = distances_from_start.get(&racetrack_cheat_enter_coord_idx).unwrap();

        // Iterate over every track around this coordinate within the allowed distance (20 microseconds)
        // to exit the cheat mode.
        for racetrack_cheat_exit_coord in racetrack.iter_coord_and_data_around_coord(&racetrack_cheat_enter_coord, 20)
            // The above returns a square section around +/- 20, but we only want the coordinates where the taxicab distance is 20, so filter the
            .filter( | (coord, _)| taxicab_distance(&racetrack_cheat_enter_coord, coord) <= 20)
            .filter( |(_, c)| **c != '#' ) // Ignore walls
            .filter( | (coord, _)| *coord != racetrack_cheat_enter_coord ) // Ignore entry point too
            .map( |(coord, _)| coord )
        {
            // Calculate the shortcut length
            let shortcut_length = taxicab_distance(&racetrack_cheat_enter_coord, &racetrack_cheat_exit_coord);

            // Get distance to from the racetrack_cheat_exit_coord to this the end
            let racetrack_cheat_exit_coord_idx = *node_map.get(&racetrack_cheat_exit_coord).unwrap();
            let distance_to_end = distances_from_end.get(&racetrack_cheat_exit_coord_idx).unwrap();

            // Total distance with shortcut
            let distance_with_this_shortcut = distance_to_start + shortcut_length + distance_to_end;

            // If the shortcut is better than no shortcut, then save it
            if distance_with_this_shortcut < shortest_path_without_shortcuts {
                let saved_distance = shortest_path_without_shortcuts - distance_with_this_shortcut;

                shortcut_map.entry(saved_distance)
                .and_modify(|coord_list| coord_list.push((racetrack_cheat_enter_coord_idx, racetrack_cheat_exit_coord_idx)) )
                .or_insert_with(|| vec![(racetrack_cheat_enter_coord_idx, racetrack_cheat_exit_coord_idx)] );
            }
        }
    }

    // Turn the map into a list for sorting
    let mut shortcut_list = shortcut_map.iter()
        .collect::<Vec<_>>();
    shortcut_list.sort_by_key(|entry| *entry.0 );

    // Print all the savings, like in the example (for verification)
    // for (savings, coord_list) in shortcut_list.iter() {
    //     println!("There are {} cheats that save {} picoseconds", coord_list.len(), *savings);
    // }

    // Info about (one of) the best solution
    // println!("Start coordinate: {start_position:?}");
    // println!("End   coordinate: {end_position:?}");
    // println!("Distance between them: {}", taxicab_distance(&start_position, &end_position));
    // let best_solution = shortcut_list.last().unwrap().1.first().unwrap();
    // let best_entry = graph.node_weight(best_solution.0).unwrap();
    // let best_exit = graph.node_weight(best_solution.1).unwrap();
    // println!("Best cheat: Entry at {:?}, Exit at {:?}, Length is {}", best_entry, best_exit, taxicab_distance(best_entry, best_exit));

    // Return how many shortcuts save at least 100 picoseconds
    shortcut_list.iter()
        .fold( 0, |total, &(savings, shortcuts)| total + if *savings >= 100 { shortcuts.len() } else { 0 } )
}
