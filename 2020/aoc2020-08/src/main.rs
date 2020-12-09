use itertools::interleave;
use std::collections::HashMap;
use std::io::{self, Read, Write};

#[derive(Clone, Copy, Debug)]
enum Instruction {
    ACC,
    JMP,
    NOP,
}

struct HandheldVM {
    pub executable_memory: HandheldProgram,
    pub accumulator: isize,
    instruction_pointer: usize,
}

type HandheldProgram = Vec<(Instruction, isize)>;

fn parse_handheld_program(input: &str) -> HandheldProgram {
    input
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
        .collect::<Vec<_>>()
}

impl HandheldVM {
    pub fn new(executable_memory: Vec<(Instruction, isize)>) -> Self {
        Self {
            executable_memory,
            accumulator: 0,
            instruction_pointer: 0,
        }
    }
    pub fn from(input: &str) -> Self {
        Self::new(parse_handheld_program(input))
    }
    pub fn run(&mut self) -> Result<isize, Box<dyn std::error::Error>> {
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
                    // println!("LOOP DETECTED! Current ACC: {}", self.accumulator);
                    return Err("LOOP DETECTED".into());
                }
            }

            match self.next() {
                Some(Ok(_)) => {}
                Some(Err(e)) => return Err(e),
                None => break,
            }
        }

        Ok(self.accumulator)
    }
}

impl Iterator for HandheldVM {
    type Item = Result<(), Box<dyn std::error::Error>>;
    fn next(&mut self) -> Option<Self::Item> {
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
                self.instruction_pointer = ((self.instruction_pointer as isize) + value) as usize;
            }
        }
        let halt_pointer = self.executable_memory.len();
        match self.instruction_pointer {
            p if p == halt_pointer => None,
            p if p > 0 && p < halt_pointer => Some(Ok(())),
            _ => Some(Err("OUT OF BOUNDS".into())),
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
    vm.run().unwrap_err();

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let corrupted_program = parse_handheld_program(input);
    let mut final_acc = 0;

    type InstructionReplacements = Vec<(usize, Instruction)>;
    // Scan through program once and build all possible replacements
    let (nops, jumps): (InstructionReplacements, InstructionReplacements) =
    corrupted_program.iter().enumerate().fold(
            (Vec::new(), Vec::new()),
            |(mut nops, mut jumps), (index, operation)| {
                match operation {
                    (Instruction::JMP, _) => {
                        jumps.push((index, Instruction::NOP));
                    }
                    (Instruction::NOP, _) => {
                        nops.push((index, Instruction::JMP));
                    }
                    _ => {}
                };
                (nops, jumps)
            },
        );

        println!("nops: {:?}", nops);
        println!("jmps: {:?}", jumps);

    // Iterate (backwards) through both replacement types and generate new program memory
    let repair_candidate_iter = interleave(jumps.iter().rev(), nops.iter().rev()).map(|(index, new_instruction)| {
        let mut new_program = corrupted_program.clone();
        let (_old_instruction, value) = new_program[*index];
        // println!("Attempting to repair program. Replacing {:?} with {:?} at position {}", old_instruction, new_instruction, index);
        new_program[*index] = (*new_instruction, value);
        new_program
    });

    for (_i, candidate_program) in repair_candidate_iter.enumerate() {
        // println!("Repair attempt {}", i);
        match HandheldVM::new(candidate_program).run() {
            Ok(acc) => {
                final_acc = acc;
                break;
            }
            Err(_) => {}
        }
    }

    writeln!(io::stdout(), "result {:?}", final_acc)?;

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

    static MOCK_VALID: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
acc +6";

    #[test]
    fn test_loop() {
        let mut vm = HandheldVM::from(MOCK_DATA);
        let result = vm.run();
        assert_eq!(result.unwrap_err().to_string(), "LOOP DETECTED");
    }

    #[test]
    fn test_valid() {
        let mut vm = HandheldVM::from(MOCK_VALID);
        let result = vm.run();
        assert_eq!(result.unwrap(), 8);
    }
}
