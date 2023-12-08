use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

use anyhow::Error;
use itertools::Itertools;
use nom::{
    character::complete::{alpha1, char, multispace1, space1},
    multi::many1,
    IResult,
};
use num::Integer;

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

fn find_terminal_states<'a, 'b>(data: &'a Data, ending: &'b str) -> HashSet<&'a str> {
    data.node_map
        .keys()
        .filter(|k| k.ends_with(ending))
        .map(|k| k.as_str())
        .collect()
}

fn traverse(data: &Data, start_state: &str, end_states: &HashSet<&str>) -> u32 {
    let instrs = data.instructions.chars().collect_vec();
    let mut ipos = 0;
    let mut pos = start_state;
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

        if end_states.contains(pos) {
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

    let start_states = find_terminal_states(&data, "A");
    let end_states = find_terminal_states(&data, "Z");

    // counts to reach an end state
    let end_state_counts = start_states
        .iter()
        .map(|start_state| traverse(&data, start_state, &end_states))
        .collect_vec();

    // find the lowest common multiple to get to the goals
    let count = end_state_counts
        .into_iter()
        .map(|x| x as u64)
        .reduce(|a, b| a.lcm(&b))
        .unwrap();

    println!("{count}");

    Ok(())
}
