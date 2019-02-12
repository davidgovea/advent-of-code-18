#[macro_use] extern crate lazy_static;
use std::io::{self, Read, Write};
use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<(), Box<std::error::Error>> {
    println!("-- Advent of Code 2018 -- Day 7 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    lazy_static! {
        static ref extract_data: Regex = Regex::new(r"Step ([A-Z]).*before step ([A-Z])").unwrap();
    }

    let parsed_input = input.lines().map(|l| {
        let data = extract_data.captures(l).unwrap();
        Prerequisite {
            step: to_binary(data.get(2).unwrap().as_str()),
            requires: to_binary(data.get(1).unwrap().as_str()),
        }
    }).collect::<Vec<_>>();

    part1(&parsed_input)?;
    part2(&parsed_input)?;

    Ok(())
}

fn to_binary(step_letter: &str) -> u32 {
    let offset = step_letter.chars().next().unwrap() as u8 - b'A';
    1 << offset
}

fn to_char(binary_representation: &u32) -> char {
    let offset = (*binary_representation as f32).log2().floor() as u8;
    (b'A' + offset) as char
}

#[derive(Debug)]
struct Prerequisite {
    step: u32,
    requires: u32,
}

fn part1(input: &Vec<Prerequisite>) -> Result<(), Box<std::error::Error>> {

    let mut compound_requirements: HashMap<u32, u32> = HashMap::new();
    let mut all_requirements = 0;
    let mut all_steps = 0;
    for prereq in input {
        *compound_requirements.entry(prereq.step).or_default() |= prereq.requires;
        all_requirements |= prereq.requires;
        all_steps |= prereq.step;
    }

    let known_steps = all_requirements | all_steps;
    let step_count = (known_steps as f32).log2() as usize;
    let mut step_list: Vec<u32> = Vec::new();
    let mut current_state = 0;

    while step_list.len() < step_count {
        let mut shifted_steps = known_steps;
        let mut digit = 0;
        while shifted_steps >= 1 {
            if dec & 1 == 0 {
                // No mention of this step..
                continue;
            }
            let num = 1 << digit;
            match compound_requirements.get(&num) {
                None if current_state & num != num => {
                    step_list.push(num);
                    current_state |= num;
                    break;
                },
                Some(r) if r & current_state == *r => {
                    if num & current_state != num {
                        step_list.push(num);
                        current_state |= num;
                        break;
                    }
                },
                _ => ()
            }
            shifted_steps >>= 1;
            digit += 1;
        }
    }

    writeln!(io::stdout(), "{:#b} current_state\n{:?} list", current_state, step_list.iter().map(to_char).collect::<String>())?;
    Ok(())
}

fn part2(input: &Vec<Prerequisite>) -> Result<(), Box<std::error::Error>> {

    writeln!(io::stdout(), "result {:?}", ())?;
    Ok(())
}

