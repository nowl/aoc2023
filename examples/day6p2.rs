use std::path::Path;

use anyhow::{anyhow, Error};
use aoc2023::{cap_name_str, read_as_lines};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_TIME: Regex = Regex::new(r"Time:\s+(?<time>.*)").unwrap();
    static ref REGEX_DISTANCE: Regex = Regex::new(r"Distance:\s+(?<distance>.*)").unwrap();
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn calculate_distance_span(total_time: u64, target: u64) -> (u64, u64) {
    let mut start = 0;
    for hold_time in 0..=total_time {
        let speed = hold_time;
        let distance = (total_time - hold_time) * speed;
        if distance > target {
            start = hold_time;
            break;
        }
    }

    (start, total_time - start)
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d6p1.txt");
    let lines = read_as_lines(file)?;

    let time = {
        let cap = REGEX_TIME.captures(&lines[0]).unwrap();
        let times = cap_name_str!(cap, "time").replace(" ", "");
        times.parse::<u64>().unwrap()
    };

    let distance = {
        let cap = REGEX_DISTANCE.captures(&lines[1]).unwrap();
        let distances = cap_name_str!(cap, "distance").replace(" ", "");
        distances.parse::<u64>().unwrap()
    };

    let race = Race { time, distance };

    let distances = calculate_distance_span(race.time, race.distance);
    let score = distances.1 - distances.0 + 1;

    println!("{score:?}");

    Ok(())
}
