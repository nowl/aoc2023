use debug_print::debug_println;
use itertools::Itertools;
use nalgebra::{SMatrix, SVector, Vector3};
use nom::combinator::{eof, map_res, opt};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::sync::{Arc, Mutex};
use std::{fs, path::Path};

use anyhow::Error;
use nom::character::complete::*;
use nom::{multi::*, sequence::*, IResult};

type I64Tri = (i64, i64, i64);

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

fn distance_func(
    p1: &Vector3<i128>,
    v1: &Vector3<i128>,
    p2: &Vector3<i128>,
    v2: &Vector3<i128>,
) -> f64 {
    let n = v1.cross(v2);
    let diff = p1 - p2;
    let num = n.dot(&diff);
    let mag = (n.dot(&n) as f64).sqrt();
    (num as f64 / mag).abs()
}

fn error_func(
    p: &Vector3<i128>,
    v: &Vector3<i128>,
    data: &Vec<(Vector3<i128>, Vector3<i128>)>,
) -> f64 {
    data.iter()
        .take(3)
        .map(|(dp, dv)| distance_func(&p, &v, &dp, &dv))
        .sum()
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d24p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;
    let data = data
        .data
        .into_iter()
        .map(|(p, v)| {
            (
                Vector3::<i128>::new(p.0 as i128, p.1 as i128, p.2 as i128),
                Vector3::<i128>::new(v.0 as i128, v.1 as i128, v.2 as i128),
            )
        })
        .collect_vec();

    debug_println!("{data:?}");

    // search around all velocities from -500 to 500

    let b = SVector::<f64, 5>::new(
        data[0].0[0] as f64,
        data[0].0[1] as f64,
        data[0].0[2] as f64,
        data[1].0[0] as f64,
        data[1].0[1] as f64,
        //data[1].0[2] as f64,
    );

    let vrange = 500;

    let cand_pos = Arc::new(Mutex::new(None::<Vector3<i128>>));
    let cand_v = Arc::new(Mutex::new(None::<Vector3<i128>>));
    (-vrange..=vrange).into_par_iter().for_each(|vx| {
        for vy in -vrange..=vrange {
            for vz in -vrange..=vrange {
                #[rustfmt::skip]
                let a = SMatrix::<f64, 5, 5>::new(
                    1.0, 0.0, 0.0, (vx - data[0].1[0]) as f64, 0.0,
                    0.0, 1.0, 0.0, (vy - data[0].1[1]) as f64, 0.0,
                    0.0, 0.0, 1.0, (vz - data[0].1[2]) as f64, 0.0,
                    1.0, 0.0, 0.0, 0.0, (vx - data[1].1[0]) as f64,
                    0.0, 1.0, 0.0, 0.0, (vy - data[1].1[1]) as f64,
                    //0.0, 0.0, 1.0, 0.0, (vz - data[1].1[2]) as f64,
                );
                let decomp = a.lu();
                if let Some(x) = decomp.solve(&b) {
                    let cand = Vector3::<i128>::new(x[0] as i128, x[1] as i128, x[2] as i128);
                    let v = Vector3::<i128>::new(vx, vy, vz);
                    let err = error_func(&cand, &v, &data);
                    if err < 10.0 {
                        debug_println!("cand: {cand:?}");
                        debug_println!("v: {v:?}");
                        debug_println!("err: {err:?}");

                        *cand_pos.lock().unwrap() = Some(cand.clone());
                        *cand_v.lock().unwrap() = Some(v.clone());
                    }
                }
            }
        }
    });

    // now get exact solution
    let cand_pos = cand_pos.lock().unwrap().unwrap().clone();
    let cand_v = cand_v.lock().unwrap().unwrap().clone();

    let mut final_pos = Vector3::<i128>::new(0, 0, 0);

    let search_span = 100;

    for vx in -search_span..=search_span {
        for vy in -search_span..=search_span {
            for vz in -search_span..=search_span {
                let offset = Vector3::<i128>::new(vx, vy, vz);
                let cand = cand_pos + offset;
                let err = error_func(&cand, &cand_v, &data);
                if err == 0.0 {
                    debug_println!("pos: {cand:?}");
                    debug_println!("v: {cand_v:?}");
                    debug_println!("err: {err:?}");
                    final_pos = cand;
                }
            }
        }
    }

    // get final answer
    let answer = final_pos[0] + final_pos[1] + final_pos[2];

    println!("{answer}");

    Ok(())
}
