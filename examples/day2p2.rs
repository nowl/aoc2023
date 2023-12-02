use std::{path::Path, str::FromStr};

use anyhow::{anyhow, Error};
use aoc2023::read_as_lines;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_GAME: Regex = Regex::new(r"Game\s+(?<id>\d+):(?<trials>.+)").unwrap();
    static ref REGEX_TRIAL: Regex = Regex::new(r"(?<trial>.+?)(?:;|$)").unwrap();
    static ref REGEX_RED: Regex = Regex::new(r"\s*(?<num>\d+)\s*red").unwrap();
    static ref REGEX_GREEN: Regex = Regex::new(r"\s*(?<num>\d+)\s*green").unwrap();
    static ref REGEX_BLUE: Regex = Regex::new(r"\s*(?<num>\d+)\s*blue").unwrap();
}

#[derive(Default, Debug)]
struct Trial {
    red: i32,
    blue: i32,
    green: i32,
}

macro_rules! cap_name_parse {
    ($capture:ident, $name:expr) => {
        $capture
            .name($name)
            .ok_or(anyhow!("regex capture error"))?
            .as_str()
            .parse()
    };
}

macro_rules! cap_name_str {
    ($capture:ident, $name:expr) => {
        $capture
            .name($name)
            .ok_or(anyhow!("regex capture error"))?
            .as_str()
    };
}

impl FromStr for Trial {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let red = match REGEX_RED.captures(s) {
            None => 0,
            Some(cap) => cap_name_parse!(cap, "num")?,
        };

        let green = match REGEX_GREEN.captures(s) {
            None => 0,
            Some(cap) => cap_name_parse!(cap, "num")?,
        };

        let blue = match REGEX_BLUE.captures(s) {
            None => 0,
            Some(cap) => cap_name_parse!(cap, "num")?,
        };

        Ok(Trial { red, green, blue })
    }
}

#[derive(Default, Debug)]
struct Game {
    id: i32,
    trials: Vec<Trial>,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Game::default();

        for cap in REGEX_GAME.captures_iter(s) {
            game.id = cap_name_parse!(cap, "id")?;
            let trials_str = cap_name_str!(cap, "trials");
            game.trials = vec![];
            for cap in REGEX_TRIAL.captures_iter(trials_str) {
                let trial = cap_name_parse!(cap, "trial")?;
                game.trials.push(trial);
            }
        }

        Ok(game)
    }
}

impl Trial {
    fn is_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        self.trials.iter().all(|t| t.is_possible())
    }

    fn min_values(&self) -> Trial {
        macro_rules! min_color {
            ($sel:ident) => {
                self.trials
                    .iter()
                    .max_by_key(|t| t.$sel)
                    .map(|t| t.$sel)
                    .unwrap_or(0)
            };
        }

        Trial {
            red: min_color!(red),
            green: min_color!(green),
            blue: min_color!(blue),
        }
    }
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d2p1.txt");
    let lines = read_as_lines(file)?;

    /*
        for line in lines {
            println!("{line}");
            let game: Game = line.parse()?;
            println!("{game:?} {}", game.is_possible());
    }
     */

    let games = lines
        .iter()
        .map(|line| line.parse())
        .collect::<Result<Vec<Game>, _>>()?;

    let sum = games.iter().fold(0, |acc, g| {
        let mins = g.min_values();
        acc + mins.red * mins.green * mins.blue
    });

    println!("{sum}");

    Ok(())
}
