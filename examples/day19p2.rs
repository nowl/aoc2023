use debug_print::debug_println;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::{fs, path::Path};

use anyhow::Error;
use nom::character::complete::*;
use nom::{bytes::complete::tag, multi::*, sequence::*, IResult};

#[derive(Debug)]
struct Data {
    workflows: Vec<Workflow>,
    parts: Vec<Part>,
}

#[derive(Debug, Clone)]
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

enum FilterResult {
    Single((State, PartRange)),
    // pass, fail
    Double((State, PartRange), PartRange),
}

impl Filter {
    fn filter_part_range(&self, part: &PartRange) -> FilterResult {
        match self {
            Filter::CondFilter {
                attr,
                op,
                amount,
                dest,
            } => {
                macro_rules! split_range {
                    ($v:ident) => {{
                        let greater = *op == '>';
                        if greater {
                            let mut part1 = part.clone();
                            part1.$v.1 = *amount;
                            let mut part2 = part.clone();
                            part2.$v.0 = amount + 1;
                            FilterResult::Double((dest.clone(), part2), part1)
                        } else {
                            let mut part1 = part.clone();
                            part1.$v.1 = amount - 1;
                            let mut part2 = part.clone();
                            part2.$v.0 = *amount;
                            FilterResult::Double((dest.clone(), part1), part2)
                        }
                    }};
                }
                match attr {
                    'x' => split_range!(x),
                    'm' => split_range!(m),
                    'a' => split_range!(a),
                    's' => split_range!(s),
                    _ => unreachable!(),
                }
            }
            Filter::DirectFilter(state) => FilterResult::Single((state.clone(), part.clone())),
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

#[derive(Debug, Clone)]
struct PartRange {
    x: (i32, i32),
    m: (i32, i32),
    a: (i32, i32),
    s: (i32, i32),
}

impl PartRange {
    fn valid(&self) -> bool {
        self.x.0 <= self.x.1 && self.m.0 <= self.m.1 && self.a.0 <= self.a.1 && self.s.0 <= self.s.1
    }

    fn combinations(&self) -> i64 {
        (self.x.1 - self.x.0 + 1) as i64
            * (self.m.1 - self.m.0 + 1) as i64
            * (self.a.1 - self.a.0 + 1) as i64
            * (self.s.1 - self.s.0 + 1) as i64
    }
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

fn process_part_range_to_accepted(
    filters: &Vec<Filter>,
    part_range: &PartRange,
) -> Vec<(State, PartRange)> {
    let mut results = vec![];

    let mut pr = part_range.clone();
    for f in filters.iter() {
        match f.filter_part_range(&pr) {
            FilterResult::Single(r) => {
                if r.1.valid() {
                    results.push(r);
                }
            }
            FilterResult::Double(passed, failed) => {
                if passed.1.valid() {
                    results.push(passed);
                }
                pr = failed;
            }
        }
    }

    results
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d19p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    let mut workflow_map = HashMap::new(); //HashMap<&str, Vec<Filter>>
    for workflow in data.workflows {
        workflow_map.insert(workflow.name, workflow.filters);
    }

    let pr = PartRange {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };

    let mut queue = VecDeque::new();
    let mut accepted = vec![];

    queue.push_back((State::Filter("in".to_string()), pr));

    while let Some((State::Filter(wf_name), pr)) = queue.pop_front() {
        debug_println!("in: {wf_name:?}, {pr:?}");

        let wf = workflow_map.get(&wf_name).unwrap();
        debug_println!("workflow: {wf:?}");

        let results = process_part_range_to_accepted(&wf, &pr);

        results.iter().for_each(|r| {
            debug_println!("{r:?}");
        });

        for r in results {
            match r.0 {
                State::Filter(_) => queue.push_back(r),
                State::Accepted => accepted.push(r.1),
                State::Rejected => (),
            }
        }
    }

    debug_println!("{accepted:?}");

    let total = accepted
        .into_iter()
        .fold(0i64, |acc, pr| acc + pr.combinations());

    println!("{total}");

    Ok(())
}
