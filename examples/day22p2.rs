use debug_print::debug_println;
use itertools::{Itertools, MinMaxResult};
use nom::combinator::eof;
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::atomic::AtomicI64;
use std::{fs, path::Path};

use anyhow::Error;
use nom::character::complete::*;
use nom::{multi::*, sequence::*, IResult};

#[derive(Debug)]
struct BrickData {
    start: (i32, i32, i32),
    end: (i32, i32, i32),
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Brick {
    id: i32,
    start: (i32, i32, i32),
    end: (i32, i32, i32),
    //internal: HashSet<(i32, i32, i32)>,
}

fn points_contained(
    (sx, sy, sz): (i32, i32, i32),
    (ex, ey, ez): (i32, i32, i32),
) -> HashSet<(i32, i32, i32)> {
    let mut r = HashSet::new();
    if sx != ex {
        for n in sx.min(ex)..=sx.max(ex) {
            r.insert((n, sy, sz));
        }
    } else if sy != ey {
        for n in sy.min(ey)..=sy.max(ey) {
            r.insert((sx, n, sz));
        }
    } else {
        for n in sz.min(ez)..=sz.max(ez) {
            r.insert((sx, sy, n));
        }
    }
    r
}

impl Brick {
    fn new(id: i32, start: (i32, i32, i32), end: (i32, i32, i32)) -> Self {
        Brick {
            id,
            start,
            end,
            //internal: points_contained(start, end),
        }
    }

    fn on_ground(&self) -> bool {
        self.start.2 == 1 || self.end.2 == 1
    }

    fn intersect_point(&self, p: &(i32, i32, i32)) -> bool {
        let internal = points_contained(self.start, self.end);
        internal.contains(p)
    }

    fn intersect_brick(&self, o: &Brick) -> bool {
        let internal = points_contained(self.start, self.end);
        internal.iter().any(|p| o.intersect_point(p))
    }

    fn can_drop(&self, others: &Vec<Brick>) -> bool {
        if self.on_ground() {
            return false;
        }

        let (sx, sy, sz) = self.start;
        let (ex, ey, ez) = self.end;

        let b = Brick::new(-1, (sx, sy, sz - 1), (ex, ey, ez - 1));
        let intersect = others
            .iter()
            .filter(|&x| x != self)
            .any(|x| x.intersect_brick(&b));

        if intersect {
            return false;
        }

        true
    }

    fn apply_drop(&mut self) {
        let (sx, sy, sz) = self.start;
        let (ex, ey, ez) = self.end;

        self.start = (sx, sy, sz - 1);
        self.end = (ex, ey, ez - 1);
        //self.internal = points_contained(self.start, self.end);
    }
}

#[derive(Debug)]
struct Data {
    data: Vec<BrickData>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let triple_parse = || tuple((digit1, char(','), digit1, char(','), digit1));
    let line_parse = terminated(
        separated_pair(triple_parse(), char('~'), triple_parse()),
        multispace1,
    );
    let mut parser = terminated(many1(line_parse), eof);
    let (i, data) = parser(i)?;

    let data = data
        .iter()
        .map(|(p1, p2)| BrickData {
            start: (
                p1.0.parse().unwrap(),
                p1.2.parse().unwrap(),
                p1.4.parse().unwrap(),
            ),
            end: (
                p2.0.parse().unwrap(),
                p2.2.parse().unwrap(),
                p2.4.parse().unwrap(),
            ),
        })
        .collect_vec();

    let data = Data { data };
    Ok((i, data))
}

fn settle_bricks(bricks: &mut Vec<Brick>) -> bool {
    let mut movement = false;
    loop {
        let mut apply_drops = vec![];
        for (n, b) in bricks.iter().enumerate() {
            debug_println!("examining {b:?}");
            let can_drop = b.can_drop(&bricks);
            debug_println!("can_drop {can_drop:?}");
            if can_drop {
                apply_drops.push(n);
            }
        }
        if apply_drops.is_empty() {
            break;
        }
        movement = true;
        for n in apply_drops {
            bricks[n].apply_drop();
        }
    }
    movement
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d22p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    debug_println!("{data:?}");

    let mut bricks = data
        .data
        .iter()
        .enumerate()
        .map(|(n, bd)| Brick::new(n as i32, bd.start, bd.end))
        .collect_vec();

    debug_println!("{bricks:?}");

    let movement = settle_bricks(&mut bricks);
    debug_assert_eq!(true, movement);

    let orig_brick_hashset = bricks.iter().cloned().collect::<HashSet<Brick>>();

    // loop through each brick and remove to determine movement
    let mut total_movement = AtomicI64::new(0);
    (0..bricks.len()).into_par_iter().for_each(|n| {
        debug_println!("n = {n}");
        let b = bricks[n].clone();
        debug_println!("removing brick {b:?}");
        let mut bricks = bricks.clone();
        bricks.swap_remove(n);
        settle_bricks(&mut bricks);

        let brick_hashset = bricks.iter().cloned().collect::<HashSet<Brick>>();
        let count = brick_hashset.difference(&orig_brick_hashset).count();
        debug_println!("count: {count}");
        total_movement.fetch_add(count as i64, std::sync::atomic::Ordering::SeqCst);
    });

    println!("{}", total_movement.get_mut());

    Ok(())
}
