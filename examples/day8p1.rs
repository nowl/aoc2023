use std::{collections::HashMap, fs, path::Path};

use anyhow::Error;
use itertools::Itertools;
use nom::{
    character::complete::{alpha1, char, multispace1, space1},
    multi::many1,
    IResult,
};

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Data {
    instructions: String,
    node_map: HashMap<String, Node>,
}

fn parse_node(i: &str) -> IResult<&str, Node> {
    let (i, _) = char('(')(i)?;
    let (i, left) = alpha1(i)?;
    let (i, _) = char(',')(i)?;
    let (i, _) = space1(i)?;
    let (i, right) = alpha1(i)?;
    let (i, _) = char(')')(i)?;

    let node = Node {
        left: left.to_string(),
        right: right.to_string(),
    };
    Ok((i, node))
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let (i, inst) = alpha1(i)?;
    let (i, _) = multispace1(i)?;
    let (i, nodes) = many1(|i| {
        let (i, name) = alpha1(i)?;
        let (i, _) = space1(i)?;
        let (i, _) = char('=')(i)?;
        let (i, _) = space1(i)?;
        let (i, node) = parse_node(i)?;
        let (i, _) = multispace1(i)?;
        Ok((i, (name, node)))
    })(i)?;

    let node_map = {
        let mut map = HashMap::new();
        for (name, node) in nodes {
            map.insert(name.to_string(), node);
        }
        map
    };

    let data = Data {
        instructions: inst.to_string(),
        node_map,
    };

    Ok((i, data))
}

fn traverse(data: &Data) -> u32 {
    let instrs = data.instructions.chars().collect_vec();
    let mut ipos = 0;
    let mut pos = "AAA";
    let mut count = 0;
    loop {
        let inst = instrs[ipos];
        ipos += 1;
        if ipos >= instrs.len() {
            ipos = 0;
        }

        pos = match inst {
            'L' => &data.node_map[pos].left,
            'R' => &data.node_map[pos].right,
            _ => unreachable!(),
        };

        count += 1;

        if pos == "ZZZ" {
            break;
        }
    }
    count
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d8p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    let count = traverse(&data);

    println!("{count:?}");

    Ok(())
}
