use debug_print::{debug_print, debug_println};
use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;
use std::{fs, path::Path};

use anyhow::Error;
use nom::character::complete::*;
use nom::{bytes::complete::tag, combinator::map_res, multi::*, sequence::*, IResult};

#[derive(Debug)]
struct Data {
    state: Vec<Plan>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Plan {
    dir: Dir,
    length: i32,
    color: String,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Loc {
    row: i32,
    col: i32,
}

#[derive(Debug)]
struct MapTile {
    walls: Vec<Dir>,
    inside: bool,
}

type Map = HashMap<Loc, MapTile>;

fn draw_map(inputs: &Vec<Plan>) -> Map {
    let mut map = HashMap::new();
    let mut cursor = Loc { row: 0, col: 0 };

    map.insert(
        cursor.clone(),
        MapTile {
            walls: vec![],
            inside: true,
        },
    );

    for input in inputs {
        let (dc, dr) = match input.dir {
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
        };

        if let Some(mt) = map.get_mut(&cursor) {
            mt.walls.push(input.dir.clone())
        }

        for _ in 0..input.length {
            cursor.row += dr;
            cursor.col += dc;

            map.entry(cursor.clone())
                .and_modify(|mt| mt.walls.push(input.dir.clone()))
                .or_insert(MapTile {
                    walls: vec![input.dir.clone()],
                    inside: true,
                });
        }
    }

    map
}

fn fill_inner(map: &mut Map) {
    use Dir::*;

    let MinMaxResult::MinMax(minr, maxr) = map.keys().map(|x| x.row).minmax() else {
        panic!("can't find min and max");
    };

    let MinMaxResult::MinMax(minc, maxc) = map.keys().map(|x| x.col).minmax() else {
        panic!("can't find min and max");
    };

    for row in minr..=maxr {
        for col in minc..=maxc {
            if map.contains_key(&Loc { row, col }) {
                continue;
            }

            // check Up, Down, Down+Left, Down+Right, Left+Up, Right+Up
            let mut count = 0;
            macro_rules! update_count {
                ($r:expr, $c:expr) => {
                    if let Some(mt) = map.get(&Loc { row: $r, col: $c }) {
                        let w = &mt.walls;
                        if w.len() == 1 && (w[0] == Up || w[0] == Down) {
                            count += 1;
                        } else if w.len() == 2 {
                            if (w[0] == Down && w[1] == Left)
                                || (w[0] == Down && w[1] == Right)
                                || (w[0] == Left && w[1] == Up)
                                || (w[0] == Right && w[1] == Up)
                            {
                                count += 1;
                            }
                        }
                    }
                };
            }

            if row == 0 && col < 0 {
                for c in minc..col {
                    update_count!(row, c);
                }
            } else {
                for c in col..=maxc {
                    update_count!(row, c);
                }
            }

            if count % 2 == 1 {
                map.insert(
                    Loc { row, col },
                    MapTile {
                        walls: vec![],
                        inside: true,
                    },
                );
            }
        }
    }
}

fn count_filled(map: &Map) -> i32 {
    let MinMaxResult::MinMax(minr, maxr) = map.keys().map(|x| x.row).minmax() else {
        panic!("can't find min and max");
    };

    let MinMaxResult::MinMax(minc, maxc) = map.keys().map(|x| x.col).minmax() else {
        panic!("can't find min and max");
    };

    let mut count = 0;

    for row in minr..=maxr {
        for col in minc..=maxc {
            if let Some(mt) = map.get(&Loc { row, col }) {
                if mt.inside {
                    count += 1;
                }
            }
        }
    }

    count
}

fn parse_plan(i: &str) -> IResult<&str, Plan> {
    let (i, dir) = terminated(one_of("UDLR"), space1)(i)?;
    let dir = match dir {
        'U' => Dir::Up,
        'D' => Dir::Down,
        'L' => Dir::Left,
        'R' => Dir::Right,
        _ => unreachable!(),
    };
    let (i, length) = map_res(terminated(digit1, space1), |c: &str| c.parse::<i32>())(i)?;
    let (i, color) = terminated(
        delimited(char('('), preceded(tag("#"), alphanumeric1), char(')')),
        multispace0,
    )(i)?;

    let plan = Plan {
        dir,
        length,
        color: color.to_string(),
    };

    Ok((i, plan))
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let (i, state) = terminated(many1(parse_plan), multispace0)(i)?;
    let data = Data { state };
    Ok((i, data))
}

fn print_map(map: &Map) {
    let MinMaxResult::MinMax(minr, maxr) = map.keys().map(|x| x.row).minmax() else {
        panic!("can't find min and max");
    };

    let MinMaxResult::MinMax(minc, maxc) = map.keys().map(|x| x.col).minmax() else {
        panic!("can't find min and max");
    };

    for row in minr..=maxr {
        for col in minc..=maxc {
            let c = {
                if let Some(mt) = map.get(&Loc { row, col }) {
                    if mt.inside {
                        '#'
                    } else {
                        '.'
                    }
                } else {
                    '?'
                }
            };
            debug_print!("{c}");
        }
        debug_println!();
    }
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d18p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    debug_println!("data: {data:?}");

    let mut map = draw_map(&data.state);

    print_map(&map);

    debug_println!();

    fill_inner(&mut map);

    print_map(&map);

    let count = count_filled(&map);

    println!("{count}");

    Ok(())
}
