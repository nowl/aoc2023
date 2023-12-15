use debug_print::debug_println;
use itertools::Itertools;
use num::Integer;
use std::{
    collections::HashMap,
    fmt::{Display, Write},
    fs,
    path::Path,
};

use anyhow::Error;
use nom::{
    character::{complete::*, streaming::newline},
    multi::*,
    sequence::*,
    IResult,
};

const TOTAL_CYCLES: i32 = 1000000000;

#[derive(Debug)]
struct Data {
    state: Vec<Vec<char>>,
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.state.iter() {
            for c in row.iter() {
                f.write_char(*c)?
            }
            f.write_char('\n')?
        }
        Ok(())
    }
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_line = terminated(many1(one_of(".#O")), newline);
    let mut parse_problem = terminated(many1(parse_line), multispace0);
    let (i, state) = parse_problem(i)?;
    let data = Data { state };
    Ok((i, data))
}

fn tilt_north(state: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut state = state.clone();
    let ncols = state[0].len();
    let nrows = state.len();

    for col in 0..ncols {
        let mut drop_point = 0;
        for row in 0..nrows {
            let spot = state[row][col];
            match spot {
                '.' => (),
                'O' => {
                    let old = state[drop_point][col];
                    state[drop_point][col] = 'O';
                    state[row][col] = old;
                    drop_point += 1;
                }
                '#' => drop_point = row + 1,
                _ => unreachable!(),
            }
        }
    }

    state
}

fn tilt_south(state: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut state = state.clone();
    let ncols = state[0].len();
    let nrows = state.len();

    for col in 0..ncols {
        let mut drop_point = nrows as i32 - 1;
        for row in (0..nrows).rev() {
            let spot = state[row][col];
            match spot {
                '.' => (),
                'O' => {
                    let old = state[drop_point as usize][col];
                    state[drop_point as usize][col] = 'O';
                    state[row][col] = old;
                    drop_point -= 1;
                }
                '#' => drop_point = row as i32 - 1,
                _ => unreachable!(),
            }
        }
    }

    state
}

fn tilt_west(state: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut state = state.clone();
    let ncols = state[0].len();
    let nrows = state.len();

    for row in 0..nrows {
        let mut drop_point = 0;
        for col in 0..ncols {
            let spot = state[row][col];
            match spot {
                '.' => (),
                'O' => {
                    let old = state[row][drop_point];
                    state[row][drop_point] = 'O';
                    state[row][col] = old;
                    drop_point += 1;
                }
                '#' => drop_point = col + 1,
                _ => unreachable!(),
            }
        }
    }

    state
}

fn tilt_east(state: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut state = state.clone();
    let ncols = state[0].len();
    let nrows = state.len();

    for row in 0..nrows {
        let mut drop_point = ncols as i32 - 1;
        for col in (0..ncols).rev() {
            let spot = state[row][col];
            match spot {
                '.' => (),
                'O' => {
                    let old = state[row][drop_point as usize];
                    state[row][drop_point as usize] = 'O';
                    state[row][col] = old;
                    drop_point -= 1;
                }
                '#' => drop_point = col as i32 - 1,
                _ => unreachable!(),
            }
        }
    }

    state
}

fn calculate_load(state: &Vec<Vec<char>>) -> i32 {
    let nrows = state.len();

    state.iter().enumerate().fold(0, |load, (n, row)| {
        let num_round = row.iter().positions(|x| *x == 'O').count();
        load + (num_round * (nrows - n)) as i32
    })
}

fn run_cycle(state: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let state = tilt_north(&state);
    let state = tilt_west(&state);
    let state = tilt_south(&state);
    tilt_east(&state)
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d14p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    debug_println!("data: {data:?}");

    let mut hash = HashMap::new();

    let mut state = data.state.clone();
    let mut earliest_cycle = None;
    let mut matched_cycle = None;
    for n in 0..10000 {
        state = run_cycle(&state);

        // detect repetition in cycles
        if let Some(&earlier_cycle) = hash.get(&state) {
            debug_println!(
                "found repetition on cycle {} that matches cycle {}",
                n,
                earlier_cycle
            );
            earliest_cycle = Some(earlier_cycle);
            matched_cycle = Some(n);
            break;
        } else {
            hash.insert(state.clone(), n);
        }
    }

    let earliest_cycle = earliest_cycle.ok_or(anyhow::format_err!("No cycles found.."))?;
    let matched_cycle = matched_cycle.ok_or(anyhow::format_err!("No cycles found.."))?;

    // now that cycles are detected skip to furthest spot
    let (skip, remaining_cycles) =
        (TOTAL_CYCLES - earliest_cycle).div_rem(&(matched_cycle - earliest_cycle));
    debug_println!(
        "run to cycle {}, skip {}, run {} more cycles",
        earliest_cycle,
        skip * (matched_cycle - earliest_cycle),
        remaining_cycles
    );

    let total_run_cycles = earliest_cycle + remaining_cycles;

    let mut state = data.state.clone();
    for _ in 0..total_run_cycles {
        state = run_cycle(&state);
    }

    let load = calculate_load(&state);

    println!("{load}");

    Ok(())
}
