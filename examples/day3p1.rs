use std::{collections::HashSet, path::Path};

use anyhow::Error;
use aoc2023::read_as_lines;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_NUM: Regex = Regex::new(r"(?<num>\d+)").unwrap();
    static ref REGEX_SYM: Regex = Regex::new(r"(?<symbol>[^\d.])").unwrap();
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Location {
    row: usize,
    col: usize,
}

fn num_adj(row: usize, col_start: usize, col_end: usize, syms: &HashSet<Location>) -> bool {
    // same row
    if col_start > 0 {
        if syms.contains(&Location {
            row,
            col: col_start - 1,
        }) {
            return true;
        }
    }

    if syms.contains(&Location {
        row,
        col: col_end + 1,
    }) {
        return true;
    }

    // above
    let col_start = if col_start > 0 {
        col_start - 1
    } else {
        col_start
    };
    if row > 0 {
        for col in col_start..=col_end + 1 {
            if syms.contains(&Location { row: row - 1, col }) {
                return true;
            }
        }
    }

    // below
    for col in col_start..=col_end + 1 {
        if syms.contains(&Location { row: row + 1, col }) {
            return true;
        }
    }

    false
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d3p1.txt");
    let lines = read_as_lines(file)?;

    let mut symbol_set = HashSet::new();
    let mut numbers = vec![];

    for (row, line) in lines.iter().enumerate() {
        for cap in REGEX_SYM.captures_iter(&line) {
            let m = cap.name("symbol").unwrap();
            assert!(m.end() - m.start() == 1);
            let col = m.start();
            let loc = Location { row, col };
            symbol_set.insert(loc);
        }

        for cap in REGEX_NUM.captures_iter(&line) {
            let m = cap.name("num").unwrap();
            let num = m.as_str().parse::<i32>()?;
            numbers.push((num, row, m.start(), m.end() - 1));
        }
    }

    let sum = numbers.iter().fold(0, |acc, &(num, row, cstart, cend)| {
        if num_adj(row, cstart, cend, &symbol_set) {
            acc + num
        } else {
            acc
        }
    });

    println!("{sum}");

    Ok(())
}
