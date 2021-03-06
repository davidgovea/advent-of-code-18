use std::io::{self, Read, Write};
use std::collections::HashMap;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Advent of Code 2018 -- Day 3 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug)]
struct FabricClaim {
    id: String,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

fn get_claims(input: &str) -> Vec<FabricClaim> {
    let extract_data = Regex::new(r"#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)").unwrap();

    input
        .lines()
        .map(|l| {
            let data = extract_data.captures(l).unwrap();
            FabricClaim {
                // Ug this is nasty
                id: data.get(1).unwrap().as_str().to_string(),
                x: data.get(2).unwrap().as_str().parse().unwrap(),
                y: data.get(3).unwrap().as_str().parse().unwrap(),
                w: data.get(4).unwrap().as_str().parse().unwrap(),
                h: data.get(5).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect::<Vec<_>>()
}

fn build_claim_map(claims: &Vec<FabricClaim>) -> HashMap<(u32, u32), u32> {
    let mut squares = HashMap::new();
    
    // easy mode: just count every claim's squares individually, then loop through encountered
    for claim in claims {
        for dx in 0..claim.w {
            for dy in 0..claim.h {
                let key = (claim.x + dx, claim.y + dy);
                squares.insert(key, squares.get(&key).unwrap_or(&0) + 1);
            }
        }
    }

    squares
}

fn part1(input: &str) -> Result<(), Box<dyn std::error::Error>> {

    let claims = get_claims(input);

    let squares = build_claim_map(&claims);

    // Count the number of multiply-claimed cells
    let multi_claim_inches = squares.iter().fold(0, |sum, (_location, claim_count)| {
        match claim_count > &1 {
            true => sum + 1,
            false => sum
        }
    });

    writeln!(io::stdout(), "whoaa, hold on a sec there! {} square inches have been claimed by more than one of you.", multi_claim_inches)?;

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn std::error::Error>> {

    let claims = get_claims(input);

    let squares = build_claim_map(&claims);

    // Loop through claims' cells again, and find the one with no overlap in the squares map
    let unique_claim = claims.iter().find(|claim| {
        for dx in 0..claim.w {
            for dy in 0..claim.h {
                let key = (claim.x + dx, claim.y + dy);
                match squares.get(&key) {
                    Some(n) if n > &1 => {
                        return false;
                    },
                    _ => ()
                }
            }
        }

        true
    }).unwrap();

    writeln!(io::stdout(), "looks like only one of you picked a fully unique piece of fabric. who had claim #{}?", unique_claim.id)?;

    Ok(())
}

