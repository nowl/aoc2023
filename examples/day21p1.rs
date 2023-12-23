use debug_print::{debug_print, debug_println};
use itertools::{repeat_n, Itertools};
use nom::combinator::eof;
use std::io;
use std::{fs, path::Path};

use anyhow::Error;
use nom::character::complete::*;
use nom::{multi::*, sequence::*, IResult};

#[derive(Debug)]
struct Data {
    map: Vec<Vec<Tile>>,
    start: (usize, usize),
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Soil,
    Rock,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let line_parse = terminated(many1(one_of(".#S")), multispace1);
    let mut parser = terminated(many1(line_parse), eof);
    let (i, data) = parser(i)?;

    let mut start = (0, 0);
    let map = data
        .iter()
        .enumerate()
        .map(|(rown, row)| {
            row.iter()
                .enumerate()
                .map(|(coln, t)| match t {
                    '.' => Tile::Soil,
                    '#' => Tile::Rock,
                    'S' => {
                        start = (rown, coln);
                        Tile::Soil
                    }
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let data = Data { map, start };
    Ok((i, data))
}

fn advance(map: &Vec<Vec<Tile>>, locs: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let nrows = map.len();
    let ncols = map[0].len();

    let mut nlocs = locs
        .iter()
        .map(|row| repeat_n(false, row.len()).collect_vec())
        .collect_vec();

    for row in 0..nrows {
        for col in 0..ncols {
            if locs[row][col] {
                // check adj
                let r = row as i32;
                let c = col as i32;

                macro_rules! run_adj {
                    ($dr:expr, $dc:expr) => {{
                        let r = r + $dr;
                        let c = c + $dc;
                        if r >= 0 && r < nrows as i32 && c >= 0 && c < ncols as i32 {
                            let r = r as usize;
                            let c = c as usize;
                            if map[r][c] == Tile::Soil {
                                nlocs[r][c] = true;
                            }
                        }
                    }};
                }

                run_adj!(1, 0);
                run_adj!(-1, 0);
                run_adj!(0, 1);
                run_adj!(0, -1);
            }
        }
    }

    nlocs
}

fn display(map: &Vec<Vec<Tile>>, locs: &Vec<Vec<bool>>) {
    let nrows = map.len();
    let ncols = map[0].len();

    for row in 0..nrows {
        for col in 0..ncols {
            if locs[row][col] {
                debug_print!("O");
            } else {
                match map[row][col] {
                    Tile::Soil => {
                        debug_print!(".");
                    }
                    Tile::Rock => {
                        debug_print!("#");
                    }
                }
            }
        }
        debug_println!();
    }
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d21p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    debug_println!("{data:?}");

    let mut locs = data
        .map
        .iter()
        .map(|row| repeat_n(false, row.len()).collect_vec())
        .collect_vec();

    locs[data.start.0][data.start.1] = true;

    for _ in 0..64 {
        locs = advance(&data.map, &locs);

        //display(&data.map, &locs);
    }

    let count = locs
        .iter()
        .map(|row| row.iter().filter(|&v| *v == true).count() as i32)
        .sum::<i32>();

    println!("{count}");

    Ok(())
}
