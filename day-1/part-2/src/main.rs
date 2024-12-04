use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};
use std::path::Path;
use std::str::{FromStr, SplitWhitespace};

fn main() {
    let lines = read_lines("../input.txt");

    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    parse_lines(lines, |item| {
        parse_item(item, &mut left_list);
        parse_item(item, &mut right_list);
    });

    let result: u32 = left_list
        .iter()
        .map(|left_item| {
            let mut matches: u32 = 0;

            for right_item in right_list.iter() {
                if left_item == right_item {
                    matches += 1;
                }
            }

            left_item * matches
        })
        .sum();

    println!("{result}");
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
