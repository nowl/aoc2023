use std::{fs, path::Path};

use anyhow::Error;
use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::*, combinator::*, multi::many1, sequence::*, IResult,
};

#[derive(Debug)]
struct Data {
    lines: Vec<Vec<i32>>,
}

fn parse_i32(i: &str) -> IResult<&str, i32> {
    let (i, n) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s: &str| {
        s.parse::<i32>()
    })(i)?;

    Ok((i, n))
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_line = terminated(many1(terminated(parse_i32, space0)), multispace0);
    let (i, lines) = many1(parse_line)(i)?;
    let data = Data { lines };
    Ok((i, data))
}

fn predict(nums: &Vec<i32>) -> i32 {
    //println!("{nums:?}");
    let mut history = vec![];
    history.push(nums.last().cloned().unwrap());
    let mut diff = nums.windows(2).map(|vals| vals[1] - vals[0]).collect_vec();
    //println!("diff: {diff:?}");
    loop {
        history.push(diff.last().cloned().unwrap());
        diff = diff.windows(2).map(|vals| vals[1] - vals[0]).collect_vec();

        //println!("diff: {diff:?}");

        if diff.iter().all(|v| *v == 0) {
            break;
        }
    }
    //println!("history: {history:?}");

    history.iter().sum()
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d9p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    let prediction = data
        .lines
        .iter()
        .map(|line| predict(line) as i64)
        .sum::<i64>();

    println!("{prediction:?}");

    Ok(())
}
