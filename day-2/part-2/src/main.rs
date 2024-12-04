use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};
use std::path::Path;
use std::str::{FromStr, SplitWhitespace};

fn main() {
    let lines = read_lines("../input.txt");

    let mut reports: Vec<Vec<u32>> = Vec::new();

    parse_lines(lines, |items| {
        let report: Vec<u32> = parse_items(items);

        reports.push(report);
    });

    let mut safe = 0;

    for report in reports {
        if is_safe(&report) {
            safe += 1;
            continue;
        }

        for (except_idx, _) in report.iter().enumerate() {
            let new_report: Vec<u32> = new_vec_except(&report, except_idx);

            if is_safe(&new_report) {
                safe += 1;
                break;
            }
        }
    }

    println!("{safe}");
}

fn new_vec_except<T>(report: &Vec<T>, except_idx: usize) -> Vec<T>
where
    T: Copy,
{
    let mut new_report: Vec<T> = Vec::new();

    for (idx, level) in report.iter().enumerate() {
        if idx == except_idx {
            continue;
        }

        new_report.push(*level);
    }

    new_report
}

fn is_safe(report: &Vec<u32>) -> bool {
    let increasing = is_increasing(&report);
    let decreasing = !increasing;

    !report.windows(2).any(|levels| {
        levels[0] == levels[1]
            || levels[0].abs_diff(levels[1]) > 3
            || (increasing && levels[0] > levels[1])
            || (decreasing && levels[0] < levels[1])
    })
}

fn is_increasing<T>(report: &Vec<T>) -> bool
where
    T: PartialOrd,
{
    let levels = match report.windows(2).next() {
        Some(levels) => levels,
        None => panic!("Input error"),
    };

    levels[0] < levels[1]
}

fn parse_items<'a, T, U>(items: T) -> Vec<U>
where
    T: Iterator<Item = &'a str>,
    U: FromStr,
    <U>::Err: Display,
{
    let mut parsed: Vec<U> = Vec::new();

    for item in items {
        let value = match item.parse() {
            Ok(v) => v,
            Err(e) => panic!("{e}: Input error"),
        };

        parsed.push(value);
    }

    parsed
}

fn parse_lines<T, F>(lines: T, mut f: F)
where
    T: Iterator<Item = Result<String, Error>>,
    F: FnMut(&mut SplitWhitespace),
{
    for line in lines {
        match line {
            Ok(s) => f(&mut s.split_whitespace()),
            Err(e) => panic!("{e}"),
        };
    }
}

fn read_lines<P>(filename: P) -> Lines<BufReader<File>>
where
    P: AsRef<Path> + Display,
{
    let file = match File::open(&filename) {
        Ok(file) => file,
        Err(e) => panic!("{filename}: {e}"),
    };

    BufReader::new(file).lines()
}
