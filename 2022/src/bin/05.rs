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

fn perform_move(count: usize, from: usize, to: usize, layout: &mut Layout) -> () {
    let mut moved_values: Vec<char>;
    {
        let from_stack = layout.get_mut(&from).unwrap();
        moved_values = from_stack.drain(from_stack.len() - count..).rev().collect();
    }

    let to_stack = layout.get_mut(&to).unwrap();
    to_stack.append(&mut moved_values);
}

fn parse_instruction(line: &str) -> [usize; 3] {
    let words = line.split_whitespace().collect::<Vec<_>>();
    [words.get(1), words.get(3), words.get(5)].map(|d| d.unwrap().parse().unwrap())
}

fn print_top_crates(layout: Layout) -> String {
    (1..=layout.len())
        .map(|c| String::from(*layout.get(&c).unwrap().last().unwrap()))
        .collect::<Vec<_>>()
        .join("")
}

pub fn part_one(input: &str) -> Option<String> {
    let [initial_layout, instructions] = parse_input(input);
    let mut layout = parse_layout(initial_layout);

    for instruction in instructions.lines() {
        let [count, from, to] = parse_instruction(instruction);

        perform_move(count, from, to, &mut layout);
    }

    Some(print_top_crates(layout))
}

fn perform_move_over9000(count: usize, from: usize, to: usize, layout: &mut Layout) -> () {
    let mut moved_values: Vec<char>;
    {
        let from_stack = layout.get_mut(&from).unwrap();
        moved_values = from_stack.drain(from_stack.len() - count..).collect();
    }

    let to_stack = layout.get_mut(&to).unwrap();
    to_stack.append(&mut moved_values);
}

pub fn part_two(input: &str) -> Option<String> {
    let [initial_layout, instructions] = parse_input(input);
    let mut layout = parse_layout(initial_layout);

    for instruction in instructions.lines() {
        let [count, from, to] = parse_instruction(instruction);

        perform_move_over9000(count, from, to, &mut layout);
    }

    Some(print_top_crates(layout))
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
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
