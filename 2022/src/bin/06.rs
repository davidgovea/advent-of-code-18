use std::collections::HashSet;

fn find_first_preamble_index(input: &str, size: usize) -> Option<u32> {
    for (i, candidate) in input.chars().collect::<Vec<_>>().windows(size).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(candidate.iter());
        if set.len() == size {
            return Some((i + size) as u32);
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    find_first_preamble_index(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_first_preamble_index(input, 14)
}

fn main() {
    let input = &aoc2022::read_file("inputs", 6);
    aoc2022::solve!(1, part_one, input);
    aoc2022::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc2022::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = aoc2022::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(26));
    }
}
