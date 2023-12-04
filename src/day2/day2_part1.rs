#![warn(clippy::pedantic)]

use once_cell::sync::Lazy;
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

fn parse_pick(pick: &str) -> HashMap<&str, u32> {
    let mut result: HashMap<&str, u32> = HashMap::from([
        ("red", 0),
        ("green", 0),
        ("blue", 0)
    ]);

    for record in pick.trim().split(',') {
        let fields: Vec<&str> = record.trim().split_whitespace().collect();
        result.insert(fields[1], fields[0].parse().unwrap());
    }

    println!("{:?}", result);
    result
}

fn parse_record(record: &str) -> (bool, u32) {
    let record = record.trim();
    let game_split: Vec<&str> = record.splitn(2, ':').collect();
    let num_split: Vec<&str> = game_split[0].trim().splitn(2, ' ').collect();
    let game_num:u32 =  num_split[1].parse().unwrap();

    let mut valid = true;

    for segment in game_split[1].split(';') {
        let pick = parse_pick(segment.trim());
        if (pick["red"] > 12) || (pick["green"] > 13) || (pick["blue"] > 14) {
            valid = false;
            break;
        }
    }

    return (valid, game_num);
}

fn main() -> Result<(), Error> {
    let infile = get_arg(1)?;
    let mut infile = File::open(infile)?;

    let mut data = String::new();
    infile.read_to_string(&mut data)?;
    let mut score:u32 = 0;

    for line in data.trim().lines() {
        println!("{line}");
        let (valid, game_num) = parse_record(line);
        if valid {
            println!("Valid.\n");
            score += game_num;
        } else {
            println!("Invalid.\n");
        }
    }

    println!("score: {score}");
    Ok(())
}
