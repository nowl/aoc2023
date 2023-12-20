use debug_print::debug_println;
use itertools::Itertools;
use std::{fs, path::Path};

use anyhow::Error;
use nom::character::complete::*;
use nom::{bytes::complete::tag, combinator::map_res, multi::*, sequence::*, IResult};

#[derive(Debug)]
struct Data {
    state: Vec<Plan>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Plan {
    dir: Dir,
    length: i32,
}

fn parse_plan(i: &str) -> IResult<&str, Plan> {
    let (i, _) = terminated(one_of("UDLR"), space1)(i)?;
    let (i, _) = map_res(terminated(digit1, space1), |c: &str| c.parse::<i32>())(i)?;
    let (i, color) = terminated(
        delimited(char('('), preceded(tag("#"), alphanumeric1), char(')')),
        multispace0,
    )(i)?;

    let dir = match color.chars().nth(5).unwrap() {
        '3' => Dir::Up,
        '1' => Dir::Down,
        '2' => Dir::Left,
        '0' => Dir::Right,
        _ => unreachable!(),
    };

    let length = i32::from_str_radix(color.get(0..5).unwrap(), 16).unwrap();

    let plan = Plan { dir, length };

    Ok((i, plan))
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let (i, state) = terminated(many1(parse_plan), multispace0)(i)?;
    let data = Data { state };
    Ok((i, data))
}

fn trace_area(inputs: &Vec<Plan>) -> i64 {
    use Dir::*;
    let mut area = 0;
    let mut y_pos = 0;

    for ((n1, p1), (_, p2), (_, p3)) in inputs.iter().enumerate().tuple_windows::<(_, _, _)>() {
        if n1 == 0 {
            if p1.dir == Left {
                y_pos = -1;
            }
        }
        if n1 % 2 == 0 {
            continue;
        }

        match (p1.dir, p2.dir, p3.dir) {
            (Up, Left, Up) => {
                y_pos -= p1.length;
                let height = y_pos + 1;
                let len = p2.length;
                area += len as i64 * height as i64;
                debug_println!("y_pos: {y_pos} height: {height} len: {len} area: {area}");
            }
            (Up, Left, Down) => {
                // 2 (correct)
                y_pos -= p1.length;
                let height = y_pos + 1;
                let len = p2.length - 1;
                area += len as i64 * height as i64;
                debug_println!("y_pos: {y_pos} height: {height} len: {len} area: {area}");
            }
            (Up, Right, Up) => {
                y_pos -= p1.length;
                let height = y_pos;
                let len = p2.length;
                area -= len as i64 * height as i64;
                debug_println!("y_pos: {y_pos} height: {height} len: {len} area: {area}");
            }
            (Up, Right, Down) => {
                // 4
                y_pos -= p1.length;
                let height = y_pos;
                let len = p2.length + 1;
                area -= len as i64 * height as i64;
                debug_println!("y_pos: {y_pos} height: {height} len: {len} area: {area}");
            }
            (Down, Left, Up) => {
                y_pos += p1.length;
                let height = y_pos + 1;
                let len = p2.length + 1;
                area += len as i64 * height as i64;
                debug_println!("y_pos: {y_pos} height: {height} len: {len} area: {area}");
            }
            (Down, Left, Down) => {
                y_pos += p1.length;
                let height = y_pos + 1;
                let len = p2.length;
                area += len as i64 * height as i64;
                debug_println!("y_pos: {y_pos} height: {height} len: {len} area: {area}");
            }
            (Down, Right, Up) => {
                // 7
                y_pos += p1.length;
                let height = y_pos;
                let len = p2.length - 1;
                area -= len as i64 * height as i64;
                debug_println!("y_pos: {y_pos} height: {height} len: {len} area: {area}");
            }
            (Down, Right, Down) => {
                y_pos += p1.length;
                let height = y_pos;
                let len = p2.length;
                area -= len as i64 * height as i64;
                debug_println!("y_pos: {y_pos} height: {height} len: {len} area: {area}");
            }
            _ => unreachable!(),
        }
    }

    area
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d18p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    debug_println!("data: {data:?}");

    let count = trace_area(&data.state);

    println!("{count}");

    Ok(())
}
