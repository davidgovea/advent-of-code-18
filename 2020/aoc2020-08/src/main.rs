// #[macro_use]
// extern crate lazy_static;
// use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read, Write};

enum Instruction {
    ACC,
    JMP,
    NOP,
}

struct HandheldVM {
    pub executable_memory: Vec<(Instruction, isize)>,
    pub accumulator: isize,
    instruction_pointer: usize,
}

impl HandheldVM {
    pub fn from(input: &str) -> Self {
        Self {
            executable_memory: input
                .lines()
                .map(|line| {
                    let mut iter = line.split(" ");
                    let instruction = iter.next().unwrap();
                    let value = iter.next().unwrap().parse::<isize>().unwrap();
                    (
                        match instruction {
                            "acc" => Instruction::ACC,
                            "jmp" => Instruction::JMP,
                            "nop" => Instruction::NOP,
                            _ => panic!("Unknown instruction"),
                        },
                        value,
                    )
                })
                .collect::<Vec<_>>(),
            accumulator: 0,
            instruction_pointer: 0,
        }
    }
    pub fn run(&mut self) -> () {
        let mut instruction_visits: HashMap<usize, usize> = HashMap::new();
        loop {
            let previous_visits = instruction_visits
                .get(&self.instruction_pointer)
                .unwrap_or(&0);
            match previous_visits {
                0 => {
                    instruction_visits.insert(self.instruction_pointer, 1);
                }
                _ => {
                    println!("LOOP DETECTED! Current ACC: {}", self.accumulator);
                    return;
                }
            }

            let (instruction, value) = &self.executable_memory[self.instruction_pointer];
            match instruction {
                Instruction::ACC => {
                    self.accumulator += value;
                    self.instruction_pointer += 1;
                }
                Instruction::NOP => {
                    self.instruction_pointer += 1;
                }
                Instruction::JMP => {
                    self.instruction_pointer =
                        ((self.instruction_pointer as isize) + value) as usize;
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Advent of Code 2020 -- Day 8 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut vm = HandheldVM::from(input);
    vm.run();

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lines = input.lines().collect::<Vec<_>>();

    // writeln!(io::stdout(), "result {:?}", result)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static MOCK_DATA: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_examole() {
        let mut vm = HandheldVM::from(MOCK_DATA);
        vm.run();
        assert_eq!("lol run with --nocapture: print 5", "lol run with --nocapture: print 5");
    }
}
