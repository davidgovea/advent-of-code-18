use std::io::{self, Read, Write};
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Advent of Code 2018 -- Day 1 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn std::error::Error>> {

    let frequency_shift: i32 = input
        .lines()
        .map(|n| n.parse::<i32>().unwrap())
        .sum();

    writeln!(io::stdout(), "Even without units, I can tell this thingy has shifted by about {:?}!", frequency_shift)?;
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let frequency_shifts = input
        .lines()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut visited: HashSet<i32> = HashSet::new();
    let mut frequency_needle = 0;

    'outer: loop {
        let shifts = frequency_shifts.iter();

        for shift in shifts {
            visited.insert(frequency_needle);
            frequency_needle = frequency_needle + shift;

            if visited.contains(&frequency_needle) {
                break 'outer;
            }
        }
    }

    writeln!(io::stdout(), "The frequency needle paused for a moment.. exactly above it's glow-trail, at frequency {}. I almost missed it!", frequency_needle)?;
    Ok(())
}
