// #[macro_use]
// extern crate lazy_static;
// use regex::Regex;
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Advent of Code 2020 -- Day 9 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

// fn parse_input(input: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let lines = input.lines().collect::<Vec<_>>();

//     Ok(())
// }

struct XmasMessage {
    data: Vec<isize>,
    preamble_size: usize,
}

impl XmasMessage {
    pub fn new(data: Vec<isize>, preamble_size: usize) -> Self {
        Self {
            data,
            preamble_size,
        }
    }
    pub fn from(input: &str, preamble_size: usize) -> Self {
        Self::new(
            input
                .lines()
                .map(|l| l.parse::<isize>().unwrap())
                .collect::<Vec<_>>(),
            preamble_size,
        )
    }
    pub fn iter(&self) -> XmasMessageIter {
        XmasMessageIter {
            data: &self.data,
            preamble_size: self.preamble_size,
            index: self.preamble_size,
        }
    }
    fn iter_invalid(&self) -> impl Iterator<Item = isize> + '_ {
        let iter = XmasMessageIter {
            data: &self.data,
            preamble_size: self.preamble_size,
            index: self.preamble_size,
        };

        iter.filter(|(data, preamble)| !check_is_sum(*data, preamble))
            .map(|(data, _)| data)
    }
    // fn iter_sum_range(&self, target_sum: isize) -> impl Iterator<Item = Vec<isize>> {
    //     let iter = XmasMessageIter {
    //         data: &self.data,
    //         preamble_size: self.preamble_size,
    //         index: self.preamble_size,
    //     };

    //     iter.filter(|(data, preamble)| !check_is_sum(*data, preamble))
    //         .map(|(data, _)| data)
    // }
    // pub fn iter_invalid(&self) ->  {
    //     let iter = XmasMessageIter {
    //         data: &self.data,
    //         preamble_size: self.preamble_size,
    //         index: self.preamble_size,
    //     };

    //     iter.filter(|(data, preamble)| check_is_sum(*data, preamble))
    // }
}

struct XmasMessageIter<'a> {
    data: &'a Vec<isize>,
    preamble_size: usize,
    index: usize,
}

impl Iterator for XmasMessageIter<'_> {
    type Item = (isize, Vec<isize>);
    fn next(&mut self) -> Option<Self::Item> {
        let starting_index = self.index - self.preamble_size;
        let preamble = self.data[starting_index..self.index].to_vec();
        match self.data.get(self.index) {
            None => None,
            Some(item) => {
                self.index += 1;
                Some((*item, preamble))
            }
        }
    }
}

fn check_is_sum(target: isize, numbers: &Vec<isize>) -> bool {
    for i in 0..(numbers.len() - 1) {
        for j in (i + 1)..numbers.len() {
            if target == numbers[i] + numbers[j] {
                return true;
            }
        }
    }

    false
}
fn part1(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let message = XmasMessage::from(input, 25);
    let mut iter_invalid = message.iter_invalid();

    let first_invalid = iter_invalid.next().unwrap();

    writeln!(io::stdout(), "First invalid number: {:?}", first_invalid)?;

    Ok(())
}

fn find_sum_range(message: &XmasMessage, target_sum: isize) -> Option<Vec<isize>> {
    for i in 0..(message.data.len()-1) {
        for j in (i + 1)..message.data.len() {
            match message.data[i..=j].iter().sum::<isize>() {
                x if x == target_sum => {
                    return Some(message.data[i..=j].to_vec());
                },
                x if x > target_sum => {
                    break
                }
                _ => {}
            }
    
        }
    }
    None
}

fn compute_weakness(message: &XmasMessage) -> isize {
    let mut iter_invalid = message.iter_invalid();

    let first_invalid = iter_invalid.next().unwrap();

    let mut range = find_sum_range(&message, first_invalid).unwrap();
    range.sort();

    let weakness = range.first().unwrap() + range.last().unwrap();
    weakness
}

fn part2(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let message = XmasMessage::from(input, 25);
    let sum = compute_weakness(&message);

    writeln!(io::stdout(), "Sum of min/max of range sum: {:?}", sum)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static MOCK_DATA: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_find_invalid() {
        let message = XmasMessage::from(MOCK_DATA, 5);
        let mut invalid_iter = message.iter_invalid();

        let first_invalid = invalid_iter.next().unwrap();

        assert_eq!(first_invalid, 127);
    }

    #[test]
    fn test_compute_weakness() {
        let message = XmasMessage::from(MOCK_DATA, 5);

        assert_eq!(compute_weakness(&message), 62);
    }
}
