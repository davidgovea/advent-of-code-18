use std::io::{self, Read, Write};

fn main() -> Result<(), Box<std::error::Error>> {
    println!("-- Advent of Code 2018 -- Day 1 --");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let frequency_shift: i32 = input.lines().map(|n| n.parse::<i32>().unwrap()).sum();

    writeln!(io::stdout(), "Even without units, I can tell this thingy has shifted by about {:?}!", frequency_shift)?;

    Ok(())
}
