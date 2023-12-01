use std::path::Path;

use anyhow::Error;
use aoc2023::{read_as_lines, Args, Parser};

fn main() -> Result<(), Error> {
    let file = Path::new("data/d1p1.txt");
    let lines = read_as_lines(file)?;

    /*
        for line in lines {
            println!("{line}: {}", getcode(&line));
    }
         */

    let sum = lines
        .iter()
        .fold(0, |acc, line| acc + getcode(&line) as i32);

    println!("{sum}");

    Ok(())
}

fn getcode(s: &str) -> u8 {
    let fval = s.chars().find_map(|c| {
        let ascii = c as u8;
        if ascii >= 0x30 && ascii <= 0x39 {
            Some(ascii - 0x30)
        } else {
            None
        }
    });

    let lval = s.chars().rev().find_map(|c| {
        let ascii = c as u8;
        if ascii >= 0x30 && ascii <= 0x39 {
            Some(ascii - 0x30)
        } else {
            None
        }
    });

    fval.unwrap() * 10u8 + lval.unwrap()
}
