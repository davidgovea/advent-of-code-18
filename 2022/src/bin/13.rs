extern crate serde;
extern crate serde_json;

use std::cmp::Ordering;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(untagged)]
enum Node {
    Number(u64),
    List(Vec<Node>),
}

impl Node {
    fn with_slice<T>(&self, f: impl FnOnce(&[Node]) -> T) -> T {
        match self {
            Self::Number(n) => f(&[Self::Number(*n)]),
            Self::List(l) => f(l),
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Number(l), Self::Number(r)) => l.partial_cmp(&r),
            (l, r) => l.with_slice(|l_as_list| {
                r.with_slice(|r_as_list| {
                    l_as_list
                        .iter()
                        .zip(r_as_list)
                        .map(|(l_el, r_el)| l_el.partial_cmp(r_el))
                        .find(|o| o != &Some(Ordering::Equal))
                        .unwrap_or_else(|| l_as_list.len().partial_cmp(&r_as_list.len()))
                })
            }),
        }
    }
}

fn parse_input(input: &str) -> Vec<(Vec<Node>, Vec<Node>)> {
    input
        .split("\n\n")
        .map(|transmission| {
            let packets = transmission
                .lines()
                .map(|line| serde_json::from_str::<Node>(line).unwrap())
                .map(|node| match node {
                    Node::List(l) => l,
                    _ => unreachable!("Bad parse: should be a top level list"),
                })
                .collect::<Vec<_>>();

            (packets[0].clone(), packets[1].clone())
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let packet_groups = parse_input(input);
    let index_sum = packet_groups
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, (left, right))| match left < right {
            true => acc + i as u32 + 1,
            false => acc,
        });

    Some(index_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc2022::read_file("inputs", 13);
    aoc2022::solve!(1, part_one, input);
    aoc2022::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc2022::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc2022::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
