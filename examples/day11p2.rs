use debug_print::debug_println;
use std::{collections::HashSet, fs, path::Path};

use anyhow::Error;
use nom::{character::complete::*, multi::many1, sequence::*, IResult};

const EXPANSION_FACTOR: u64 = 1000000;

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

fn find_gaps(data: &Data) -> (HashSet<i32>, HashSet<i32>) {
    let mut row_gaps = HashSet::new();
    let mut col_gaps = HashSet::new();

    let galaxy_rows = data
        .galaxies
        .iter()
        .map(|&Spot { row, .. }| row)
        .collect::<HashSet<_>>();

    let galaxy_cols = data
        .galaxies
        .iter()
        .map(|&Spot { col, .. }| col)
        .collect::<HashSet<_>>();

    for col in 0..data.num_cols {
        if !galaxy_cols.contains(&col) {
            col_gaps.insert(col);
        }
    }

    for row in 0..data.num_rows {
        if !galaxy_rows.contains(&row) {
            row_gaps.insert(row);
        }
    }

    (row_gaps, col_gaps)
}

fn count_path_lengths(data: &Data, row_gaps: &HashSet<i32>, col_gaps: &HashSet<i32>) -> u64 {
    let num_galaxies = data.galaxies.len() as i32;
    let mut count = 0;
    for a in 0..(num_galaxies - 1) {
        for b in a + 1..num_galaxies {
            let Spot {
                row: arow,
                col: acol,
            } = data.galaxies[a as usize];

            let Spot {
                row: brow,
                col: bcol,
            } = data.galaxies[b as usize];

            let mut base_length = ((brow - arow).abs() + (bcol - acol).abs()) as u64;

            debug_println!("base dist {} to {} = {base_length}", a + 1, b + 1);

            // check row gaps
            let x = arow.min(brow);
            let y = arow.max(brow);
            for row in x + 1..=y - 1 {
                if row_gaps.contains(&row) {
                    base_length += EXPANSION_FACTOR - 1;
                }
            }

            // check col gaps
            let x = acol.min(bcol);
            let y = acol.max(bcol);
            for col in x + 1..=y - 1 {
                if col_gaps.contains(&col) {
                    base_length += EXPANSION_FACTOR - 1;
                }
            }

            debug_println!("total dist {} to {} = {base_length}", a + 1, b + 1);

            count += base_length;
        }
    }

    count
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d11p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    debug_println!("data: {data:?}");

    let gaps = find_gaps(&data);

    debug_println!("gaps: {gaps:?}");

    let path_lengths = count_path_lengths(&data, &gaps.0, &gaps.1);

    println!("{path_lengths}");

    Ok(())
}
