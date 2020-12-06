use itertools::Itertools;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Advent of Code 2019 -- Day 4 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn is_valid(input: &str) -> Result<bool, Box<dyn std::error::Error>> {
    if input.len() != 6 {
        return Ok(false);
    }

    let char_iter = input.chars();
    let mut adjacent_repeat = false;
    let mut increasing_digits = true;
    for (digit, next_digit) in char_iter.tuple_windows() {
        if digit == next_digit {
            adjacent_repeat = true;
        }
        if digit > next_digit {
            increasing_digits = false;
        }
    }

    Ok(adjacent_repeat && increasing_digits)
}

fn parse_input(input: &str) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let mut split = input.split('-');
    let min = split.next().unwrap().parse::<usize>()?;
    let max = split.next().unwrap().parse::<usize>()?;
    Ok((min, max))
}

fn naive_search(min: usize, max: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    Ok((min..=max)
        .filter_map(|n| {
            let string_num = n.to_string();
            match is_valid(&string_num) {
                Ok(true) => Some(string_num),
                _ => None,
            }
        })
        .collect::<Vec<_>>())
}

fn part1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let (min, max) = parse_input(input)?;

    let valid = naive_search(min, max)?;

    writeln!(io::stdout(), "result {:?}", valid.len())?;

    Ok(valid.len())
}

fn is_valid2(input: &str) -> Result<bool, Box<dyn std::error::Error>> {
    if input.len() != 6 {
        return Ok(false);
    }

    let char_iter = input.chars();
    let mut adjacent_repeat = false;
    let mut repeated_digit: Option<char> = None;
    let mut increasing_digits = true;
    for (digit, next_digit) in char_iter.tuple_windows() {
        match repeated_digit {
            Some(d) if d == next_digit => {
                adjacent_repeat = false;
            }
            Some(_d) => {
                repeated_digit = None;
            }
            _ => {
                if !adjacent_repeat && (digit == next_digit) {
                    adjacent_repeat = true;
                    repeated_digit = Some(digit);
                }
            }
        }

        if digit > next_digit {
            increasing_digits = false;
        }
    }

    Ok(adjacent_repeat && increasing_digits)
}

fn part2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let (min, max) = parse_input(input)?;

    let valid = (min..=max)
        .filter_map(|n| {
            let string_num = n.to_string();
            match is_valid2(&string_num) {
                Ok(true) => Some(string_num),
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    writeln!(io::stdout(), "result {:?}", valid.len())?;

    Ok(valid.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    static MOCK_INPUT_1: &str = "111111";
    static MOCK_INPUT_2: &str = "223450";

    #[test]
    fn test_validity() {
        assert_eq!(is_valid(MOCK_INPUT_1).unwrap(), true);
        assert_eq!(is_valid(MOCK_INPUT_2).unwrap(), false);
        assert_eq!(is_valid("652200").unwrap(), false);
    }

    #[test]
    fn test_validity2() {
        assert_eq!(is_valid2("112233").unwrap(), true);
        assert_eq!(is_valid2("123444").unwrap(), false);
        assert_eq!(is_valid2("111122").unwrap(), true);
    }
}
