#![warn(clippy::pedantic)]

use std::env;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io::Read;

fn get_arg(pos: usize) -> Result<String, Error> {
    match env::args().nth(pos) {
        Some(arg) => Ok(arg.trim().to_string()),
        None => Err(Error::new(ErrorKind::NotFound, "Argument not found")),
    }
}


fn find_code(input: & str) -> u32 {
    let mut code = String::with_capacity(2);

    for c in input.chars() {
        if c.is_digit(10) {
            code.push(c);
            break;
        }
    }

    for c in input.chars().rev() {
        if c.is_digit(10) {
            code.push(c);
            break;
        }
    }

    code.parse().unwrap()
}

fn main() -> Result<(), Error> {
    let infile = get_arg(1)?;
    let mut infile = File::open(infile)?;

    let mut data = String::new();
    infile.read_to_string(&mut data)?;

    let mut sum:u32 = 0;

    for line in data.lines() {
        let code = find_code(line);
        println!("{}: {}", line, code);
        sum += code;
    }
    println!("Sum: {}", sum);
    Ok(())
}
