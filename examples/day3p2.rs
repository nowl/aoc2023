use std::{collections::HashSet, path::Path};

use anyhow::Error;
use aoc2023::read_as_lines;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_NUM: Regex = Regex::new(r"(?<num>\d+)").unwrap();
    static ref REGEX_GEAR: Regex = Regex::new(r"(?<symbol>\*)").unwrap();
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Location {
    row: usize,
    col: usize,
}

fn gear_adj(gear_loc: &Location, nums: &Vec<(i32, usize, usize, usize)>) -> Option<(i32, i32)> {
    let &Location {
        row: grow,
        col: gcol,
    } = gear_loc;

    let mut adj_nums = vec![];

    for &(num, row, col_start, col_end) in nums.iter() {
        // same row
        if col_start > 0 {
            if col_start - 1 == gcol && grow == row {
                adj_nums.push(num);
                continue;
            }
        }

        if col_end + 1 == gcol && grow == row {
            adj_nums.push(num);
            continue;
        }

        // above
        let col_start = if col_start > 0 {
            col_start - 1
        } else {
            col_start
        };
        if row > 0 {
            if grow == row - 1 && gcol >= col_start && gcol <= col_end + 1 {
                adj_nums.push(num);
                continue;
            }
        }

        // below
        if grow == row + 1 && gcol >= col_start && gcol <= col_end + 1 {
            adj_nums.push(num);
            continue;
        }
    }

    if adj_nums.len() == 2 {
        Some((adj_nums[0], adj_nums[1]))
    } else {
        None
    }
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d3p1.txt");
    let lines = read_as_lines(file)?;

    let mut gear_set = HashSet::new();
    let mut numbers = vec![];

    for (row, line) in lines.iter().enumerate() {
        for cap in REGEX_GEAR.captures_iter(&line) {
            let m = cap.name("symbol").unwrap();
            assert!(m.end() - m.start() == 1);
            let col = m.start();
            let loc = Location { row, col };
            gear_set.insert(loc);
        }

        for cap in REGEX_NUM.captures_iter(&line) {
            let m = cap.name("num").unwrap();
            let num = m.as_str().parse::<i32>()?;
            numbers.push((num, row, m.start(), m.end() - 1));
        }
    }

    let sum = gear_set
        .iter()
        .fold(0, |acc, loc| match gear_adj(loc, &numbers) {
            Some((a, b)) => acc + a * b,
            None => acc,
        });

    println!("{sum}");

    Ok(())
}
