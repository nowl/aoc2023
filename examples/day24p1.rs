use debug_print::debug_println;
use itertools::Itertools;
use nom::combinator::{eof, map_res, opt};
use std::{fs, path::Path};

use anyhow::Error;
use nom::character::complete::*;
use nom::{multi::*, sequence::*, IResult};

type I64Tri = (i64, i64, i64);
type F64Tri = (f64, f64, f64);

#[derive(Debug)]
struct Data {
    data: Vec<(I64Tri, I64Tri)>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_int = || {
        map_res(tuple((opt(char('-')), digit1)), |(negate, num)| {
            if negate.is_some() {
                // negative
                let mut s = String::new();
                s += "-";
                s += num;
                s.parse::<i64>()
            } else {
                num.parse::<i64>()
            }
        })
    };

    let loc_parse = || tuple((digit1, char(','), space0, digit1, char(','), space0, digit1));
    let vel_parse = || {
        tuple((
            parse_int(),
            char(','),
            space0,
            parse_int(),
            char(','),
            space0,
            parse_int(),
        ))
    };

    let line_parse = terminated(
        separated_pair(loc_parse(), tuple((space0, char('@'), space0)), vel_parse()),
        multispace1,
    );
    let mut parser = terminated(many1(line_parse), eof);
    let (i, data) = parser(i)?;

    let data = data
        .into_iter()
        .map(|((p1, _, _, p2, _, _, p3), (v1, _, _, v2, _, _, v3))| {
            (
                (
                    p1.parse().unwrap(),
                    p2.parse().unwrap(),
                    p3.parse().unwrap(),
                ),
                (v1, v2, v3),
            )
        })
        .collect();

    let data = Data { data };
    Ok((i, data))
}

// m1 = 1
// m0 = -0.5
// x0 = 19, y0 = 13
// x1 = 18, y1 = 19

// y = mx + b
// y - y0 = m(x - x0)
// x(t) = x0 + dx * t
// y(t) = y0 + dy * t
// x10 + dx1 * t = x20 + dx2 * t
// x1 - x2 = t(dx2 - dx1)
// t = (x1 - x2) / (dx2 - dx1)

fn line2d(p1: I64Tri, v1: I64Tri, p2: I64Tri, v2: I64Tri) -> (F64Tri, f64, f64) {
    let m0 = v1.1 as f64 / v1.0 as f64;
    let m1 = v2.1 as f64 / v2.0 as f64;
    let x0 = p1.0 as f64;
    let y0 = p1.1 as f64;
    let x1 = p2.0 as f64;
    let y1 = p2.1 as f64;

    let a = 1.0 / (m1 - m0) * (-m0 * x0 + y0) + 1.0 / (m0 - m1) * (-m1 * x1 + y1);
    let b = m1 / (m1 - m0) * (-m0 * x0 + y0) + m0 / (m0 - m1) * (-m1 * x1 + y1);

    // x = x0 + v1.0 * t
    let t1 = (a - x0) / v1.0 as f64;
    // y = y1 + v2.1 * t
    let t2 = (b - y1) / v2.1 as f64;

    ((a, b, 0.0), t1, t2)
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d24p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;
    let data = data.data;

    debug_println!("{data:?}");

    let test_area = (200000000000000.0, 400000000000000.0);

    let mut count = 0;
    for v in data.iter().combinations(2) {
        let d0 = v[0];
        let d1 = v[1];
        let (test, t1, t2) = line2d(d0.0, d0.1, d1.0, d1.1);
        debug_println!("{:?}, {:?} -> {:?} at {:?}, {:?}", d0, d1, test, t1, t2);

        if t1 >= 0.0
            && t2 >= 0.0
            && test.0 >= test_area.0
            && test.0 <= test_area.1
            && test.1 >= test_area.0
            && test.1 <= test_area.1
        {
            debug_println!("PASSED: {:?}", test);

            count += 1;
        }
    }

    println!("{count}");

    Ok(())
}
