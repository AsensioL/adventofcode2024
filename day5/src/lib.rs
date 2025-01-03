use regex::Regex;

static SEPARATOR: &str = "

";

pub fn part1(input: &str) -> i32
{
    // Split text into rules and pages
    let (rules_txt, pages_txt) = {
        let sep_idx = input.find(SEPARATOR)
            .expect("Failed to split input text. Line endings (CRLF vs LF) might be wrong");
        let sep_len = SEPARATOR.len();
        (&input[0..sep_idx], &input[sep_idx + sep_len..])
    };
    // println!("--{rules_txt}--");
    // println!("--{pages_txt}--");

    // Parse rules into a Vec of tuples
    let rules = Regex::new(r"(\d+)\|(\d+)").unwrap()
        .captures_iter(rules_txt)
        .map( |c| {
            let n1 = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let n2 = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
            (n1, n2)
        })
        .collect::<Vec<(i32,i32)>>();
    // println!("--{rules:?}--");

    // Parse pages into a Vec<Vec> of i32
    let page_re = Regex::new(r"\d+").unwrap();
    let pages : Vec<_> = pages_txt.lines()
        .map( |line| page_re.captures_iter(line) )
        .map( |c| c.map( |o| o.get(0).unwrap().as_str().parse::<i32>().unwrap() ).collect::<Vec<i32>>() )
        .collect();

    // Calculate "valid" pages
    // let valid_pages = pages.iter()
    //     .filter( |page| page_meets_rules(page, &rules) )
    //     .collect::<Vec<_>>();
    // println!("--{valid_pages:?}--");

    // Sum the middle page of "valid" pages
    pages.iter()
        .filter( |page| page_meets_rules(page, &rules) )
        .map( |page| page[page.len()/2] )
        .sum()
}

// Check if a page meets the rules
fn page_meets_rules(page: &Vec<i32>, rules: &Vec<(i32,i32)>) -> bool
{
    for (n1, n2) in rules {
        match (find_item(page, n1), find_item(page, n2)) {
            (Some(pos1), Some(pos2)) => if pos2 < pos1 { return false; }
            _ => continue
        }
    }
    true
}



// Take a reference to an array-like (ie: like Vec) and to an Item
// Return Some(<location in the array>) or None (if not present)
use std::ops::Deref;
fn find_item<I, T>(iterable: &I, item: &T) -> Option<usize>
where   I: Deref<Target = [T]>,
        T: std::cmp::PartialEq
{
    iterable.iter().position( |i| i == item )
}

pub fn part2(input: &str) -> i32
{
    // Split text into rules and pages
    let (rules_txt, pages_txt) = {
        let sep_idx = input.find(SEPARATOR)
            .expect("Failed to split input text. Line endings (CRLF vs LF) might be wrong");
        let sep_len = SEPARATOR.len();
        (&input[0..sep_idx], &input[sep_idx + sep_len..])
    };
    // println!("--{rules_txt}--");
    // println!("--{pages_txt}--");

    // Parse rules into a Vec of tuples
    let rules = Regex::new(r"(\d+)\|(\d+)").unwrap()
        .captures_iter(rules_txt)
        .map( |c| {
            let n1 = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let n2 = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
            (n1, n2)
        })
        .collect::<Vec<(i32,i32)>>();
    // println!("--{rules:?}--");

    // Parse pages into a Vec<Vec> of i32
    let page_re = Regex::new(r"\d+").unwrap();
    let pages : Vec<_> = pages_txt.lines()
        .map( |line| page_re.captures_iter(line) )
        .map( |c| c.map( |o| o.get(0).unwrap().as_str().parse::<i32>().unwrap() ).collect::<Vec<i32>>() )
        .collect();

    // Sum the middle page of "invalid" pages after reordering
    pages.iter()
        .filter( |page| !page_meets_rules(page, &rules) )
        .map( |page| fix_page_to_meet_rules(page, &rules) )
        .map( |page| page[page.len()/2] )
        .sum()
}

// Check if a page meets the rules
fn fix_page_to_meet_rules(page: &Vec<i32>, rules: &Vec<(i32,i32)>) -> Vec<i32>
{
    let mut new_page = page.clone();

    while !page_meets_rules(&new_page, rules) {
        // Iterate over every rule
        for (n1, n2) in rules
        {
            // If they don't meet the rules, move left the right side
            match (find_item(&new_page, n1), find_item(&new_page, n2))
            {
                (Some(pos1), Some(pos2)) =>
                {
                    // This item breaks the rule, mutate the vector and exit the for loop
                    if pos2 < pos1
                    {
                        // println!("Vector status: {new_page:?}");
                        // println!("Broken rule: {n1} should be left of {n2} but their respective positions were {pos1} and {pos2}");
                        let misplaced_element = new_page.remove(pos1);
                        new_page.insert(pos2, misplaced_element);
                        break;
                    }
                }
                _ => continue
            }
        }
    }

    new_page
}