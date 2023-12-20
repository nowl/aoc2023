use debug_print::debug_println;
use itertools::Itertools;
use std::collections::HashMap;
use std::{fs, path::Path};

use anyhow::Error;
use nom::character::complete::*;
use nom::{bytes::complete::tag, multi::*, sequence::*, IResult};

#[derive(Debug)]
struct Data {
    workflows: Vec<Workflow>,
    parts: Vec<Part>,
}

#[derive(Debug)]
enum State {
    Filter(String),
    Accepted,
    Rejected,
}

#[derive(Debug)]
enum Filter {
    CondFilter {
        attr: char,
        op: char,
        amount: i32,
        dest: State,
    },
    DirectFilter(State),
}

impl Filter {
    fn filter_part(&self, part: &Part) -> Option<&State> {
        match self {
            Filter::CondFilter {
                attr,
                op,
                amount,
                dest,
            } => {
                let test = match attr {
                    'x' => part.x - amount,
                    'm' => part.m - amount,
                    'a' => part.a - amount,
                    's' => part.s - amount,
                    _ => unreachable!(),
                };
                match op {
                    '>' if test > 0 => Some(dest),
                    '<' if test < 0 => Some(dest),
                    _ => None,
                }
            }
            Filter::DirectFilter(state) => Some(state),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    filters: Vec<Filter>,
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

fn parse_filter(i: &str) -> IResult<&str, Filter> {
    let (i, name) = alpha1(i)?;

    let cond_filter_parser = nom::sequence::tuple((one_of("><"), digit1, char(':'), alpha1));

    let dst_parse = |v| match v {
        "A" => State::Accepted,
        "R" => State::Rejected,
        other => State::Filter(other.to_string()),
    };

    let (i, cond_filter) = nom::combinator::opt(cond_filter_parser)(i)?;

    let filter = {
        if let Some(result) = cond_filter {
            let dst_name = dst_parse(result.3);
            let amount = result.1.parse::<i32>().unwrap();
            debug_assert_eq!(name.len(), 1);
            Filter::CondFilter {
                attr: name.chars().nth(0).unwrap(),
                op: result.0,
                amount,
                dest: dst_name,
            }
        } else {
            let dst_name = dst_parse(name);
            Filter::DirectFilter(dst_name)
        }
    };

    Ok((i, filter))
}

fn parse_workflow(i: &str) -> IResult<&str, Workflow> {
    let (i, name) = alpha1(i)?;
    let (i, filters) = terminated(
        delimited(
            char('{'),
            separated_list1(tag(","), parse_filter),
            char('}'),
        ),
        line_ending,
    )(i)?;

    let wf = Workflow {
        name: name.to_string(),
        filters,
    };

    Ok((i, wf))
}

fn parse_data(i: &str) -> IResult<&str, Data> {
    let parse_part = terminated(
        delimited(
            char('{'),
            nom::sequence::tuple((
                tag("x="),
                digit1,
                tag(",m="),
                digit1,
                tag(",a="),
                digit1,
                tag(",s="),
                digit1,
            )),
            char('}'),
        ),
        line_ending,
    );

    let (i, workflows) = terminated(many1(parse_workflow), line_ending)(i)?;
    let (i, parts) = terminated(many1(parse_part), multispace0)(i)?;

    let parts = parts
        .into_iter()
        .map(|x| Part {
            x: x.1.parse::<i32>().unwrap(),
            m: x.3.parse::<i32>().unwrap(),
            a: x.5.parse::<i32>().unwrap(),
            s: x.7.parse::<i32>().unwrap(),
        })
        .collect_vec();
    let data = Data { workflows, parts };
    Ok((i, data))
}

fn process_part<'a, 'b>(filters: &'a Vec<Filter>, part: &'b Part) -> &'a State {
    for f in filters.iter() {
        if let Some(state) = f.filter_part(part) {
            return state;
        }
    }

    unreachable!();
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d19p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    debug_println!("data: {data:?}");

    let mut workflow_map = HashMap::new(); //HashMap<&str, Vec<Filter>>
    for workflow in data.workflows {
        workflow_map.insert(workflow.name, workflow.filters);
    }

    debug_println!("{workflow_map:?}");

    let mut total = 0;

    for part in data.parts {
        let mut wf = workflow_map.get("in").unwrap();

        loop {
            let state = process_part(&wf, &part);
            match state {
                State::Filter(other) => wf = workflow_map.get(other).unwrap(),
                State::Accepted => {
                    total += (part.x + part.m + part.a + part.s) as i64;
                    break;
                }
                State::Rejected => break,
            }
        }
    }

    println!("{total}");

    Ok(())
}
