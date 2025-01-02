pub fn part1(input: &str) -> usize
{
    let char_table = input.lines()
        .map( |line|  line.chars().collect::<Vec<char>>() )
        .collect::<Vec<Vec<char>>>();

    let list_of_stars = char_table.iter()
        .enumerate()
        .flat_map( |(line_idx, line)|
        {
            line.iter()
                .enumerate()
                .map( move |(col_idx, ch)| (line_idx, col_idx, ch) )
        })
        .map( |(line_idx , col_idx, ch)|
        {
            match *ch {
                'X' => get_star(&char_table, line_idx, col_idx),
                _   => vec!()
            }
        })
        .collect::<Vec<Vec<String>>>();

    list_of_stars.iter()
        .flat_map( |string_list| string_list.iter() )
        .filter( |txt| *txt == "XMAS" )
        .count()
}


fn get_star(char_map: &Vec<Vec<char>>, ridx: usize, cidx: usize) -> Vec<String> {
    let mut list = vec!{};

    list.push(get_characters_in_direction(char_map, 4, ridx, cidx, -1, -1));
    list.push(get_characters_in_direction(char_map, 4, ridx, cidx, -1,  0));
    list.push(get_characters_in_direction(char_map, 4, ridx, cidx, -1,  1));
    list.push(get_characters_in_direction(char_map, 4, ridx, cidx,  0, -1));
    list.push(get_characters_in_direction(char_map, 4, ridx, cidx,  0,  1));
    list.push(get_characters_in_direction(char_map, 4, ridx, cidx,  1, -1));
    list.push(get_characters_in_direction(char_map, 4, ridx, cidx,  1,  0));
    list.push(get_characters_in_direction(char_map, 4, ridx, cidx,  1,  1));

    list.into_iter()
        .filter( |o| o.is_some() )
        .map( |o| o.unwrap() )
        .collect()
}

fn get_characters_in_direction(char_map: &Vec<Vec<char>>, nchars: usize, row: usize, col: usize, dr: i32, dc: i32) -> Option<String> {
    let height = char_map.len();
    let width = char_map[0].len();

    if  !(0..height).contains(&row) ||
        !(0..width ).contains(&col) ||
        !(0..height as i32).contains(&((row as i32) + ((nchars - 1) as i32) * dr)) ||
        !(0..width  as i32).contains(&((col as i32) + ((nchars - 1) as i32) * dc))
    {
        return None;
    }

    let mut result = Vec::with_capacity(nchars);

    for idx in 0..nchars {
        result.push(char_map[((row as i32) + (idx as i32) * dr) as usize][((col as i32) + (idx as i32) * dc) as usize])
    }
    Some(result.into_iter().collect::<String>())
}

pub fn part2(input: &str) -> usize
{
    let char_table = input.lines()
        .map( |line|  line.chars().collect::<Vec<char>>() )
        .collect::<Vec<Vec<char>>>();

    let list_of_diagonals = char_table.iter()
        .enumerate()
        .flat_map( |(line_idx, line)|
        {
            line.iter()
                .enumerate()
                .map( move |(col_idx, ch)| (line_idx, col_idx, ch) )
        })
        .map( |(line_idx , col_idx, ch)|
        {
            match *ch {
                'A' => get_diagonals(&char_table, line_idx, col_idx),
                _   => vec!()
            }
        })
        .collect::<Vec<Vec<String>>>();

    // Check how many locations have 2 diagonals that contain "MAS"
    list_of_diagonals.iter()
        .map( |string_list|
        {
            string_list.iter()
                .filter( |txt| *txt == "MAS" )
                .count() == 2
        })
        .filter( |b| *b)
        .count()
}

fn get_diagonals(char_map: &Vec<Vec<char>>, ridx: usize, cidx: usize) -> Vec<String> {
    let mut list = vec!{};

    // Prevent usize underflow by returning an empty vector
    if ridx == 0 || cidx == 0 {
        return vec!();
    }

    list.push(get_characters_in_direction(char_map, 3, ridx + 1, cidx + 1, -1, -1));
    list.push(get_characters_in_direction(char_map, 3, ridx + 1, cidx - 1, -1,  1));
    list.push(get_characters_in_direction(char_map, 3, ridx - 1, cidx + 1,  1, -1));
    list.push(get_characters_in_direction(char_map, 3, ridx - 1, cidx - 1,  1,  1));

    list.into_iter()
        .filter( |o| o.is_some() )
        .map( |o| o.unwrap() )
        .collect()
}