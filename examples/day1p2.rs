use std::path::Path;

use anyhow::Error;
use aoc2023::{read_as_lines, Args, Parser};

fn main() -> Result<(), Error> {
    let file = Path::new("data/d1p1.txt");
    let lines = read_as_lines(file)?;

    //for line in lines {
    //    println!("{line}: {}", getcode(&line));
    //}

    let sum = lines
        .iter()
        .fold(0, |acc, line| acc + getcode(&line) as i32);

    println!("{sum}");

    Ok(())
}

fn parse_textnum(text: &str, s: &str) -> Vec<usize> {
    let mut idxs = vec![];

    if text.len() > s.len() {
        return idxs;
    }
    for n in 0..s.len() - text.len() + 1 {
        if &s[n..n + text.len()] == text {
            idxs.push(n);
        }
    }

    idxs
}

fn parse_textnums(s: &str) -> Vec<(usize, u8)> {
    let options = vec![
        ("one", 1u8),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        //("zero", 0),
    ];

    options
        .iter()
        .flat_map(|&(text, value)| {
            parse_textnum(text, s)
                .iter()
                .map(|&idx| (idx, value))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn min_max_textnum(s: &str) -> (Option<(usize, u8)>, Option<(usize, u8)>) {
    let mut textnums = parse_textnums(s);
    textnums.sort();
    (textnums.first().cloned(), textnums.last().cloned())
}

fn getcode(s: &str) -> u8 {
    let fval = s.chars().enumerate().find_map(|(i, c)| {
        let ascii = c as u8;
        if ascii >= 0x30 && ascii <= 0x39 {
            Some((i, ascii - 0x30))
        } else {
            None
        }
    });

    let lval = s.chars().rev().enumerate().find_map(|(i, c)| {
        let ascii = c as u8;
        if ascii >= 0x30 && ascii <= 0x39 {
            Some((s.len() - i - 1, ascii - 0x30))
        } else {
            None
        }
    });

    let (ftext, ltext) = min_max_textnum(s);

    let v1 = if fval.is_some() && ftext.is_some() {
        let (i1, v1) = fval.unwrap();
        let (i2, v2) = ftext.unwrap();
        if i1 < i2 {
            v1
        } else {
            v2
        }
    } else {
        let (_, v) = fval.unwrap_or_else(|| ftext.unwrap());
        v
    };

    let v2 = if lval.is_some() && ltext.is_some() {
        let (i1, v1) = lval.unwrap();
        let (i2, v2) = ltext.unwrap();
        if i1 > i2 {
            v1
        } else {
            v2
        }
    } else {
        let (_, v) = lval.unwrap_or_else(|| ltext.unwrap());
        v
    };

    v1 * 10u8 + v2
}
