#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;

type Layout<'a> = HashMap<usize, Vec<char>>;

fn parse_input(input: &str) -> [&str; 2] {
    input
        .split("\n\n")
        .collect::<Vec<_>>()
        .try_into()
        .ok()
        .unwrap()
}

fn parse_layout(layout: &str) -> Layout {
    let mut layout_iter = layout.lines().rev();
    let columns = layout_iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut state = HashMap::new();

    columns.clone().into_iter().for_each(|name| {
        state.insert(name, Vec::new());
    });

    layout_iter.for_each(|row| {
        let chars = row.as_bytes();
        columns
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(i, col_name)| {
                let target_index = i * 4 + 1;
                match chars.get(target_index) {
                    Some(c) if !(*c as char).is_whitespace() => state
                        .entry(col_name)
                        .or_insert_with(Vec::new)
                        .push(*c as char),
                    _ => (),
                };
            });
    });

    state
}

fn perform_move(from: usize, to: usize, count: usize, layout: &mut Layout) -> () {
    let mut moved_values: Vec<char>;
    {
        let from_stack = layout.get_mut(&from).unwrap();
        moved_values = from_stack.drain(from_stack.len() - count..).rev().collect();
    }

    let to_stack = layout.get_mut(&to).unwrap();
    to_stack.append(&mut moved_values);
}

fn run_instructions(instructions: &str, layout: &mut Layout) -> () {
    lazy_static! {
        static ref INSTRUCTION_EXTRACT: Regex =
            Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }

    for instruction in instructions.lines() {
        let [count, from, to]: [usize; 3] = INSTRUCTION_EXTRACT
            .captures(instruction)
            .unwrap()
            .iter()
            .skip(1)
            .map(|d| d.unwrap().as_str().parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .unwrap();

        perform_move(from, to, count, layout);
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let [layout, instructions] = parse_input(input);

    let mut state = parse_layout(layout);

    run_instructions(instructions, &mut state);

    Some(
        (1..=state.len())
            .map(|c| String::from(*state.get(&c).unwrap().last().unwrap()))
            .collect::<Vec<_>>()
            .join(""),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc2022::read_file("inputs", 5);
    aoc2022::solve!(1, part_one, input);
    aoc2022::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc2022::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = aoc2022::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
