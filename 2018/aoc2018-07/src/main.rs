#[macro_use] extern crate lazy_static;
use std::io::{self, Read, Write};
use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

struct BinaryDigitIterator {
    digits: u32,
    max_digits: u32,
}

impl Iterator for BinaryDigitIterator {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let value = 1 << self.digits;
        match self.digits < self.max_digits {
            true => {
                self.digits += 1;
                Some(value)
            },
            false => None
        }
    }
}

fn binary_digit_iterator(digits: u32) -> BinaryDigitIterator {
    BinaryDigitIterator { digits: 0, max_digits: digits }
}

fn part1(input: &Vec<Prerequisite>) -> Result<(), Box<dyn std::error::Error>> {

    let compound_requirements: HashMap<u32, u32> = input.iter()
        .fold(HashMap::new(), |mut map, prereq| {
            *map.entry(prereq.step).or_default() |= prereq.requires;
            map
        });

    let all_steps = compound_requirements.keys().fold(0, |all, step| all | step);
    let all_requirements = compound_requirements.values().fold(0, |all, req| all | req);

    let known_steps = all_requirements | all_steps;
    let step_count = (known_steps as f32).log2() as u32;

    let mut current_state = 0;
    let step_list2 = (0..step_count)
        .map(|_n| {
            for (index, step) in binary_digit_iterator(step_count).enumerate() {
                if step & (1 << index) == 0 {
                    // No mention of this step..
                    continue;
                }
                let num = 1 << index;
                if current_state & num == num {
                    // This step already performed
                    continue;
                }
                let requirements = compound_requirements.get(&num).unwrap_or(&0);
                if requirements & current_state == *requirements {
                    // Requirements are met - perform!
                    current_state |= num;
                    return num;
                }
            }
            panic!()
        })
        .collect::<Vec<_>>();

    writeln!(io::stdout(), "steps in order: {}", step_list2.iter().map(to_char).collect::<String>())?;
    Ok(())
}

fn part2(input: &Vec<Prerequisite>) -> Result<(), Box<dyn std::error::Error>> {

    writeln!(io::stdout(), "result {:?}", ())?;
    Ok(())
}

