use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};
use std::path::Path;
use std::str::{FromStr, SplitWhitespace};

fn main() {
    let lines = read_lines("../input.txt");

    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    parse_lines(lines, |items| {
        left_list.push(parse_item(&mut items.next()));
        right_list.push(parse_item(&mut items.next()));
    });

    left_list.sort();
    right_list.sort();

    if left_list.len() != right_list.len() {
        panic!("Input error");
    }

    let difference: u32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(&left, &right)| left.abs_diff(right))
        .sum();

    println!("{difference}");
}

fn parse_item<T>(opt: &mut Option<&str>) -> T
where
    T: FromStr,
    <T>::Err: Display,
{
    let item = match opt {
        Some(v) => v,
        None => panic!("Input error"),
    };

    match item.parse() {
        Ok(v) => v,
        Err(e) => panic!("{e}: Input error"),
    }
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
