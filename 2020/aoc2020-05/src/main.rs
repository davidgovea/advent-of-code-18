// #[macro_use]
// extern crate lazy_static;
// use regex::Regex;
use itertools::Itertools;
use std::collections::BTreeSet;
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Advent of Code 2020 -- Day 5 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn bsp_index(input: &str, max: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let (mut lower, mut upper) = (0, max - 1);
    let mut input_chars = input.chars();
    loop {
        let diff = (upper - lower) as f32 / 2_f32;
        match input_chars.next() {
            Some('F') | Some('L') => {
                upper = upper - diff.ceil() as usize;
            }
            Some('B') | Some('R') => {
                lower = lower + diff.ceil() as usize;
            }
            _ => break,
        }
    }

    Ok(lower)
}

fn parse_seat(input: &str) -> Result<(usize, usize, usize), Box<dyn std::error::Error>> {
    let row = bsp_index(&input[0..7], 128)?;
    let seat = bsp_index(&input[7..10], 8)?;
    Ok((row, seat, row * 8 + seat))
}

fn part1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut highest_seat = 0;
    for line in input.lines() {
        let (_, _, seat_id) = parse_seat(line)?;
        if seat_id > highest_seat {
            highest_seat = seat_id;
        }
    }

    writeln!(io::stdout(), "highest seat id {:?}", highest_seat)?;

    Ok(highest_seat)
}

fn part2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut seats: BTreeSet<usize> = BTreeSet::new();
    for line in input.lines() {
        let (_, _, seat_id) = parse_seat(line)?;
        seats.insert(seat_id);
    }

    let mut my_seat: Option<usize> = None;
    for (s1, s2) in seats.iter().tuple_windows() {
        if *s2 == (*s1 + 2) {
            my_seat = Some(s1 + 1);
            break;
        }
    }

    writeln!(io::stdout(), "my seat {:?}", my_seat.unwrap())?;

    Ok(my_seat.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsp() {
        assert_eq!(bsp_index("FBFBBFF", 128).unwrap(), 44);
        assert_eq!(bsp_index("RLR", 8).unwrap(), 5);
    }

    #[test]
    fn test_seats() {
        assert_eq!(parse_seat("FBFBBFFRLR").unwrap(), (44, 5, 357));
        assert_eq!(parse_seat("BFFFBBFRRR").unwrap(), (70, 7, 567));
        assert_eq!(parse_seat("FFFBBBFRRR").unwrap(), (14, 7, 119));
        assert_eq!(parse_seat("BBFFBBFRLL").unwrap(), (102, 4, 820));
    }
}
