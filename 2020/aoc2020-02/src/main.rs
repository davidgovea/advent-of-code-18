#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Advent of Code 2020 -- Day 1 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn valid_password_entry(input: &str) -> Result<bool, Box<dyn std::error::Error>> {
    lazy_static! {
        static ref PASSWORD_MATCHER: Regex =
            Regex::new(r"(?P<min>\d+)-(?P<max>\d+)\s+(?P<char>[a-z]):\s(?P<password>.*)").unwrap();
    }
    let data = PASSWORD_MATCHER.captures(input).unwrap();
    let min = &data["min"].parse::<usize>().unwrap();
    let max = &data["max"].parse::<usize>().unwrap();
    let c = &data["char"].chars().last().unwrap();
    let char_count = &data["password"]
        .chars()
        .fold(0, |sum, observed_c| match &observed_c == c {
            true => sum + 1,
            false => sum,
        });
    return Ok(char_count >= min && char_count <= max);
}

fn valid_password_entry_2(input: &str) -> Result<bool, Box<dyn std::error::Error>> {
    lazy_static! {
        static ref PASSWORD_MATCHER: Regex =
            Regex::new(r"(?P<pos1>\d+)-(?P<pos2>\d+)\s+(?P<char>[a-z]):\s(?P<password>.*)").unwrap();
    }
    let data = PASSWORD_MATCHER.captures(input).unwrap();
    let pos1 = &data["pos1"].parse::<usize>().unwrap() - 1;
    let pos2 = &data["pos2"].parse::<usize>().unwrap() - 1;
    let c = &data["char"].as_bytes()[0];
    let pass = &data["password"].as_bytes();

    let pos1_set = pass.get(pos1).unwrap_or(&0) == c;
    let pos2_set = pass.get(pos2).unwrap_or(&0) == c;
    println!("{} {} {} {:?}\n{}", pos1, pos2, c, pass, pos1_set ^ pos2_set);

    return Ok(pos1_set ^ pos2_set);
}

fn part1(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = input
        .lines()
        .fold(0, |valid_count, l| match valid_password_entry(l) {
            Ok(true) => valid_count + 1,
            _ => valid_count,
        });
    writeln!(io::stdout(), "result {:?}", result)?;
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = input
        .lines()
        .fold(0, |valid_count, l| match valid_password_entry_2(l) {
            Ok(true) => valid_count + 1,
            _ => valid_count,
        });
    writeln!(io::stdout(), "result {:?}", result)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validity() {
        assert_eq!(valid_password_entry("1-3 a: abcde").unwrap(), true);
        assert_eq!(valid_password_entry("1-3 b: cdefg").unwrap(), false);
        assert_eq!(valid_password_entry("2-9 c: ccccccccc").unwrap(), true);
    }

    #[test]
    fn test_validity2() {
        assert_eq!(valid_password_entry_2("1-3 a: abcde").unwrap(), true);
        assert_eq!(valid_password_entry_2("1-3 b: cdefg").unwrap(), false);
        assert_eq!(valid_password_entry_2("2-9 c: ccccccccc").unwrap(), false);
    }
}