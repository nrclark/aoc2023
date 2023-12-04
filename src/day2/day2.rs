#![warn(clippy::pedantic)]

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

#[derive(Default)]
struct MarblePick {
    red: u32,
    blue: u32,
    green: u32,
}

fn parse_pick(pick: &str) -> MarblePick {
    let mut result = MarblePick::default();

    for record in pick.trim().split(',') {
        let fields: Vec<&str> = record.split_whitespace().collect();
        let count = fields[0].parse().unwrap();
        let color = fields[1];
        match color {
            "red" => {
                result.red = count;
            }
            "blue" => {
                result.blue = count;
            }
            "green" => {
                result.green = count;
            }
            &_ => todo!(),
        }
    }

    result
}

fn parse_record(record: &str) -> u32 {
    let record = record.trim();
    let game_split: Vec<&str> = record.splitn(2, ':').collect();
    let mut required = MarblePick::default();

    for segment in game_split[1].split(';') {
        let pick = parse_pick(segment.trim());

        required.red = std::cmp::max(pick.red, required.red);
        required.green = std::cmp::max(pick.green, required.green);
        required.blue = std::cmp::max(pick.blue, required.blue);
    }

    required.red * required.green * required.blue
}

fn main() -> Result<(), Error> {
    let infile = get_arg(1)?;
    let mut infile = File::open(infile)?;

    let mut data = String::new();
    infile.read_to_string(&mut data)?;
    let mut score: u32 = 0;

    for line in data.trim().lines() {
        println!("{line}");
        score += parse_record(line);
    }

    println!("score: {score}");
    Ok(())
}
