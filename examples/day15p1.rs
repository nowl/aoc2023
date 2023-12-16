use debug_print::debug_println;
use std::{fs, path::Path};

use anyhow::Error;
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::*,
    multi::*,
    sequence::*,
    IResult,
};

#[derive(Debug)]
struct Data {
    terms: Vec<String>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let (i, terms) = terminated(separated_list1(tag(","), is_not(",\n")), multispace0)(i)?;
    let data = Data {
        terms: terms.iter().map(|x| x.to_string()).collect(),
    };
    Ok((i, data))
}

fn hash_alg(s: &str) -> u8 {
    s.chars().fold(0, |acc, c| {
        let ac = c as u8;
        (((acc as i32 + ac as i32) * 17) % 256) as u8
    })
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d15p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    debug_println!("data: {data:?}");

    let answer = data.terms.iter().map(|t| hash_alg(t) as i32).sum::<i32>();

    println!("{answer}");

    Ok(())
}
