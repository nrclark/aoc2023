#![warn(clippy::pedantic)]

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::{Error, ErrorKind};

fn get_arg(pos: usize) -> Result<String, Error> {
    match env::args().nth(pos) {
        Some(arg) => Ok(arg.trim().to_string()),
        None => Err(Error::new(ErrorKind::NotFound, "Argument not found")),
    }
}

fn find_digits(word: &str) -> Vec<char> {
    let digit_lookup = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
    ]);

    let mut result = Vec::new();

    for (i, _) in word.char_indices() {
        let substring = &word[i..];

        for (key, value) in &digit_lookup {
            if substring.starts_with(key) {
                result.push(*value);
                break;
            }
        }
    }
    result
}

fn find_code(input: &str) -> u32 {
    let mut code = String::with_capacity(2);
    let digits = find_digits(input);

    code.push(*digits.first().unwrap());
    code.push(*digits.last().unwrap());
    code.parse().unwrap()
}

fn main() -> Result<(), Error> {
    let infile = get_arg(1)?;
    let mut infile = File::open(infile)?;

    let mut data = String::new();
    infile.read_to_string(&mut data)?;

    let mut sum: u32 = 0;

    for word in data.lines() {
        let code = find_code(word);
        println!("{word}: {code}");
        sum += code;
    }

    println!("Sum: {sum}");
    Ok(())
}
