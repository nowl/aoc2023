use debug_print::debug_println;
use itertools::Itertools;
use std::{fs, path::Path};

use anyhow::Error;
use nom::{bytes::complete::tag, character::complete::*, multi::*, sequence::*, IResult};

#[derive(Debug, Clone, PartialEq, Eq)]
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

fn validate(layout: &Vec<SpringType>, groupings: &Vec<i32>) -> bool {
    use itertools::FoldWhile::*;
    use SpringType::*;
    debug_println!("validating: {layout:?}, {groupings:?}");
    let traversal = layout
        .iter()
        .skip_while(|&e| *e == Operational)
        .fold_while((groupings[0], 0), |current @ (count, idx), e| {
            debug_println!("({count}, {idx}), examining {e:?}");
            match e {
                Damaged if count > 0 => Continue((count - 1, idx)),
                Damaged | Unknown => Done((0, 0)),
                Operational if count == 0 && idx + 1 == groupings.len() => Continue(current),
                Operational if count == 0 => Continue((groupings[idx + 1], idx + 1)),
                Operational if count == groupings[idx] => Continue(current),
                Operational => Done((0, 0)),
            }
        })
        .into_inner();

    debug_println!("final acc: {traversal:?}");

    traversal.0 == 0 && traversal.1 == groupings.len() - 1
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
            let groupings = line.1.iter().map(|s| s.parse().unwrap()).collect_vec();
            let space = line
                .0
                .iter()
                .map(|c| match c {
                    '#' => Damaged,
                    '.' => Operational,
                    '?' => Unknown,
                    _ => unreachable!(),
                })
                .collect_vec();
            Problem { space, groupings }
        })
        .collect_vec();
    let data = Data { problems };
    Ok((i, data))
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
        let mut count = 0;

        let unknown_positions = space
            .iter()
            .positions(|v| *v == SpringType::Unknown)
            .collect_vec();
        debug_println!("unknown_positions {unknown_positions:?}");

        let mut test = space.clone();

        // loop through all possible
        for comb in 0..2i32.pow(unknown_positions.len() as u32) {
            for (i, idx) in unknown_positions.iter().enumerate() {
                test[*idx] = match (comb >> i) & 1 {
                    0 => SpringType::Damaged,
                    1 => SpringType::Operational,
                    _ => unreachable!(),
                };
            }
            debug_println!("test: {test:?}");

            let result = validate(&test, &groupings);
            debug_println!("result {result}");

            if result {
                count += 1;
            }
        }

        total_count += count;
        debug_println!("total_count {total_count}");
    }

    println!("{total_count}");

    Ok(())
}
