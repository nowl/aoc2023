use std::{
    collections::{HashMap, HashSet},
    io::LineWriter,
    path::Path,
    str::FromStr,
};

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
    fn wins(&self) -> usize {
        self.wins.intersection(&self.guesses).count()
    }
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d4p1.txt");
    let lines = read_as_lines(file)?;

    let cards = lines
        .iter()
        .map(|s| s.parse::<Card>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut totals = HashMap::<i32, u32>::with_capacity(cards.len());
    let max_card_id = cards.iter().map(|c| c.id).max().unwrap();

    cards.iter().for_each(|c| {
        totals.insert(c.id, 1);
    });

    for card in cards {
        let id = card.id;
        // skip last card
        if id == max_card_id {
            continue;
        }
        let wins = card.wins();
        let start = (id + 1).min(max_card_id);
        let end = (id + wins as i32).min(max_card_id);
        let copies = totals[&id];
        //println!("begin for {id} {totals:?}");
        for n in start..=end {
            totals
                .entry(n)
                .and_modify(|e| {
                    *e += copies;
                })
                .or_insert(1);

            //println!("{totals:?}");
        }
        //println!("end for {id} {totals:?}");
    }

    let copies = totals.values().sum::<u32>();

    println!("{copies}");

    Ok(())
}
