use debug_print::debug_println;
use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
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

#[derive(Debug)]
struct Data {
    state: Vec<Vec<u8>>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_line = terminated(digit1, newline);
    let (i, state) = terminated(many1(parse_line), multispace0)(i)?;
    let state = state
        .iter()
        .map(|x| {
            x.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let data = Data { state };
    Ok((i, data))
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d17p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    debug_println!("data: {data:?}");

    let grid = data.state;
    let nrows = grid.len() as i32;
    let ncols = grid[0].len() as i32;

    let mut queue = BinaryHeap::new();
    let mut visited_states = HashSet::new();

    queue.push(Reverse((0, 0, 0, 0, None)));

    let mut min_cost = None;

    while let Some(Reverse((pcost, mp, row, col, prev_dir))) = queue.pop() {
        use Dir::*;

        debug_println!(
            "examining {row},{col} from the {prev_dir:?} with cost {pcost} and move points {mp}"
        );
        // check out of bounds
        if row < 0 || row >= nrows || col < 0 || col >= ncols {
            continue;
        }

        // determine current cost
        let cost = if let None = prev_dir {
            assert_eq!(pcost, 0);
            pcost
        } else {
            let gc = grid[row as usize][col as usize] as i32;
            pcost + gc
        };

        // check exceeded cost
        if let Some(min_c) = min_cost {
            if cost > min_c {
                break;
            }
        }

        // check exceeded movement
        if mp > 10 {
            continue;
        }

        // check end condition
        if row == nrows - 1 && col == nrows - 1 && mp >= 4 {
            if let Some(c) = min_cost {
                if cost < c {
                    min_cost = Some(cost);
                }
            } else {
                min_cost = Some(cost);
            }
            continue;
        }

        macro_rules! maybe_push_state {
            ($state:expr) => {
                let state = $state;
                let row_col = (state.1, state.2, state.3, state.4.clone());
                if !visited_states.contains(&row_col) {
                    visited_states.insert(row_col);
                    queue.push(Reverse(state));
                }
            };
        }

        // add move scenarios
        match prev_dir {
            None => {
                maybe_push_state!((cost, 1, row, col + 1, Some(Right)));
                maybe_push_state!((cost, 1, row + 1, col, Some(Down)));
            }
            Some(Up) => {
                if mp >= 4 {
                    maybe_push_state!((cost, 1, row, col - 1, Some(Left)));
                    maybe_push_state!((cost, 1, row, col + 1, Some(Right)));
                    maybe_push_state!((cost, mp + 1, row - 1, col, Some(Up)));
                } else {
                    maybe_push_state!((cost, mp + 1, row - 1, col, Some(Up)));
                }
            }
            Some(Down) => {
                if mp >= 4 {
                    maybe_push_state!((cost, 1, row, col - 1, Some(Left)));
                    maybe_push_state!((cost, 1, row, col + 1, Some(Right)));
                    maybe_push_state!((cost, mp + 1, row + 1, col, Some(Down)));
                } else {
                    maybe_push_state!((cost, mp + 1, row + 1, col, Some(Down)));
                }
            }
            Some(Left) => {
                if mp >= 4 {
                    maybe_push_state!((cost, 1, row - 1, col, Some(Up)));
                    maybe_push_state!((cost, 1, row + 1, col, Some(Down)));
                    maybe_push_state!((cost, mp + 1, row, col - 1, Some(Left)));
                } else {
                    maybe_push_state!((cost, mp + 1, row, col - 1, Some(Left)));
                }
            }
            Some(Right) => {
                if mp >= 4 {
                    maybe_push_state!((cost, 1, row - 1, col, Some(Up)));
                    maybe_push_state!((cost, 1, row + 1, col, Some(Down)));
                    maybe_push_state!((cost, mp + 1, row, col + 1, Some(Right)));
                } else {
                    maybe_push_state!((cost, mp + 1, row, col + 1, Some(Right)));
                }
            }
        }
    }

    if let Some(mc) = min_cost {
        println!("{mc}");
    };

    Ok(())
}
