use debug_print::debug_println;
use itertools::Itertools;
use nom::combinator::eof;
use pathfinding::prelude::dijkstra;
use rand::{thread_rng, Rng};
use std::collections::{HashMap, HashSet, VecDeque};
use std::{fs, path::Path};

use anyhow::Error;
use nom::character::complete::*;
use nom::{multi::*, sequence::*, IResult};

#[derive(Debug)]
struct Data {
    data: Vec<(String, Vec<String>)>,
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let line_parse = terminated(
        tuple((
            terminated(alpha1, tuple((char(':'), space0))),
            separated_list1(space1, alpha1),
        )),
        multispace1,
    );
    let mut parser = terminated(many1(line_parse), eof);
    let (i, data) = parser(i)?;

    let data = data
        .into_iter()
        .map(|(n, v)| {
            (
                n.to_string(),
                v.into_iter().map(|n| n.to_string()).collect(),
            )
        })
        .collect();

    let data = Data { data };
    Ok((i, data))
}

type LinkMap = HashMap<String, HashSet<String>>;

fn connected(
    links: &LinkMap,
    names: &HashSet<String>,
    splits: &Vec<(&String, &String)>,
) -> Vec<usize> {
    let mut unvisited = names.clone();
    let mut linked_groups = vec![];

    loop {
        let val = unvisited.iter().cloned().next();
        let mut linked = vec![];
        match val {
            Some(n) => {
                debug_println!("visiting {n:?}");

                let mut to_visit = VecDeque::new();
                to_visit.push_back(n);

                while let Some(next_to_visit) = to_visit.pop_front() {
                    debug_println!("connected {next_to_visit:?}");
                    if unvisited.contains(&next_to_visit) {
                        unvisited.remove(&next_to_visit);
                        linked.push(next_to_visit.clone());

                        let link_set = links.get(&next_to_visit).unwrap();
                        for link in link_set {
                            if !splits.contains(&(&next_to_visit, link))
                                && !splits.contains(&(link, &next_to_visit))
                            {
                                to_visit.push_back(link.clone());
                            }
                        }
                    }
                }
            }
            None => break,
        }
        linked_groups.push(linked);
    }

    linked_groups.iter().map(|v| v.len()).collect()
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d25p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    debug_println!("{data:?}");

    let mut mappings: LinkMap = HashMap::new();
    let mut names = HashSet::new();

    for (name, links) in data.data {
        names.insert(name.clone());

        links.iter().for_each(|n| {
            names.insert(n.clone());

            mappings
                .entry(n.to_string())
                .and_modify(|v| {
                    v.insert(name.clone());
                })
                .or_insert_with(|| {
                    let mut s = HashSet::new();
                    s.insert(name.clone());
                    s
                });
        });

        mappings
            .entry(name)
            .and_modify(|v| v.extend(links.iter().cloned()))
            .or_insert_with(|| {
                let mut s = HashSet::new();
                s.extend(links.iter().cloned());
                s
            });
    }

    debug_println!("{mappings:?}");
    debug_println!("{names:?}");

    mappings.iter().for_each(|(k, v)| {
        debug_println!("{k:?}: {v:?}");
    });

    // find shortest path between random starting and ending points to identify most visited edges

    let names_vec = names.iter().collect_vec();
    let mut rng = thread_rng();
    let mut visited_edge_count = HashMap::new();

    for _ in 0..1000 {
        let start = {
            let i = rng.gen_range(0..names_vec.len());
            names_vec[i]
        };
        let end = {
            let i = rng.gen_range(0..names_vec.len());
            names_vec[i]
        };

        let path = dijkstra(
            start,
            |n| mappings[n].iter().map(|x| (x.clone(), 1)).collect_vec(),
            |n| n == end,
        );

        if let Some(path) = path {
            for n in path.0.windows(2) {
                visited_edge_count
                    .entry((n[0].clone(), n[1].clone()))
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        }
    }

    // grab top 10 most visited edges and try cutting combinations of 3 of them to identify connectivity

    let most_visited = visited_edge_count
        .iter()
        .sorted_by_key(|(_, &v)| v)
        .rev()
        .take(10)
        .collect_vec();

    for v in most_visited.iter() {
        debug_println!("{:?}", v);
    }

    let mut val_pair = None;

    for splits in most_visited.iter().map(|(k, _)| k).combinations(3) {
        let splits = vec![
            (&splits[0].0, &splits[0].1),
            (&splits[1].0, &splits[1].1),
            (&splits[2].0, &splits[2].1),
        ];
        let vals = connected(&mappings, &names, &splits);

        if vals.len() == 2 {
            debug_println!("{vals:?}");
            val_pair = Some(vals);
            break;
        }
    }

    // find answer
    if let Some(val_pair) = val_pair {
        let answer = val_pair[0] * val_pair[1];
        println!("{}", answer);
    }

    Ok(())
}
