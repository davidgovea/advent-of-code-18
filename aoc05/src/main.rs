use std::io::{self, Read, Write};

fn main() -> Result<(), Box<std::error::Error>> {
    println!("-- Advent of Code 2018 -- Day 5 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn reverse_polarity(c: char) -> String {
    match c.is_uppercase() {
        true => c.to_lowercase().to_string(),
        false => c.to_uppercase().to_string()
    }
}

fn part1(input: &str) -> Result<(), Box<std::error::Error>> {

    let mut final_reduction = input.trim().chars().collect::<Vec<_>>();
    let starting_length = final_reduction.len();

    let mut search_index = 0;
    loop {
        let is_match = match (final_reduction.get(search_index), final_reduction.get(search_index + 1)) {
            (Some(c1), Some(c2)) => reverse_polarity(*c1) == c2.to_string(),
            _ => break,
        };

        match is_match {
            true => {
                final_reduction.remove(search_index + 1);
                final_reduction.remove(search_index);
                if search_index > 0 {
                    search_index -= 1;
                };
            },
            false => {
                search_index += 1;
            }
        }
    }

    let final_polymer = final_reduction.iter().collect::<String>();
    println!("final {}", final_polymer);

    writeln!(io::stdout(), "polymer reduction complete! initial length: {} - reduced to {}", starting_length, final_polymer.len())?;
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<std::error::Error>> {

    // writeln!(io::stdout(), "result {:?}", ())?;
    Ok(())
}

