use aoc2023::pause_enter;
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

    debug_println!("rows, cols: {nrows}, {ncols}");

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

fn simulate(map: &Vec<Vec<Tile>>, row: usize, col: usize, num: i32) -> u64 {
    let mut locs = map
        .iter()
        .map(|row| repeat_n(false, row.len()).collect_vec())
        .collect_vec();

    locs[row][col] = true;

    for _ in 0..num {
        locs = advance(&map, &locs);

        //display(&map, &locs);

        //pause_enter();
    }

    display(&map, &locs);

    let count = locs
        .iter()
        .map(|row| row.iter().filter(|&v| *v == true).count() as i32)
        .sum::<i32>();

    debug_println!("{count}");

    count as u64
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d21p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    // input data is 131 x 131
    // 65 steps to get to edge, 131 steps to move across map
    // 26501365 total (26501365 - 65) / 131 = 202300 crossings

    // simulations

    // full map
    let full_tile_even = simulate(&data.map, data.start.0, data.start.1, 234);
    let full_tile_odd = simulate(&data.map, data.start.0, data.start.1, 233);

    // corner maps
    let mut corners = 0;
    corners += simulate(&data.map, 130, 65, 130);
    corners += simulate(&data.map, 0, 65, 130);
    corners += simulate(&data.map, 65, 0, 130);
    corners += simulate(&data.map, 65, 130, 130);

    // diagonal
    let mut small_diags = 0;
    small_diags += simulate(&data.map, 0, 0, 64);
    small_diags += simulate(&data.map, 130, 0, 64);
    small_diags += simulate(&data.map, 0, 130, 64);
    small_diags += simulate(&data.map, 130, 130, 64);

    let mut big_diags = 0;
    big_diags += simulate(&data.map, 0, 0, 65 + 130);
    big_diags += simulate(&data.map, 130, 0, 65 + 130);
    big_diags += simulate(&data.map, 0, 130, 65 + 130);
    big_diags += simulate(&data.map, 130, 130, 65 + 130);

    // example with 4
    //    DCD
    //   DE*ED      O
    //  DE***ED    OEO
    // DE*****ED  OEOEO
    // C***S***C OEOEOEO
    // DE*****ED  OEOEO
    //  DE***ED    OEO
    //   DE*ED      O  O=16,E=9
    //    DCD

    let target: u64 = 202300 * 2 - 1;
    //let target: u64 = 4 * 2 - 1;
    let mut total_filled = target;
    let mut diag_count = 0;
    let mut odds = target / 2 + 1;
    let mut evens = target / 2;
    for c in (1..target).step_by(2) {
        total_filled += c * 2;
        diag_count += 1;
        odds += (c / 2 + 1) * 2;
        evens += (c / 2) * 2;
    }
    diag_count += 1;
    debug_println!("filled: {total_filled}");
    debug_println!("diag: {diag_count}");
    debug_println!("odds: {odds}");
    debug_println!("evens: {evens}");
    debug_println!("odds+evens: {}", odds + evens);

    let total = corners
        + diag_count * small_diags
        + (diag_count - 1) * big_diags
        + odds * full_tile_even
        + evens * full_tile_odd;

    println!("{total}");

    Ok(())
}
