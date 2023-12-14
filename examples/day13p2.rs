use debug_print::debug_println;
use itertools::Itertools;
use std::{fs, path::Path};

use anyhow::Error;
use nom::{
    character::{complete::*, streaming::newline},
    multi::*,
    sequence::*,
    IResult,
};

#[derive(Debug)]
struct Data {
    problems: Vec<Vec<Vec<char>>>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_line = terminated(many1(one_of(".#")), newline);
    let parse_problem = terminated(many1(parse_line), multispace0);
    let (i, problems) = many1(parse_problem)(i)?;
    let data = Data { problems };
    Ok((i, data))
}

fn make_horizontal(grid: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|&c| match c {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn make_vertical(grid: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let mut v = vec![];

    for col in 0..grid[0].len() {
        let v2 = grid
            .iter()
            .map(|row| match row[col] {
                '#' => true,
                '.' => false,
                _ => unreachable!(),
            })
            .collect();

        v.push(v2);
    }

    v
}

fn check_equality_from(col: usize, v: &Vec<Vec<bool>>) -> bool {
    let to_check = (v.len() - col - 2).min(col);

    let mut unmatched = None;

    for i in 0..=to_check {
        let a = col - i;
        let b = col + i + 1;

        if v[a] != v[b] {
            if unmatched.is_some() {
                return false;
            }
            unmatched = Some((&v[a], &v[b]));
        }
    }

    if unmatched.is_none() {
        return false;
    }

    let differences = unmatched
        .unwrap()
        .0
        .iter()
        .zip_eq(unmatched.unwrap().1.iter())
        .fold(0, |acc, (a, b)| if a == b { acc } else { acc + 1 });
    differences == 1
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d13p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    debug_println!("data: {data:?}");

    let mut cols_left_of = 0;
    let mut rows_above = 0;

    for problem in data.problems {
        let horizontals = make_horizontal(&problem);
        debug_println!("horizontals: {horizontals:?}");

        let verticals = make_vertical(&problem);
        debug_println!("verticals: {verticals:?}");

        for col in 0..verticals.len() - 1 {
            if check_equality_from(col, &verticals) {
                debug_println! {"match on {col}"}
                cols_left_of += col + 1;
            }
        }
        for row in 0..horizontals.len() - 1 {
            if check_equality_from(row, &horizontals) {
                debug_println! {"match on {row}"}
                rows_above += row + 1;
            }
        }
    }

    let score = cols_left_of + 100 * rows_above;
    println!("{score}");

    Ok(())
}
