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

    aoc2019::intcode::run_intcode_program(&mut initial_memory)?;

    // writeln!(io::stdout(), "result {:?}", ())?;

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    // let initial_memory = aoc2019::intcode::parse_intcode_program(input)?;

    // let result_1202 = aoc2019::intcode::perform_computation(12, 2, &initial_memory)?;


    // writeln!(io::stdout(), "result {:?}", ())?;

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
