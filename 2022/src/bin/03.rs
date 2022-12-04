use std::{collections::HashSet};

fn get_priority(item: &char) -> u32 {
    match *item as u32 {
        lower if lower >= 97 => lower - 96, // a-z maps to 1-26
        upper if upper >= 65 => upper - 38, // A-Z maps to 27-52
        _ => 0,                             // other characters map to 0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let priorities = input.lines().map(|rucksack| {
        let compartment_size = rucksack.len() / 2;

        // Split string into 2 HashSets at midpoint
        let (compartment1, compartment2) = rucksack.chars().enumerate().fold(
            (HashSet::new(), HashSet::new()),
            |(mut c1, mut c2), (index, item)| {
                match index < compartment_size {
                    true => c1.insert(item),
                    false => c2.insert(item),
                };
                (c1, c2)
            },
        );

        // Find the (first) item present in both: problem states we will always have 1 only
        let error_item = compartment1.intersection(&compartment2).next().unwrap();

        // Transform ascii values to desired ranges
        get_priority(error_item)
    });

    Some(priorities.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let all_rucksacks = input.lines().collect::<Vec<&str>>();
    let groups = all_rucksacks.chunks(3);

    let badges = groups.map(|g| {
        let shared_items = g
            .into_iter()
            .map(|g| -> HashSet<char> { g.chars().collect() })
            .fold(None, |set_opt, next_set| match set_opt {
                None => Some(next_set),
                Some(set) => Some(set.intersection(&next_set).cloned().collect()),
            })
            .unwrap();
        let badge_char = shared_items.iter().next().unwrap(); // Assume always 1

        get_priority(badge_char)
    });

    Some(badges.sum())
}

fn main() {
    let input = &aoc2022::read_file("inputs", 3);
    aoc2022::solve!(1, part_one, input);
    aoc2022::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc2022::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = aoc2022::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
