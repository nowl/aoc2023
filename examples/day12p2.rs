use debug_print::debug_println;
use itertools::Itertools;
use std::{fs, path::Path};

use anyhow::Error;
use nom::{bytes::complete::tag, character::complete::*, multi::*, sequence::*, IResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringType {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Problem {
    space: Vec<SpringType>,
    groupings: Vec<i32>,
}

#[derive(Debug)]
struct Data {
    problems: Vec<Problem>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_space = terminated(many1(one_of(".#?")), space1);
    let parse_groupings = terminated(separated_list1(tag(","), digit1), multispace1);
    let parse_line = pair(parse_space, parse_groupings);
    let (i, lines) = many1(parse_line)(i)?;

    let problems = lines
        .into_iter()
        .map(|line| {
            use SpringType::*;
            let groupings = line
                .1
                .iter()
                .map(|s| s.parse().unwrap())
                .collect_vec()
                .repeat(5);
            let mut space = line
                .0
                .iter()
                .map(|c| match c {
                    '#' => Damaged,
                    '.' => Operational,
                    '?' => Unknown,
                    _ => unreachable!(),
                })
                .collect_vec();
            space.push(Unknown);
            space = space.repeat(5);
            space.pop().unwrap();
            Problem { space, groupings }
        })
        .collect_vec();
    let data = Data { problems };
    Ok((i, data))
}

fn determine_match(
    space: &Vec<SpringType>,
    table: &Vec<Vec<u64>>,
    i: usize,
    j: usize,
    val: i32,
) -> u64 {
    let test = space[j..(j + val as usize).min(space.len())]
        .iter()
        .collect_vec();
    debug_println!("testing against {test:?}");
    let next_idx = j + val as usize;
    if next_idx < space.len() && space[next_idx] == SpringType::Damaged {
        debug_println!("early exit, no match due to ending");
        return 0;
    }
    if test.len() == val as usize
        && test
            .iter()
            .all(|&v| *v == SpringType::Unknown || *v == SpringType::Damaged)
    {
        debug_println!("got a match");
        table[i - 1][(j + val as usize + 1).min(table[i - 1].len() - 1)]
    } else {
        0
    }
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d12p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    debug_println!("data: {data:?}");

    let mut total_count = 0;

    for Problem { space, groupings } in data.problems {
        use SpringType::*;

        let mut table = vec![];
        let mut row0 = itertools::repeat_n(0, space.len()).collect_vec();
        row0.push(1);
        table.push(row0);
        for _ in 0..groupings.len() {
            table.push(itertools::repeat_n(0, space.len() + 1).collect_vec());
        }

        for j in (0..space.len()).rev() {
            if space[j] == SpringType::Damaged {
                break;
            }

            table[0][j] = table[0][j + 1];
        }

        for i in 1..=groupings.len() {
            let val = groupings[groupings.len() - i];
            debug_println!("examining val: {val}");
            for j in (0..space.len()).rev() {
                debug_println!("{i}, {j}");
                let combinations = match space[j] {
                    Operational => table[i][j + 1],
                    Damaged => determine_match(&space, &table, i, j, val),
                    Unknown => table[i][j + 1] + determine_match(&space, &table, i, j, val),
                };

                debug_println!("result is combinations {combinations}");

                table[i][j] = combinations;

                debug_println!("{table:?}");
            }
        }

        debug_println!("{table:?}");

        total_count += table[groupings.len()][0];
    }

    println!("{total_count}");

    Ok(())
}
