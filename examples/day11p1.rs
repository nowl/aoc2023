use debug_print::debug_println;
use std::{fs, path::Path};

use anyhow::Error;
use nom::{character::complete::*, multi::many1, sequence::*, IResult};

#[derive(Debug, Eq, PartialEq, Hash, Default, Clone)]
struct Spot {
    row: i32,
    col: i32,
}

#[derive(Debug)]
struct Data {
    num_rows: i32,
    num_cols: i32,
    galaxies: Vec<Spot>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_line = terminated(many1(one_of(".#")), multispace0);
    let (i, lines) = many1(parse_line)(i)?;
    let mut galaxies = vec![];
    let num_rows = lines.len() as i32;
    let num_cols = lines[0].len() as i32;
    lines.into_iter().enumerate().for_each(|(row, line)| {
        line.into_iter().enumerate().for_each(|(col, tile)| {
            let spot = Spot {
                row: row as i32,
                col: col as i32,
            };
            if tile == '#' {
                galaxies.push(spot);
            }
        });
    });
    let data = Data {
        galaxies,
        num_rows,
        num_cols,
    };
    Ok((i, data))
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d11p1t.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    debug_println!("data: {data:?}");

    Ok(())
}
