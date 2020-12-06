use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Advent of Code 2019 -- Day 1 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn calculate_fuel(module_mass: i32) -> i32 {
    (module_mass as f64 / 3.0f64) as i32 - 2
}

fn calculate_fuel_recursive(module_mass: i32) -> i32 {
    match calculate_fuel(module_mass) {
        n if n > 0 => {
            n + calculate_fuel_recursive(n)
        },
        _ => 0
    }
}

fn part1(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let fuel_requirements = input.lines().map(|n| {
        let module_mass = n.parse::<i32>().unwrap();
        calculate_fuel(module_mass)
    });

    let total_fuel = fuel_requirements.sum::<i32>();

    writeln!(io::stdout(), "result {:?}", total_fuel)?;
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let fuel_requirements = input.lines().map(|n| {
        let module_mass = n.parse::<i32>().unwrap();
        calculate_fuel_recursive(module_mass)
    });

    let total_fuel = fuel_requirements.sum::<i32>();

    writeln!(io::stdout(), "result {:?}", total_fuel)?;
    Ok(())
}
