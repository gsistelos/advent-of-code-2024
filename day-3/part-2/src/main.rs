use regex::Regex;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = std::fs::read_to_string("../input.txt")?;

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)")?;

    let mut doing = true;

    let mut result: u32 = 0;

    for found in re.find_iter(&file_content) {
        let s = found.as_str();

        if s == "do()" {
            doing = true;
            continue;
        }

        if s == "don't()" {
            doing = false;
            continue;
        }

        if !doing {
            continue;
        }

        let mut parts = s.split(',');

        let first_number = parse_number(parts.next(), |value| &value[4..])?;
        let second_number = parse_number(parts.next(), |value| &value[..value.len() - 1])?;

        result += first_number * second_number;
    }

    println!("{result}");

    Ok(())
}

fn parse_number<F>(part: Option<&str>, f: F) -> Result<u32, Box<dyn Error>>
where
    F: Fn(&str) -> &str,
{
    let slice = match part {
        Some(value) => f(value),
        None => return Err("Missing value".into()),
    };

    Ok(slice.parse()?)
}
