use debug_print::debug_println;
use itertools::Itertools;
use nom::combinator::{eof, map_res};
use std::collections::{HashMap, VecDeque};
use std::io;
use std::{fs, path::Path};

use anyhow::Error;
use nom::character::complete::*;
use nom::{bytes::complete::tag, multi::*, sequence::*, IResult};

#[derive(Debug)]
struct Data {
    modules: Vec<Module>,
}

#[derive(Debug, Clone)]
enum Module {
    Broadcaster(Vec<String>),
    FlipFlop(String, Vec<String>, bool),
    Conjunction(String, Vec<String>, HashMap<String, SignalValue>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SignalValue {
    High,
    Low,
}

#[derive(Debug, Clone)]
struct Pulse(String, String, SignalValue);

fn parse_data(i: &str) -> IResult<&str, Data> {
    let output_parser = || separated_list1(tuple((tag(","), space0::<&str, _>)), alpha1);

    let broadcaster_parser = map_res(
        terminated(
            preceded(
                tuple((tag("broadcaster"), space0, tag("->"), space0)),
                output_parser(),
            ),
            multispace0,
        ),
        |res| {
            Ok::<Module, Error>(Module::Broadcaster(
                res.iter().map(|x| x.to_string()).collect(),
            ))
        },
    );

    let flip_flop_parser = map_res(
        terminated(
            tuple((
                preceded(tag("%"), alpha1),
                preceded(tuple((space0, tag("->"), space0)), output_parser()),
            )),
            multispace0,
        ),
        |(name, res)| {
            Ok::<Module, Error>(Module::FlipFlop(
                name.to_string(),
                res.iter().map(|x| x.to_string()).collect(),
                false,
            ))
        },
    );

    let conjunction_parser = map_res(
        terminated(
            tuple((
                preceded(tag("&"), alpha1),
                preceded(tuple((space0, tag("->"), space0)), output_parser()),
            )),
            multispace0,
        ),
        |(name, res)| {
            Ok::<Module, Error>(Module::Conjunction(
                name.to_string(),
                res.iter().map(|x| x.to_string()).collect(),
                HashMap::new(),
            ))
        },
    );

    let mut parser = terminated(
        many1(nom::branch::alt((
            broadcaster_parser,
            flip_flop_parser,
            conjunction_parser,
        ))),
        eof,
    );

    let (i, modules) = parser(i)?;

    let data = Data { modules };
    Ok((i, data))
}

fn press_button(mod_map: &HashMap<String, Module>) -> Vec<Pulse> {
    let Module::Broadcaster(outputs) = mod_map.get("broadcaster").unwrap() else {
        panic!("can't retrieve broadcaster outputs");
    };

    outputs
        .iter()
        .map(|dest| Pulse("broadcaster".to_string(), dest.clone(), SignalValue::Low))
        .collect_vec()
}

fn main() -> Result<(), Error> {
    let file = Path::new("data/d20p1.txt");
    let contents = fs::read_to_string(file)?;
    let data = parse_data(&contents);
    let data = data.map_err(|err| err.map_input(|s| s.to_string()))?;
    assert!(data.0 == "");
    let data = data.1;

    let mut mod_map = {
        let mut m = HashMap::<String, Module>::new();
        for module in data.modules.iter() {
            match module {
                Module::Broadcaster(_) => {
                    m.insert("broadcaster".to_string(), module.clone());
                }
                Module::FlipFlop(n, ..) => {
                    m.insert(n.to_string(), module.clone());
                }
                Module::Conjunction(n, ..) => {
                    m.insert(n.to_string(), module.clone());
                }
            };
        }
        m
    };

    // reset conjunctions
    for module in data.modules.iter() {
        match module {
            Module::Broadcaster(out_names) => {
                for n in out_names.iter() {
                    if let Some(Module::Conjunction(_, _, input_map)) = mod_map.get_mut(n) {
                        input_map
                            .entry("broadcaster".to_string())
                            .and_modify(|v| *v = SignalValue::Low)
                            .or_insert(SignalValue::Low);
                    }
                }
            }
            Module::FlipFlop(in_name, out_names, _) => {
                for n in out_names.iter() {
                    if let Some(Module::Conjunction(_, _, input_map)) = mod_map.get_mut(n) {
                        input_map
                            .entry(in_name.clone())
                            .and_modify(|v| *v = SignalValue::Low)
                            .or_insert(SignalValue::Low);
                    }
                }
            }
            Module::Conjunction(in_name, out_names, _) => {
                for n in out_names.iter() {
                    if let Some(Module::Conjunction(_, _, input_map)) = mod_map.get_mut(n) {
                        input_map
                            .entry(in_name.clone())
                            .and_modify(|v| *v = SignalValue::Low)
                            .or_insert(SignalValue::Low);
                    }
                }
            }
        };
    }

    // identify input to rx

    let Some(Module::Conjunction(_, _, target_outputs)) = mod_map.values().find(|v| match v {
        Module::Broadcaster(_) => false,
        Module::FlipFlop(_, _, _) => false,
        Module::Conjunction(_, outputs, _) => outputs.iter().any(|v| v == "rx"),
    }) else {
        panic!("can't find target outputs for rx");
    };
    let mut target_outputs = target_outputs.keys().map(|k| (k.clone(), 0)).collect_vec();

    debug_println!("target_output: {target_outputs:?}");

    let mut button_presses = 0;
    let mut done = false;

    let mut queue = VecDeque::new();

    loop {
        button_presses += 1;
        queue.extend(press_button(&mod_map));
        while let Some(pulse) = queue.pop_front() {
            // watch for each target value to go low
            if pulse.2 == SignalValue::Low
                && target_outputs
                    .iter()
                    .position(|(n, v)| *n == pulse.1 && *v == 0)
                    .is_some()
            {
                // update here
                let v = target_outputs
                    .iter_mut()
                    .find(|(n, _)| *n == pulse.1)
                    .unwrap();
                v.1 = button_presses;
            }

            if target_outputs.iter().all(|(_, v)| *v != 0) {
                done = true;
            }

            if mod_map.get(&pulse.1).is_none() {
                // unconnected outputs
                continue;
            }

            match mod_map.get_mut(&pulse.1).unwrap() {
                Module::Broadcaster(_) => unreachable!(),
                Module::FlipFlop(_, outputs, state) => {
                    if pulse.2 == SignalValue::Low {
                        *state = !*state;

                        outputs.iter().for_each(|out| {
                            if *state == true {
                                queue.push_back(Pulse(
                                    pulse.1.clone(),
                                    out.clone(),
                                    SignalValue::High,
                                ));
                            } else {
                                queue.push_back(Pulse(
                                    pulse.1.clone(),
                                    out.clone(),
                                    SignalValue::Low,
                                ));
                            }
                        });
                    }
                }
                Module::Conjunction(_, outputs, input_map) => {
                    input_map
                        .entry(pulse.0)
                        .and_modify(|v| *v = pulse.2.clone())
                        .or_insert(pulse.2.clone());
                    outputs.iter().for_each(|out| {
                        if input_map.values().all(|v| *v == SignalValue::High) {
                            queue.push_back(Pulse(pulse.1.clone(), out.clone(), SignalValue::Low));
                        } else {
                            queue.push_back(Pulse(pulse.1.clone(), out.clone(), SignalValue::High));
                        }
                    });
                }
            }
        }

        if done == true {
            break;
        }
    }

    let result = target_outputs
        .iter()
        .map(|(_, v)| *v as i64)
        .reduce(|acc, v| acc * v)
        .unwrap();

    println!("{result}");

    Ok(())
}
