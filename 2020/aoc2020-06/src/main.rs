use aoc2020::input::parse_groups;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Advent of Code 2020 -- Day 6 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let groups = parse_groups(input)?;

    let questions_per_group = groups
        .iter()
        .map(|group| {
            let mut set: HashSet<char> = HashSet::new();
            for line in group {
                for c in line.chars() {
                    set.insert(c);
                }
            }
            set.len()
        })
        .collect::<Vec<_>>();

    let sum = questions_per_group.iter().sum::<usize>();
    writeln!(io::stdout(), "result {:?}", sum)?;

    Ok(sum)
}

fn part2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let groups = parse_groups(input)?;

    let questions_per_group = groups
        .iter()
        .map(|group| {
            let mut map: HashMap<char, usize> = HashMap::new();
            for line in group {
                let mut line_set: HashSet<char> = HashSet::new();

                for c in line.chars() {
                    if !line_set.contains(&c) {
                        map.insert(c, map.get(&c).unwrap_or(&0usize) + 1);
                        line_set.insert(c);
                    }
                }
            }
            map.iter()
                .filter(|(_, count)| **count == group.len())
                .count()
        })
        .collect::<Vec<_>>();

    let sum = questions_per_group.iter().sum::<usize>();
    writeln!(io::stdout(), "result {:?}", sum)?;

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    static MOCK_DATA: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_part1() {
        assert_eq!(part1(MOCK_DATA).unwrap(), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(MOCK_DATA).unwrap(), 6);
    }
}
