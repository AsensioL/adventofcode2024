use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use rectangle::{Rectangle, Rectangular, RectangularData, Direction};


#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct State {
    coord: (usize, usize),
    dir: Direction,
}

impl State {
    fn new(coord: &(usize, usize), dir: Direction) -> Self {
        Self {coord: *coord, dir}
    }
}

#[derive(Copy, Clone)]
struct Transition {
    initial_state: State,
    new_state: State,
}

impl Transition {
    fn new(initial_state: &State, new_coord: (usize, usize), dir: Direction) -> Self {
        Self{initial_state: *initial_state, new_state: State::new(&new_coord, dir)}
    }
}


trait Warehouse: RectangularData<char> + Rectangular {
    fn is_wall(&self, coord: &(usize, usize)) -> bool {
        let Ok(ch) = self.get(coord) else { return false; };
        *ch == '#'
    }

    fn is_start(&self, coord: &(usize, usize)) -> bool {
        let Ok(ch) = self.get(coord) else { return false; };
        *ch == 'S'
    }

    fn is_end(&self, coord: &(usize, usize)) -> bool {
        let Ok(ch) = self.get(coord) else { return false; };
        *ch == 'E'
    }

    fn transitions_from(&self, state: &State) -> [Option<Transition>; 4] {
        let coord = &state.coord;
        let mut ret = [None; 4];

        // For each of 4 possible directions
        ret.iter_mut()
            .zip( [Direction::Up, Direction::Down, Direction::Left, Direction::Right] )
            .for_each( |(sub_result, direction)| {

                // If next coordinate in that direction is not valid, keep it as None
                let Some(new_coord) = self.next_coord(coord, direction) else {
                    return;
                };

                // If next coordinate in that direction is a wall, keep it as None
                if self.is_wall(&new_coord) {
                    return;
                }

                *sub_result = Some(Transition::new(state, new_coord, direction));

            });

        ret
    }
}
impl Warehouse for Rectangle<char> {}


pub fn part1(input: &str) -> u32
{
    // Parse input
    let maze = Rectangle::from_char_str(input)
        .map_err( |error|  panic!("Failed to parse rectangle. Reason: {error}") )
        .unwrap();

    // Define initial state (position and direction)
    let start_position = maze.iter_coord()
        .find(|coord| maze.is_start(coord) )
        .expect("Failed to find start position");
    let start_state = State::new(&start_position, Direction::Right);

    // Define final state (direction only)
    let end_position = maze.iter_coord()
       .find(|coord| maze.is_end(coord) )
       .expect("Failed to find end position");

    // Keep track of previous positions with a hashmap that tracks cost
    let mut state_map = HashMap::new();
    state_map.insert(start_state, 0);

    // Keep the frontier of places that must be visited next
    let mut transitions_pending = Vec::new();
    transitions_pending.extend( maze.transitions_from(&start_state).into_iter().flatten() );

    // Pop transitions until empty
    while let Some(Transition{initial_state, new_state}) = transitions_pending.pop() {
        //println!("Popping transition: {initial_state:?} -> {new_state:?}");

        // Get the steps it took to arrive to the initial state (this must succeed)
        let initial_cost = state_map.get(&initial_state).expect("Initial state must exist");

        // Calculate new cost to arrive to new position
        let new_steps_cost = if initial_state.dir.is_horizontal() == new_state.dir.is_horizontal() {
            initial_cost + 1
        } else {
            initial_cost + 1001
        };

        if let Some(existing_steps_cost) = state_map.get_mut(&new_state) {

            // Compare existing and new costs to arrive to new position
            if new_steps_cost >= *existing_steps_cost {
                // This might be a closed loop, or a worse way to get to the same place
                continue;
            }
            *existing_steps_cost = new_steps_cost;
        }
        else {
            state_map.insert(new_state, new_steps_cost);
        }

        // If the state's coord is not the end, push its transitions into pending list
        // Don't move back by skipping transitions that go the current initial position
        if !maze.is_end(&new_state.coord) {
            let new_transitions = maze.transitions_from(&new_state).into_iter()
                .flatten()
                .filter(|t| t.new_state.coord != initial_state.coord);

            transitions_pending.extend(new_transitions);
        }
    }

    // There are 4 different ways to get to the end state (ie: looking in each direction), get the min trip cost
    [Direction::Up, Direction::Down, Direction::Left, Direction::Right].into_iter()
        .map(|d| State::new(&end_position, d) )
        .filter_map(|s: State| state_map.get(&s) )
        .copied()
        .min()
        .unwrap()
}


#[derive(Debug)]
struct StepInfo {
    dir: Direction,
    cost: usize,
    parent_indices: Vec<usize>
}

// TODO: Consider rewriting using petgraph

