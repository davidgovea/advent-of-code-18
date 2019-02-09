use std::io::{self, Read, Write};
use std::collections::HashMap;

fn main() -> Result<(), Box<std::error::Error>> {
    println!("-- Advent of Code 2018 -- Day 5 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

// Flips capitalization
fn reverse_polarity(c: char) -> String {
    match c.is_uppercase() {
        true => c.to_lowercase().to_string(),
        false => c.to_uppercase().to_string()
    }
}

fn reduce_polymer(input: &str) -> Result<String, Box<std::error::Error>> {
    let mut final_reduction = input.trim().chars().collect::<Vec<_>>();
    let starting_length = final_reduction.len();

    let mut search_index = 0;
    loop {
        let is_match = match (final_reduction.get(search_index), final_reduction.get(search_index + 1)) {
            (Some(c1), Some(c2)) => reverse_polarity(*c1) == c2.to_string(),
            _ => break,
        };

        // When a match is found, back-track search index by one.
        //  A greedy matcher could be more efficient
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

    Ok(final_reduction.iter().collect::<String>())
}

fn part1(input: &str) -> Result<(), Box<std::error::Error>> {

    let starting_length = input.trim().len();
    let final_polymer = reduce_polymer(input)?;

    writeln!(io::stdout(), "polymer reduction complete! initial length: {} - reduced to {}", starting_length, final_polymer.len())?;
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<std::error::Error>> {

    let starting_length = input.trim().len();
    let base_polymer = reduce_polymer(input)?;

    let mut unit_tests: HashMap<String, usize> = HashMap::new();
    for c in base_polymer.chars() {
        let unit = c.to_lowercase().to_string();
        if !unit_tests.contains_key(&unit) {
            let stripped_polymer = base_polymer.replace(&unit, "").replace(&c.to_uppercase().to_string(), "");
            unit_tests.insert(unit, reduce_polymer(&stripped_polymer)?.len());
        }
    }

    let mut sorted_polymer_tests = unit_tests.iter().collect::<Vec<_>>();
    sorted_polymer_tests.sort_by_key(|t| t.1);
    let (most_effecient_removal, new_length) = sorted_polymer_tests.first().unwrap();

    writeln!(io::stdout(), "removing unit '{}' resulted in the most significant reduction in polymer length: down to {}!", most_effecient_removal, new_length)?;
    Ok(())
}

