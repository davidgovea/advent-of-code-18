use std::ops::{Range, RangeInclusive};

fn get_sorted_ranges(line: &str) -> [RangeInclusive<u32>; 2] {
    let mut ranges = line
        .split(',')
        .map(|elf| {
            let [lower, upper]: [u32; 2] = elf
                .split('-')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .ok()
                .unwrap();

            lower..=upper
        })
        .collect::<Vec<_>>();

    ranges.sort_by(|a, b| a.start().cmp(&b.start()).then(b.end().cmp(&a.end())));

    ranges.try_into().ok().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut contained_pairs = 0;

    for line in input.lines() {
        let [elf1, elf2] = get_sorted_ranges(line);

        if elf1.contains(&elf2.start()) && elf1.contains(&(elf2.end())) {
            contained_pairs += 1;
        }
    }
    Some(contained_pairs)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut overlapping_pairs = 0;

    for line in input.lines() {
        let [elf1, elf2] = get_sorted_ranges(line);

        if elf1.contains(&elf2.start()) {
            overlapping_pairs += 1;
        }
    }
    Some(overlapping_pairs)
}

fn main() {
    let input = &aoc2022::read_file("inputs", 4);
    aoc2022::solve!(1, part_one, input);
    aoc2022::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc2022::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc2022::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
