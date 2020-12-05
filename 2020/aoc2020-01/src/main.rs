use std::io::{self, Read, Write};

fn main() -> Result<(), Box<std::error::Error>> {
    println!("-- Advent of Code 2020 -- Day 1 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<(), Box<std::error::Error>> {
    let numbers = input
        .lines()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut nums = (0, 0);

    'search: for i in 0..numbers.len() {
        nums.0 = numbers[i];
        for j in i..numbers.len() {
            if nums.0 + numbers[j] == 2020 {
                nums.1 = numbers[j];
                break 'search;
            }
        }
    }

    let result = nums.0 * nums.1;

    writeln!(io::stdout(), "result {:?}", result)?;
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<std::error::Error>> {
    let numbers = input
        .lines()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut nums = (0, 0, 0);

    'search: for i in 0..numbers.len() {
        nums.0 = numbers[i];
        for j in i..numbers.len() {
            nums.1 = numbers[j];
            for k in j..numbers.len() {
                if nums.0 + nums.1 + numbers[k] == 2020 {
                    nums.2 = numbers[k];
                    break 'search;
                }
            }
        }
    }

    let result = nums.0 * nums.1 * nums.2;

    writeln!(io::stdout(), "result {:?}", result)?;
    Ok(())
}
