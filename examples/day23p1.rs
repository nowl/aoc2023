use aoc2023::pause_enter;
use debug_print::debug_println;
use nom::combinator::eof;
use std::collections::VecDeque;
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

fn adj(p: (i32, i32), map: &Vec<Vec<char>>, prev: Option<(i32, i32)>) -> Vec<(i32, i32)> {
    let nrows = map.len();
    let ncols = map[0].len();

    let mut adjs = vec![];

    macro_rules! check_pos {
        ($dr:expr, $dc:expr) => {{
            let nr = p.0 + $dr;
            let nc = p.1 + $dc;

            if prev == None || prev.unwrap() != (nr, nc) {
                if nr >= 0 && nr < nrows as i32 && nc >= 0 && nc < ncols as i32 {
                    let tile = map[nr as usize][nc as usize];
                    match tile {
                        '.' => {
                            adjs.push((nr, nc));
                        }
                        '>' if $dc == 1 => {
                            adjs.push((nr, nc));
                        }
                        'v' if $dr == 1 => {
                            adjs.push((nr, nc));
                        }
                        _ => (),
                    }
                }
            }
        }};
    }

    let tile = map[p.0 as usize][p.1 as usize];

    if tile == '>' {
        return vec![(p.0, p.1 + 1)];
    } else if tile == 'v' {
        return vec![(p.0 + 1, p.1)];
    } else {
        check_pos!(1, 0);
        check_pos!(-1, 0);
        check_pos!(0, 1);
        check_pos!(0, -1);
    }

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

    queue.push_back((start, None, 0));

    while let Some((pos, prev, steps)) = queue.pop_front() {
        debug_println!("examining {pos:?}, prev {prev:?}, steps {steps}");

        if pos == end {
            debug_println!("path hit ending with steps: {steps:?}");
            max_steps = max_steps.max(steps);
        }

        let adjs = adj(pos, &map, prev);

        debug_println!("adjs: {adjs:?}");

        adjs.into_iter().for_each(|a| {
            queue.push_back((a, Some(pos), steps + 1));
        });

        //pause_enter();
    }

    println!("{max_steps}");

    Ok(())
}
