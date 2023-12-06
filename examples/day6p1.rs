use std::path::Path;

use anyhow::{anyhow, Error};
use aoc2023::{cap_name_str, read_as_lines};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_TIME: Regex = Regex::new(r"Time:\s+(?<time>.*)").unwrap();
    static ref REGEX_DISTANCE: Regex = Regex::new(r"Distance:\s+(?<distance>.*)").unwrap();
    static ref REGEX_NUM: Regex = Regex::new(r"\s*(?<num>\d+)\s*").unwrap();
}

#[derive(Debug)]
struct Race {
    time: i32,
    distance: i32,
}

fn calculate_distances(total_time: i32) -> Vec<i32> {
    let mut distances = vec![];
    for hold_time in 0..=total_time {
        let speed = hold_time;
        let distance = (total_time - hold_time) * speed;
        distances.push(distance);
    }
    distances
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d6p1.txt");
    let lines = read_as_lines(file)?;

    let times = {
        let cap = REGEX_TIME.captures(&lines[0]).unwrap();
        let times = cap_name_str!(cap, "time");
        REGEX_NUM
            .captures_iter(&times)
            .map(|c| c.name("num").unwrap().as_str().parse::<i32>().unwrap())
            .collect_vec()
    };

    let distances = {
        let cap = REGEX_DISTANCE.captures(&lines[1]).unwrap();
        let distances = cap_name_str!(cap, "distance");
        REGEX_NUM
            .captures_iter(&distances)
            .map(|c| c.name("num").unwrap().as_str().parse::<i32>().unwrap())
            .collect_vec()
    };

    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| Race { time, distance })
        .collect_vec();

    let score = races
        .iter()
        .map(|race| {
            let distances = calculate_distances(race.time);
            let record_count = distances.iter().filter(|&d| *d > race.distance).count();
            //println!("{record_count:?}");
            record_count
        })
        .product::<usize>();

    println!("{score:?}");

    Ok(())
}
