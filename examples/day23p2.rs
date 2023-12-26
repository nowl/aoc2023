use aoc2023::pause_enter;
use debug_print::debug_println;
use nom::combinator::eof;
use std::collections::{HashSet, VecDeque};
use std::{fs, path::Path};

use anyhow::Error;
use nom::character::complete::*;
use nom::{multi::*, sequence::*, IResult};

#[derive(Debug)]
struct Data {
    data: Vec<Vec<char>>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let line_parse = terminated(many1(one_of(".#>v")), multispace1);
    let mut parser = terminated(many1(line_parse), eof);
    let (i, data) = parser(i)?;

    let data = Data { data };
    Ok((i, data))
}

fn adj(
    p: (i32, i32),
    map: &Vec<Vec<char>>,
    prev: Option<(i32, i32)>,
    prev_branches: &HashSet<(i32, i32)>,
) -> Vec<(i32, i32)> {
    let nrows = map.len();
    let ncols = map[0].len();

    let mut adjs = vec![];

    macro_rules! check_pos {
        ($dr:expr, $dc:expr) => {{
            let nr = p.0 + $dr;
            let nc = p.1 + $dc;

            if !prev_branches.contains(&(nr, nc)) && (prev == None || prev.unwrap() != (nr, nc)) {
                if nr >= 0 && nr < nrows as i32 && nc >= 0 && nc < ncols as i32 {
                    let tile = map[nr as usize][nc as usize];
                    match tile {
                        '.' | '>' | 'v' => {
                            adjs.push((nr, nc));
                        }
                        _ => (),
                    }
                }
            }
        }};
    }

    check_pos!(1, 0);
    check_pos!(-1, 0);
    check_pos!(0, 1);
    check_pos!(0, -1);

    adjs
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d23p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    debug_println!("{data:?}");

    let map = data.data;

    let nrows = map.len();
    let ncols = map[0].len();

    let start = (0, 1);
    let end = (nrows as i32 - 1, ncols as i32 - 1 - 1);

    let mut max_steps = 0;

    let mut queue = VecDeque::new();

    queue.push_back((start, None, HashSet::new(), 0));

    while let Some((pos, prev, prev_branches, steps)) = queue.pop_front() {
        debug_println!(
            "examining {pos:?}, prev {prev:?}, prev_branches: {prev_branches:?}, steps {steps}"
        );

        if pos == end {
            debug_println!("path hit ending with steps: {steps:?}");
            max_steps = max_steps.max(steps);
            println!("best max: {max_steps}, remaining in queue: {}", queue.len());
            continue;
        }

        let adjs = adj(pos, &map, prev, &prev_branches);

        debug_println!("adjs: {adjs:?}");

        let adjs_len = adjs.len();

        // DFS in order to keep queue small
        if adjs_len > 1 {
            adjs.into_iter().for_each(|a| {
                let mut prev_branches = prev_branches.clone();
                prev_branches.insert(pos);
                prev_branches.insert(a);
                queue.push_front((a, Some(pos), prev_branches, steps + 1));
            });
        } else if adjs_len == 1 {
            // optimized path to save clone of hashset
            queue.push_front((adjs[0], Some(pos), prev_branches, steps + 1));
        }

        //pause_enter();
    }

    println!("{max_steps}");

    Ok(())
}
