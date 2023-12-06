use std::{path::Path, str::FromStr};

use anyhow::{anyhow, Error};
use aoc2023::{cap_name_str, read_as_lines};
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
    fn map_src(&self, x: u64) -> Option<u64> {
        if x < self.src {
            None
        } else {
            let diff = x - self.src;
            if diff <= self.len {
                Some(self.dst + diff)
            } else {
                None
            }
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

fn main() -> Result<(), Error> {
    let file = Path::new("data/d5p1.txt");
    let lines = read_as_lines(file)?;

    let seeds = {
        let cap = REGEX_SEED.captures(lines.iter().next().unwrap()).unwrap();
        let seeds = cap_name_str!(cap, "seeds");
        seeds
            .split(" ")
            .map(|s| s.parse())
            .collect::<Result<Vec<u64>, _>>()?
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
            //println!("seed: {s}");
            let map = seed_to_soil.iter().find_map(|m| m.map_src(s)).unwrap_or(s);
            //println!("soil: {map:?}");
            let map = soil_to_fertilizer
                .iter()
                .find_map(|m| m.map_src(map))
                .unwrap_or(map);
            //println!("fertilizer: {map:?}");
            let map = fertilizer_to_water
                .iter()
                .find_map(|m| m.map_src(map))
                .unwrap_or(map);
            //println!("water: {map:?}");
            let map = water_to_light
                .iter()
                .find_map(|m| m.map_src(map))
                .unwrap_or(map);
            //println!("light: {map:?}");
            let map = light_to_temperature
                .iter()
                .find_map(|m| m.map_src(map))
                .unwrap_or(map);
            //println!("termperature: {map:?}");
            let map = temperature_to_humidity
                .iter()
                .find_map(|m| m.map_src(map))
                .unwrap_or(map);
            //println!("humidity: {map:?}");
            let map = humidity_to_location
                .iter()
                .find_map(|m| m.map_src(map))
                .unwrap_or(map);
            //println!("location: {map:?}");
            map
        })
        .min()
        .unwrap();

    println!("{min_location}");

    Ok(())
}
