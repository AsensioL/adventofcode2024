use regex::Regex;

pub fn day3_part1(text: &str) -> i32
{
    let re: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(text)
        .map( |c|
        {
            (c.get(1).map( |m| m.as_str().parse::<i32>().unwrap() ).unwrap(),
             c.get(2).map( |m| m.as_str().parse::<i32>().unwrap() ).unwrap())
        })
        .map( |(a, b)| a * b )
        .sum()
}

pub fn day3_part2(text: &str) -> i32
{
    // Combine lines
    let text: String = text.lines().fold( String::new(), |acc, e| acc + e );

    // Remove dont't()...do() sections
    let dont_do_re = Regex::new(r"(don't\(\).+?do\(\))").unwrap();
    let text = dont_do_re.replace_all(&text, "").to_string();

    // Remove trailing don't().. section if any
    let trailing_dont_re = Regex::new(r"(don't\(\).+$)").unwrap();
    let text = trailing_dont_re.replace_all(&text, "").to_string();

    let re: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(&text)
        .map( |c|
        {
            (c.get(1).map( |m| m.as_str().parse::<i32>().unwrap() ).unwrap(),
             c.get(2).map( |m| m.as_str().parse::<i32>().unwrap() ).unwrap())
        })
        .map( |(a, b)| a * b )
        .sum()
}