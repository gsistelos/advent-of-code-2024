use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let lines = read_lines("../input.txt");

    let (mut left_list, mut right_list): (Vec<u32>, Vec<u32>) = parse_lines(lines);

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

fn parse_item<'a, T, U>(it: &mut T, list: &mut Vec<U>)
where
    T: Iterator<Item = &'a str>,
    U: FromStr,
    <U>::Err: Display,
{
    if let Some(item) = it.next() {
        let value = match item.parse() {
            Ok(v) => v,
            Err(e) => panic!("{e}: Input error"),
        };

        list.push(value);
    } else {
        panic!("Input error");
    }
}

fn parse_line<T>(line: &str, left_list: &mut Vec<T>, right_list: &mut Vec<T>)
where
    T: FromStr,
    <T>::Err: Display,
{
    let mut pair = line.split_whitespace();

    parse_item(&mut pair, left_list);
    parse_item(&mut pair, right_list);
}

fn parse_lines<T, U>(lines: T) -> (Vec<U>, Vec<U>)
where
    T: Iterator<Item = Result<String, Error>>,
    U: FromStr,
    <U>::Err: Display,
{
    let mut left_list: Vec<U> = Vec::new();
    let mut right_list: Vec<U> = Vec::new();

    for line in lines {
        match line {
            Ok(s) => parse_line(s.as_ref(), &mut left_list, &mut right_list),
            Err(e) => panic!("{e}"),
        };
    }

    (left_list, right_list)
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
