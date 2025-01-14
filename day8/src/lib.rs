use std::collections::HashMap;
use std::collections::HashSet;

use itertools::iproduct;


fn parse_input(input: &str) -> (HashMap<char, Vec<(i32, i32)>>, (i32, i32))
{
    let mut antennas_locations: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let char_table = input.lines()
        .map( |line|  line.chars().collect::<Vec<char>>() )
        .collect::<Vec<Vec<char>>>();

    let height = char_table.len() as i32;
    let width = char_table[0].len() as i32;

    char_table.iter()
        .enumerate()
        .flat_map( |(line_idx, line)|
        {
            line.iter()
                .enumerate()
                .filter( |(_, &ch)| ch != '.' )
                .map( move |(col_idx, &ch)| (line_idx as i32, col_idx as i32, ch) )
        })
        .for_each( |(line_idx , col_idx, ch)|
        {
            antennas_locations.entry(ch)
                .and_modify(|e| e.push((col_idx, line_idx)) )
                .or_insert_with(|| vec!{(col_idx, line_idx)} );
        });

    (antennas_locations, (width, height))
}

pub fn part1(input: &str) -> usize
{
    let (antennas_locations, (width, height)) = parse_input(input);

    let antinodes = antennas_locations.keys()
        .flat_map( |ch| iproduct!(antennas_locations[ch].iter(), antennas_locations[ch].iter()) )
        .filter( |(t1, t2)| t1 != t2 )
        .map( | (t1, t2)| (2*t2.0 - t1.0, 2*t2.1 - t1.1) )
        .filter( |t| (0..width).contains(&t.0) && (0..height).contains(&t.1) )
        .collect::<HashSet<(i32,i32)>>();

    antinodes.len()
}

pub fn part2(input: &str) -> usize
{
    let (antennas_locations, (width, height)) = parse_input(input);

    let antinodes = antennas_locations.keys()
        .flat_map( |ch| iproduct!(antennas_locations[ch].iter(), antennas_locations[ch].iter()) )
        .filter( |(t1, t2)| t1 != t2 )
        .flat_map( | (t1, t2)|
        {
            let mut candidates = vec!{};

            let mut p = *t2;
            let d = (t2.0 - t1.0, t2.1 - t1.1);

            while (0..width).contains(&p.0) && (0..height).contains(&p.1)
            {
                candidates.push(p);
                p = (p.0 + d.0, p.1 + d.1);
            }

            candidates.into_iter()
        })
        .collect::<HashSet<(i32,i32)>>();


    // DEBUG PRINT
    //let antennas = antennas_locations.values()
    //    .flat_map( |v| v.clone().into_iter() )
    //    .collect::<HashSet<(i32,i32)>>();
    //
    //input.lines()
    //    .enumerate()
    //    .for_each( |(line_idx, line)|
    //    {
    //        line.chars()
    //            .enumerate()
    //            .for_each( |(col_idx, ch)|
    //            {
    //                let coord = (col_idx as i32, line_idx as i32);
    //                if antennas.contains(&coord)
    //                {
    //                    print!("{ch}");
    //                }
    //                else if antinodes.contains(&coord)
    //                {
    //                    print!("#");
    //                }
    //                else
    //                {
    //                    print!(".");
    //                }
    //            });
    //        print!("\n");
    //    });

    antinodes.len()
}
