use std::{path::Path, str::FromStr};

use anyhow::{anyhow, Error};
use aoc2023::{cap_name_str, read_as_lines};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_SEED: Regex = Regex::new(r"seeds:\s+(?<seeds>.*)").unwrap();
}

#[derive(Debug)]
struct Span {
    dst: u64,
    src: u64,
    len: u64,
}

impl FromStr for Span {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim().split(" ");
        let dst = split.next().unwrap().parse()?;
        let src = split.next().unwrap().parse()?;
        let len = split.next().unwrap().parse()?;
        Ok(Span { dst, src, len })
    }
}

impl Span {
    // returns mapped span, residuals
    fn map_src(&self, (start, len): (u64, u64)) -> (Vec<(u64, u64)>, Vec<(u64, u64)>) {
        let end = start + len - 1;
        let s = self.src;
        let e = self.src + self.len - 1;

        if end <= s || start > e {
            (vec![], vec![(start, len)])
        } else if start < s && end >= s && end <= e {
            (vec![(self.dst, end - s + 1)], vec![(start, s - start + 1)])
        } else if end > e && start <= e && start >= s {
            (
                vec![(self.dst + start - s, e - start + 1)],
                vec![(e + 1, end - e)],
            )
        } else if s <= start && e >= end {
            (vec![(self.dst + start - s, end - start + 1)], vec![])
        } else if s >= start && end >= e {
            (
                vec![(self.dst, e - s + 1)],
                vec![(start, s - start + 1), (e + 1, end - e)],
            )
        } else {
            unreachable!()
        }
    }
}

fn parse_mapping(lines: &Vec<String>, mapping: &str) -> Vec<Span> {
    let start = lines.iter().position(|l| l == mapping).unwrap();
    lines
        .iter()
        .skip(start + 1)
        .take_while(|&s| s != "")
        .map(|s| s.parse())
        .collect::<Result<Vec<Span>, _>>()
        .unwrap()
}

fn determine_mappings(inputs: &Vec<(u64, u64)>, mappings: &Vec<Span>) -> Vec<(u64, u64)> {
    //println!("inputs: {inputs:?}");
    //println!("mappings: {mappings:?}");
    let mut outputs = vec![];
    for s in inputs {
        let (mut mapped, mut unmapped): (Vec<(u64, u64)>, Vec<(u64, u64)>) =
            mappings
                .iter()
                .fold((vec![], vec![*s]), |(mut mapped, unmapped), m| {
                    let (mapped2, unmapped2): (Vec<_>, Vec<_>) =
                        unmapped.iter().map(|input| m.map_src(*input)).unzip();
                    let mut mapped2 = mapped2.iter().flatten().cloned().collect_vec();
                    let unmapped2 = unmapped2.iter().flatten().cloned().collect_vec();
                    mapped.append(&mut mapped2);
                    //println!("mapped: {mapped:?}");
                    //println!("unmapped2: {unmapped2:?}");
                    (mapped, unmapped2)
                });
        outputs.append(&mut mapped);
        outputs.append(&mut unmapped);
    }
    //println!("outputs: {outputs:?}");
    outputs
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d5p1.txt");
    let lines = read_as_lines(file)?;

    let seeds = {
        let cap = REGEX_SEED.captures(lines.iter().next().unwrap()).unwrap();
        let seeds = cap_name_str!(cap, "seeds");
        let seeds = seeds
            .split(" ")
            .map(|s| s.parse())
            .collect::<Result<Vec<u64>, _>>()?;
        seeds.chunks(2).map(|v| (v[0], v[1])).collect::<Vec<_>>()
    };

    let seed_to_soil = parse_mapping(&lines, "seed-to-soil map:");
    let soil_to_fertilizer = parse_mapping(&lines, "soil-to-fertilizer map:");
    let fertilizer_to_water = parse_mapping(&lines, "fertilizer-to-water map:");
    let water_to_light = parse_mapping(&lines, "water-to-light map:");
    let light_to_temperature = parse_mapping(&lines, "light-to-temperature map:");
    let temperature_to_humidity = parse_mapping(&lines, "temperature-to-humidity map:");
    let humidity_to_location = parse_mapping(&lines, "humidity-to-location map:");

    let min_location = seeds
        .iter()
        .map(|&s| {
            //println!("seed: {s:?}");
            let soil = determine_mappings(&vec![s], &seed_to_soil);
            //println!("soil: {soil:?}");
            let fertilizer = determine_mappings(&soil, &soil_to_fertilizer);
            //println!("fertilizer: {fertilizer:?}");
            let water = determine_mappings(&fertilizer, &fertilizer_to_water);
            //println!("water: {water:?}");
            let light = determine_mappings(&water, &water_to_light);
            //println!("light: {light:?}");
            let temperature = determine_mappings(&light, &light_to_temperature);
            //println!("temperature: {temperature:?}");
            let humidity = determine_mappings(&temperature, &temperature_to_humidity);
            //println!("humidity: {humidity:?}");
            let location = determine_mappings(&humidity, &humidity_to_location);
            //println!("location: {location:?}");

            location.iter().map(|x| x.0).min().unwrap()
        })
        .min()
        .unwrap();

    println!("{min_location}");

    Ok(())
}
