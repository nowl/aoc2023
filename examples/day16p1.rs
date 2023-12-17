use debug_print::debug_println;
use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    fs,
    iter::repeat,
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
    state: Vec<Vec<char>>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_line = terminated(many1(one_of(r".|-/\")), newline);
    let (i, state) = terminated(many1(parse_line), multispace0)(i)?;
    let data = Data { state };
    Ok((i, data))
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Beam {
    row: i32,
    col: i32,
    dir: Dir,
}

impl Beam {
    fn proj(&self, dir: Dir) -> Beam {
        use Dir::*;
        match dir {
            Up => Beam {
                col: self.col,
                row: self.row - 1,
                dir,
            },
            Down => Beam {
                col: self.col,
                row: self.row + 1,
                dir,
            },
            Left => Beam {
                col: self.col - 1,
                row: self.row,
                dir,
            },
            Right => Beam {
                col: self.col + 1,
                row: self.row,
                dir,
            },
        }
    }
}

fn trace_beam(b: Beam, e: &mut Vec<Vec<HashSet<Dir>>>, g: &Vec<Vec<char>>) -> Vec<Beam> {
    use Dir::*;

    let nrows = g.len();
    let ncols = g[0].len();

    if b.row < 0 || b.row >= nrows as i32 || b.col < 0 || b.col >= ncols as i32 {
        return vec![];
    }

    let row = b.row as usize;
    let col = b.col as usize;

    debug_println!("examining {row}, {col}");

    if e[row][col].contains(&b.dir) {
        return vec![];
    } else {
        e[row][col].insert(b.dir.clone());
    }
    match g[row][col] {
        '.' => vec![b.proj(b.dir.clone())],
        '|' => {
            if b.dir == Left || b.dir == Right {
                vec![b.proj(Up), b.proj(Down)]
            } else {
                vec![b.proj(b.dir.clone())]
            }
        }
        '-' => {
            if b.dir == Up || b.dir == Down {
                vec![b.proj(Right), b.proj(Left)]
            } else {
                vec![b.proj(b.dir.clone())]
            }
        }
        '\\' => match b.dir {
            Up => vec![b.proj(Left)],
            Down => vec![b.proj(Right)],
            Left => vec![b.proj(Up)],
            Right => vec![b.proj(Down)],
        },
        '/' => match b.dir {
            Up => vec![b.proj(Right)],
            Down => vec![b.proj(Left)],
            Left => vec![b.proj(Down)],
            Right => vec![b.proj(Up)],
        },
        _ => unreachable!(),
    }
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d16p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    debug_println!("data: {data:?}");

    let mut beams = VecDeque::new();
    beams.push_back(Beam {
        row: 0,
        col: 0,
        dir: Dir::Right,
    });

    let mut energized_areas = {
        let mut a = vec![];
        for r in data.state.iter() {
            let b = repeat(HashSet::new()).take(r.len()).collect_vec();
            a.push(b);
        }
        a
    };

    while let Some(beam) = beams.pop_front() {
        let new_beams = trace_beam(beam, &mut energized_areas, &data.state);
        for beam in new_beams {
            beams.push_back(beam);
        }

        debug_println!("{energized_areas:?}");
    }

    let energized = energized_areas
        .iter()
        .map(|row| row.iter().filter(|&v| !v.is_empty()).count() as i32)
        .sum::<i32>();

    println!("{energized}");

    Ok(())
}
