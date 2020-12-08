use aoc2019::intcode::IntcodeVM;
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Advent of Code 2019 -- Day 4 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut initial_memory = aoc2019::intcode::parse_intcode_program(input)?;

    let mut vm = IntcodeVM::new(&mut initial_memory, Some(|_last_output| 1));
    let outputs = vm.run();

    writeln!(io::stdout(), "outputs: {:?}", outputs)?;

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut initial_memory = aoc2019::intcode::parse_intcode_program(input)?;

    let mut vm = IntcodeVM::new(&mut initial_memory, Some(|_last_output| 5));
    let outputs = vm.run();

    writeln!(io::stdout(), "outputs: {:?}", outputs)?;

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     static MOCK_INPUT_1: &str = "";

//     #[test]
//     fn test_validity() {
//         assert_eq!(is_valid(MOCK_INPUT_1).unwrap(), true);
//         assert_eq!(is_valid(MOCK_INPUT_2).unwrap(), false);
//         assert_eq!(is_valid("652200").unwrap(), false);
//     }
// }
