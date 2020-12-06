use std::error::Error;
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn Error>> {
    println!("-- Advent of Code 2019 -- Day 2 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let initial_memory = aoc2019::intcode::parse_intcode_program(input)?;

    let result_1202 = aoc2019::intcode::perform_computation(12, 2, &initial_memory)?;
    writeln!(io::stdout(), "result {:?}", result_1202)?;
    Ok(result_1202)
}

fn discover_noun_and_verb(
    desired_result: i32,
    initial_memory: &Vec<i32>,
) -> Result<(i32, i32), Box<dyn Error>> {
    for noun in 0..100 {
        for verb in 0..100 {
            match aoc2019::intcode::perform_computation(noun, verb, &initial_memory)? {
                result if result == desired_result => {
                    return Ok((noun, verb));
                }
                _ => (),
            };
        }
    }

    Err("Unable to find valid noun and verb inputs".into())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let initial_memory = aoc2019::intcode::parse_intcode_program(input)?;
    let desired_course = 19690720;

    let (noun, verb) = discover_noun_and_verb(desired_course, &initial_memory)?;
    writeln!(
        io::stdout(),
        "Computation complete! Noun: {}, Verb: {}, Final result: {}",
        noun,
        verb,
        100 * noun + verb
    )?;

    Ok(())
}
