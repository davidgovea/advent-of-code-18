use std::error::Error;
use std::io::{self, Read, Write};

#[derive(Debug)]
enum OpCode {
    Add,
    Multiply,
    Halt,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("-- Advent of Code 2019 -- Day 1 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse_op_code(input: i32) -> Result<OpCode, Box<dyn Error>> {
    match input {
        1 => Ok(OpCode::Add),
        2 => Ok(OpCode::Multiply),
        99 => Ok(OpCode::Halt),
        _ => Err("Bad opcode".into()),
    }
}

fn resolve_parameters(
    instruction_pointer: usize,
    program_memory: &Vec<i32>,
) -> Result<(i32, i32, usize), Box<dyn Error>> {
    let (operand_1_pointer, operand_2_pointer, dest_pointer) =
        get_parameter_pointers(&program_memory[instruction_pointer + 1..instruction_pointer + 4])?;
    Ok((
        program_memory[operand_1_pointer],
        program_memory[operand_2_pointer],
        dest_pointer,
    ))
}

fn get_parameter_pointers(input: &[i32]) -> Result<(usize, usize, usize), Box<dyn Error>> {
    match input {
        [a, b, c] => Ok(((*a) as usize, (*b) as usize, (*c) as usize)),
        _ => Err("Invalid parameters".into()),
    }
}

fn run_intcode_program(program_memory: &mut Vec<i32>) -> Result<&Vec<i32>, Box<dyn Error>> {
    let mut instruction_pointer = 0;
    loop {
        match parse_op_code(program_memory[instruction_pointer])? {
            OpCode::Halt => break,
            OpCode::Add => {
                let (operand_1, operand_2, dest) =
                    resolve_parameters(instruction_pointer, &program_memory)?;
                program_memory[dest] = operand_1 + operand_2;
            }
            OpCode::Multiply => {
                let (operand_1, operand_2, dest) =
                    resolve_parameters(instruction_pointer, &program_memory)?;
                program_memory[dest] = operand_1 * operand_2;
            }
        }
        instruction_pointer += 4;
    }

    Ok(program_memory)
}

fn parse_intcode_program(input: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    Ok(input
    .split(",")
    .map(|n| n.trim().parse::<i32>().unwrap())
    .collect::<Vec<i32>>())
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut program_memory = parse_intcode_program(input)?;

    // Perform 1202 alarm modifications
    program_memory[1] = 12;
    program_memory[2] = 2;

    run_intcode_program(&mut program_memory)?;

    let result_1202 = program_memory[0];

    writeln!(io::stdout(), "result {:?}", result_1202)?;
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<std::error::Error>> {
    writeln!(io::stdout(), "result {:?}", ())?;
    Ok(())
}
