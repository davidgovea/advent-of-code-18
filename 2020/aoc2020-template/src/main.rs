// #[macro_use]
// extern crate lazy_static;
// use regex::Regex;
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Advent of Code 2020 -- Day X --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse_input(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lines = input.lines().collect::<Vec<_>>();

    // writeln!(io::stdout(), "result {:?}", result)?;

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lines = input.lines().collect::<Vec<_>>();

    // writeln!(io::stdout(), "result {:?}", result)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static MOCK_DATA: &str = "";

    #[test]
    fn test_foo() {
        assert_eq!(parse_input(MOCK_DATA).unwrap(), "foo");
    }

}