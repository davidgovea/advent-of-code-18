use std::collections::BinaryHeap;

pub fn part_one(input: &str) -> Option<u32> {
    let elf_inventories = input.split("\n\n");
    let mut max_calories = None;

    for elf in elf_inventories {
        let elf_total = elf.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>();

        // Using `map_or` allows the "None-checking" to happen in one operation.
        // Alternately, could be `max_c.is_none() || elf_total > max_c.unwrap()`
        // tbh.. I think that is nicer actually^
        if max_calories.map_or(true, |n| elf_total > n) {
            max_calories = Some(elf_total);
        }
    }

    max_calories
}

pub fn part_two(input: &str) -> Option<u32> {
    let elf_inventories = input.split("\n\n");
    let mut heap = BinaryHeap::with_capacity(3);

    for elf in elf_inventories {
        let elf_total = elf.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>();

        heap.push(elf_total);
    }

    // Sooo.. the BinaryHeap with capacity=3 might have more than 3 elements.
    Some(heap.into_sorted_vec().into_iter().rev().take(3).sum())
}

fn main() {
    let input = &aoc2022::read_file("inputs", 1);
    aoc2022::solve!(1, part_one, input);
    aoc2022::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc2022::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = aoc2022::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
