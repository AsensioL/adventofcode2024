pub fn day2_part1(text: &str) -> usize
{
    // Get reports
    let reports: Vec<Vec<i32>> = text.lines()
        .map( |line|
            line.split(' ')
                .map( |val| val.parse::<i32>().expect("Failed to parse") )
                .collect::<Vec<i32>>()
        )
        .collect();

    // Get a list of diffs across rows
    let diffs = reports.into_iter()
        .map( |report|
        {
            report.windows(2)
                .map( |window| window[1] - window[0] )
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();

    diffs.into_iter()
        .map( |diff|
        {
            let all_positive = diff.iter().all( |d| *d > 0);
            let all_negative = diff.iter().all( |d| *d < 0);
            let small_change = diff.iter().all( |d| d.abs() <= 3);
            small_change && (all_positive || all_negative)
        })
        //.collect::<Vec<bool>>();
        .filter(|b| *b)
        .count()
}


/******* Skip function attempts *******/
use std::slice::Iter;

pub fn skip_nth0<T>(v1: &[T], idx: usize) -> Vec<&T> {
    v1.iter().take(idx).chain(v1.iter().skip(idx + 1)).collect::<Vec<&T>>()
}

pub fn skip_nth1<T>(v1: &[T], idx: usize) -> impl Iterator<Item = &T> {
    v1.iter().take(idx).chain(v1.iter().skip(idx + 1))
}

pub fn skip_nth2<T>(it: Iter<'_, T>, idx: usize) -> Vec<&T> {
    it.clone().take(idx).chain(it.skip(idx + 1)).collect::<Vec<&T>>()
}

pub fn skip_nth3<'a, T, I>(it: I , idx: usize) -> Vec<&'a T>
where I: Iterator<Item = &'a T> + Clone {
    it.clone().take(idx).chain(it.skip(idx + 1)).collect::<Vec<&T>>()
}

pub fn skip_nth4<'a, T, I>(it: I , idx: usize) -> impl Iterator<Item = &'a T>
where T: 'a,
      I: Iterator<Item = &'a T> + Clone {
    it.clone().take(idx).chain(it.skip(idx + 1))
}

pub struct Skipper<I>
where
    I: Iterator,
{
    idx: usize,
    skip: usize,
    it: I,
}

impl<I> Skipper<I>
where
    I: Iterator,
{
    pub fn new(it: I, skip: usize) -> Self {
        Self { idx: 0, skip, it }
    }
}

impl<I> Iterator for Skipper<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.skip {
            _ = self.it.next();
        }
        self.idx += 1;
        self.it.next()
    }
}
/******* Skip function attempts *******/

fn report_is_valid(report: &[i32]) -> bool {
    let diff = report.windows(2)
        .map( |window| window[1] - window[0] )
        .collect::<Vec<i32>>();

    let all_positive = diff.iter().all( |d| *d > 0);
    let all_negative = diff.iter().all( |d| *d < 0);
    let small_change = diff.iter().all( |d| d.abs() <= 3);
    small_change && (all_positive || all_negative)
}

pub fn day2_part2(text: &str) -> usize
{
    // Get reports
    let reports: Vec<Vec<i32>> = text.lines()
        .map( |line|
            line.split(' ')
                .map( |val| val.parse::<i32>().expect("Failed to parse") )
                .collect::<Vec<i32>>()
        )
        .collect();

    // Get a list of diffs across rows
    reports.into_iter()
        .map( |report|
        {
            // Evaluate unmodified report
            if report_is_valid(&report) { return true; }

            // Evaluate modified report
            for idx in 0..report.len() {
                // Skip idx-th item report
                let new_report = skip_nth0(&report, idx).into_iter().copied().collect::<Vec<i32>>();
                if report_is_valid(&new_report) { return true; }
            }

            // If none of the above is valid, the report is invalid
            false
        })
        //.collect::<Vec<bool>>();
        .filter(|b| *b)
        .count()
}