pub fn part2(input: &str) -> usize
{
    // Parse input
    let maze = Rectangle::from_char_str(input)
        .map_err( |error|  panic!("Failed to parse rectangle. Reason: {error}") )
        .unwrap();

    // Define initial state (position and direction)
    let start_position = maze.iter_coord()
        .find(|coord| maze.is_start(coord) )
        .expect("Failed to find start position");
    let start_state = State::new(&start_position, Direction::Right);

    // Define final state (direction only)
    let end_position = maze.iter_coord()
       .find(|coord| maze.is_end(coord) )
       .expect("Failed to find end position");

    // Keep track of previous movements with a hashmap that tracks
    let mut state_map = HashMap::with_capacity(maze.height * maze.width); // Contains indices to steps_graph
    let mut steps_graph = Vec::with_capacity(maze.height * maze.width);
    steps_graph.push(StepInfo { dir: start_state.dir, cost: 0, parent_indices: Vec::new() } );
    state_map.insert(start_state, 0_usize);

    // Keep the frontier of places that must be visited next
    let mut transitions_pending = Vec::with_capacity(maze.height * maze.width);
    transitions_pending.extend( maze.transitions_from(&start_state).into_iter().flatten() );

    // Pop transitions until empty
    while let Some(Transition{initial_state, new_state}) = transitions_pending.pop() {
        //println!("Popping transition: {initial_state:?} -> {new_state:?}");

        // Get the last step taken to arrive to the initial state (this must succeed)
        let initial_step_idx = *state_map.get(&initial_state).expect("Initial state must exist");
        let initial_step_info = steps_graph.get(initial_step_idx).unwrap();

        // Calculate cost to arrive to new position
        let new_steps_cost = if initial_state.dir.is_horizontal() == new_state.dir.is_horizontal() {
            initial_step_info.cost + 1
        } else {
            initial_step_info.cost + 1001
        };

        // If we have already visited this state
        if let Some(& existintg_step_idx) = state_map.get(&new_state) {
            let existing_step_info = steps_graph.get_mut(existintg_step_idx).unwrap();

            // Compare existing and new costs to arrive to new position
            match new_steps_cost.cmp(&existing_step_info.cost) {
                Ordering::Greater => continue, // This might be a closed loop, or a worse way to get to the same place
                Ordering::Less => {
                    // This is a better path, replace existing data with new (only 1 parent now)
                    existing_step_info.cost = new_steps_cost;
                    existing_step_info.dir = new_state.dir;
                    existing_step_info.parent_indices[0] = initial_step_idx;
                    existing_step_info.parent_indices.truncate(1);
                },
                Ordering::Equal => {
                    // This is an equivalent path, push a new parent (if new)
                    if !existing_step_info.parent_indices.contains(&initial_step_idx) {
                        existing_step_info.parent_indices.push(initial_step_idx);
                    }

                    // Also, don't queue transitions from here because they have already been queued
                    continue;
                }
            }
        }
        else {
            // This position has not been visited yet
            // Create a new step in the step graph and add the (state -> step_idx) to the state map
            steps_graph.push( StepInfo {cost: new_steps_cost, dir: new_state.dir, parent_indices: vec![initial_step_idx]} );
            state_map.insert(new_state, steps_graph.len() - 1 );
        }

        // If the state's coord is not the end, push its transitions into pending list
        // Don't move back by skipping transitions that go the current initial position
        if !maze.is_end(&new_state.coord) {
            let new_transitions = maze.transitions_from(&new_state).into_iter()
                .flatten()
                .filter(|t| t.new_state.coord != initial_state.coord);

            transitions_pending.extend(new_transitions);
        }
    }

    // There are 4 different ways to get to the end state (ie: looking in each direction), get their step indices
    let final_step_indices = [Direction::Up, Direction::Down, Direction::Left, Direction::Right].into_iter()
        .map(|d| State::new(&end_position, d) )
        .filter_map(|s: State| state_map.get(&s) )
        .copied()
        .collect::<Vec<_>>();

    // Get the min trip cost
    let min_cost = final_step_indices.iter()
        .map(|step_idx| steps_graph.get(*step_idx).unwrap() )
        .map(|step_info| step_info.cost )
        .min()
        .unwrap();

    // Get final steps that have the min cost (there may be multiple ones)
    let best_final_steps_indices = final_step_indices.iter()
        .filter(|&&step_idx| steps_graph.get(step_idx).unwrap().cost == min_cost )
        .copied()
        .collect::<Vec<_>>();

    // Find the best positions by walking back from the final position
    let best_positions = best_final_steps_indices.iter()
        .flat_map( |&best_final_step_idx| {

            // Iterate from the end_position in reverse, appending every position to
            // a set (using Vec for speed because there are few duplicates)
            // The paths are divergent so also keep track using a pending list
            let mut pending_steps = vec![(end_position, best_final_step_idx)];
            let mut best_positions = Vec::new();

            // Pop items from the list until it is empty
            while let Some((position, step_idx)) = pending_steps.pop() {
                let step_info = steps_graph.get(step_idx).unwrap();
                best_positions.push(position);

                pending_steps.extend(
                    step_info.parent_indices.iter()
                        .map(|parent_step_idx|  {
                            let parent_step_info = steps_graph.get(*parent_step_idx).unwrap();
                            let parent_coord = maze.next_coord(&position, parent_step_info.dir.reverse()).unwrap();
                            (parent_coord, *parent_step_idx)
                        })
                );
            }
            best_positions.into_iter()
        })
        .collect::<HashSet<(usize, usize)>>();

    // Debug print statement
    // let output = maze.to_string_with_changes(
    //     |coord, c| if best_positions.contains(&coord) { 'O' } else { c } );
    // println!("{output}");

    // Count unique positions
    best_positions.len()
}
