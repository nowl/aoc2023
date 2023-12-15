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
    state: Vec<Vec<char>>,
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

fn calculate_load(state: &Vec<Vec<char>>) -> i32 {
    let nrows = state.len();

    state.iter().enumerate().fold(0, |load, (n, row)| {
        let num_round = row.iter().positions(|x| *x == 'O').count();
        load + (num_round * (nrows - n)) as i32
    })
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d14p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    debug_println!("data: {data:?}");

    let new_state = tilt_north(&data.state);

    debug_println!("new_state: {new_state:?}");

    let load = calculate_load(&new_state);

    println!("{load}");

    Ok(())
}
