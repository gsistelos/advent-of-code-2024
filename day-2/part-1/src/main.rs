use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let lines = read_lines("../input.txt");

    let reports: Vec<Vec<u32>> = parse_lines(lines);

    let mut safe = 0;

    for report in reports {
        let increasing = is_increasing(&report);
        let decreasing = !increasing;

        let is_safe = !report.windows(2).any(|levels| {
            levels[0] == levels[1]
                || levels[0].abs_diff(levels[1]) > 3
                || (increasing && levels[0] > levels[1])
                || (decreasing && levels[0] < levels[1])
        });

        if is_safe {
            safe += 1;
        }
    }

    println!("{safe}");
}

fn is_increasing<T>(report: &Vec<T>) -> bool
where
    T: PartialOrd,
{
    if let Some(levels) = report.windows(2).next() {
        if levels[0] > levels[1] {
            return false;
        }
    } else {
        panic!("Input error");
    }

    true
}

fn parse_line<T>(line: &str, report: &mut Vec<T>)
where
    T: FromStr,
    <T>::Err: Display,
{
    let items = line.split_whitespace();

    for item in items {
        let value = match item.parse() {
            Ok(v) => v,
            Err(e) => panic!("{e}: Input error"),
        };

        report.push(value);
    }
}

fn parse_lines<T, U>(lines: T) -> Vec<Vec<U>>
where
    T: Iterator<Item = Result<String, Error>>,
    U: FromStr,
    U::Err: Display,
{
    let mut reports: Vec<Vec<U>> = Vec::new();

    for line in lines {
        let mut report = Vec::new();

        match line {
            Ok(s) => parse_line(s.as_ref(), &mut report),
            Err(e) => panic!("{e}"),
        };

        reports.push(report);
    }

    reports
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
