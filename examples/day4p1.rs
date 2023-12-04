use std::{collections::HashSet, path::Path, str::FromStr};

use anyhow::{anyhow, Error};
use aoc2023::{cap_name_parse, cap_name_str, read_as_lines};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_CARD: Regex =
        Regex::new(r"Card\s+(?<card>\d+):(?<wins>.+)\|(?<nums>.+)").unwrap();
}

#[derive(Debug)]
struct Card {
    id: i32,
    wins: HashSet<i32>,
    guesses: HashSet<i32>,
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = REGEX_CARD.captures(s).unwrap();
        let card = cap_name_parse!(caps, "card")?;
        let wins = cap_name_str!(caps, "wins");
        let guesses = cap_name_str!(caps, "nums");

        let wins = wins
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;

        let guesses = guesses
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;

        Ok(Card {
            id: card,
            wins,
            guesses,
        })
    }
}

impl Card {
    fn points(&self) -> u32 {
        let len = self.wins.intersection(&self.guesses).count();
        match len {
            0 => 0,
            n => 2u32.pow(n as u32 - 1),
        }
    }
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d4p1.txt");
    let lines = read_as_lines(file)?;

    let cards = lines
        .iter()
        .map(|s| s.parse::<Card>())
        .collect::<Result<Vec<_>, _>>()?;

    let points = cards.iter().map(|c| c.points()).sum::<u32>();

    println!("{points}");

    Ok(())
}
