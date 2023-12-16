use debug_print::debug_println;
use itertools::Itertools;
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
enum Op {
    Equal(String, u8),
    Remove(String),
}

#[derive(Debug)]
struct Data {
    terms: Vec<Op>,
}

fn parse_op(i: &str) -> IResult<&str, Op> {
    let (i, name) = is_not("-=")(i)?;
    let (i, typ) = one_of("-=")(i)?;
    match typ {
        '-' => Ok((i, Op::Remove(name.to_string()))),
        '=' => {
            let (i, n) = digit1(i)?;
            Ok((i, Op::Equal(name.to_string(), n.parse().unwrap())))
        }
        _ => unreachable!(),
    }
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let (i, terms) = terminated(separated_list1(tag(","), parse_op), multispace0)(i)?;
    let data = Data { terms };
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

    let mut boxes = std::iter::repeat(Vec::<(String, u8)>::new())
        .take(256)
        .collect_vec();

    for term in data.terms {
        match term {
            Op::Remove(n) => {
                let box_num = hash_alg(&n) as usize;
                if let Some(pos) = boxes[box_num].iter().position(|(name, ..)| *name == n) {
                    boxes[box_num].remove(pos);
                }
            }
            Op::Equal(n, v) => {
                let box_num = hash_alg(&n) as usize;
                if let Some(pos) = boxes[box_num].iter().position(|(name, ..)| *name == n) {
                    boxes[box_num][pos].1 = v;
                } else {
                    boxes[box_num].push((n.clone(), v));
                }
            }
        };

        debug_println!("boxes = {boxes:?}");
    }

    let score = boxes
        .iter()
        .enumerate()
        .map(|(n, entries)| {
            let box_power = entries
                .iter()
                .enumerate()
                .fold(0, |acc, (n2, &(ref _name, val))| {
                    acc + (n2 as u32 + 1) * val as u32
                });
            (n as u32 + 1) * box_power
        })
        .sum::<u32>();

    println!("{score}");

    Ok(())
}
