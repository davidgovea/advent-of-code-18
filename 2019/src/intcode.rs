use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum OpCode {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    Halt = 99,
}

lazy_static! {
    static ref ARGUMENT_COUNTS: HashMap<OpCode, usize> = hashmap! {
        OpCode::Add => 3,
        OpCode::Multiply => 3,
        OpCode::Input => 1,
        OpCode::Output => 1,
        OpCode::Halt => 0,
    };
}

fn get_parameters(
    instruction_pointer: &mut usize,
    count: usize,
    program_memory: &Vec<i32>,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    *instruction_pointer += 1;
    for _param_i in 0..count {
        let param_value = program_memory[*instruction_pointer];
        results.push(param_value);
        *instruction_pointer += 1;
    }
    Ok(results)
}

fn resolve_parameter(
    param_value: &i32,
    param_mode: &usize,
    program_memory: &Vec<i32>,
) -> Result<i32, Box<dyn std::error::Error>> {
    match param_mode {
        0 => Ok(program_memory[(*param_value) as usize]),
        1 => Ok(*param_value),
        _ => Err("Bad mode".into()),
    }
}

fn build_parameter_modes(
    num_parameters: usize,
    param_mode_data: Vec<usize>,
) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let mut modes = Vec::new();
    for i in 0..num_parameters {
        modes.push(*param_mode_data.get(i).unwrap_or(&0));
    }
    Ok(modes)
}

fn parse_op_code(input: i32) -> Result<(OpCode, Vec<usize>), Box<dyn std::error::Error>> {
    let mut digits_reverse = input
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>()
        .into_iter()
        .rev();
    let ones = digits_reverse.next().unwrap();
    let tens = digits_reverse.next().unwrap_or(0);
    let param_mode_data = digits_reverse.collect::<Vec<usize>>();

    let op_code = tens * 10 + ones;
    // println!("OPCODE {} {:?}", op_code, input);
    match op_code {
        1 => Ok((OpCode::Add, build_parameter_modes(3, param_mode_data)?)),
        2 => Ok((OpCode::Multiply, build_parameter_modes(3, param_mode_data)?)),
        3 => Ok((OpCode::Input, build_parameter_modes(1, param_mode_data)?)),
        4 => Ok((OpCode::Output, build_parameter_modes(1, param_mode_data)?)),
        99 => Ok((OpCode::Halt, vec![])),
        _ => Err("Bad opcode".into()),
    }
}

pub fn parse_intcode_program(input: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    Ok(input
        .split(",")
        .map(|n| n.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>())
}

pub fn run_intcode_program(
    program_memory: &mut Vec<i32>,
) -> Result<&Vec<i32>, Box<dyn std::error::Error>> {
    let mut instruction_pointer = 0;
    loop {
        let (op_code, param_modes) = parse_op_code(program_memory[instruction_pointer])?;
        // println!("MEM: {:?}", program_memory);
        // println!("PTR: {} | Parsed opcode & modes {:?} {:?}", instruction_pointer, op_code, param_modes);
        match op_code {
            OpCode::Halt => break,
            OpCode::Add => {
                let parameters = get_parameters(&mut instruction_pointer, 3, &program_memory)?;
                // println!("ADD Params: {:?}", parameters);

                let operand_1 = resolve_parameter(
                    parameters.get(0).unwrap(),
                    param_modes.get(0).unwrap(),
                    &program_memory,
                )?;
                let operand_2 = resolve_parameter(
                    parameters.get(1).unwrap(),
                    param_modes.get(1).unwrap(),
                    &program_memory,
                )?;
                let dest_pointer = *parameters.get(2).unwrap() as usize;
                // println!("ADD Setting target {}: to {}+{} = {}", dest_pointer, operand_1, operand_2, operand_1 + operand_2);
                program_memory[dest_pointer] = operand_1 + operand_2;
            }
            OpCode::Multiply => {
                let parameters = get_parameters(&mut instruction_pointer, 3, &program_memory)?;
                // println!("MULT Params: {:?}", parameters);

                let operand_1 = resolve_parameter(
                    parameters.get(0).unwrap(),
                    param_modes.get(0).unwrap(),
                    &program_memory,
                )?;
                let operand_2 = resolve_parameter(
                    parameters.get(1).unwrap(),
                    param_modes.get(1).unwrap(),
                    &program_memory,
                )?;
                let dest_pointer = *parameters.get(2).unwrap() as usize;
                // println!("MULT Setting target {}: to {}+{} = {}", dest_pointer, operand_1, operand_2, operand_1 * operand_2);
                program_memory[dest_pointer] = operand_1 * operand_2;
            }
            OpCode::Input => {
                let parameters = get_parameters(&mut instruction_pointer, 1, &program_memory)?;
                // println!("INPUT Params: {:?}", parameters);

                let target = *parameters.get(0).unwrap() as usize;
                // println!("FORCING INPUT=1 AT: {}", target);
                program_memory[target] = 1;
            }
            OpCode::Output => {
                let parameters = get_parameters(&mut instruction_pointer, 1, &program_memory)?;
                // println!("OUTPUT Params: {:?}", parameters);

                let output = resolve_parameter(
                    parameters.get(0).unwrap(),
                    param_modes.get(0).unwrap(),
                    &program_memory,
                )?;
                println!("OUTPUT: {}", output);
            }
        }
    }

    Ok(program_memory)
}

pub fn perform_computation(
    noun: i32,
    verb: i32,
    initial_memory: &Vec<i32>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut program_memory = initial_memory.clone();

    // Add 'noun' / 'verb' parameters
    program_memory[1] = noun;
    program_memory[2] = verb;

    run_intcode_program(&mut program_memory)?;

    Ok(program_memory[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    static MOCK_INPUT_1: &str = "1,9,10,3,2,3,11,0,99,30,40,50";

    #[test]
    fn test_sample_1() {
        let mut memory = parse_intcode_program(MOCK_INPUT_1).unwrap();
        run_intcode_program(&mut memory).unwrap();
        assert_eq!(memory[..], [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn test_sample_2() {
        let mut memory = parse_intcode_program("1,0,0,0,99").unwrap();
        run_intcode_program(&mut memory).unwrap();
        assert_eq!(memory[..], [2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_sample_3() {
        let mut memory = parse_intcode_program("2,3,0,3,99").unwrap();
        run_intcode_program(&mut memory).unwrap();
        assert_eq!(memory[..], [2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_sample_4() {
        let mut memory = parse_intcode_program("2,4,4,5,99,0").unwrap();
        run_intcode_program(&mut memory).unwrap();
        println!("{:?}", memory);
        assert_eq!(memory[..], [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_sample_5() {
        let mut memory = parse_intcode_program("1,1,1,4,99,5,6,0,99").unwrap();
        run_intcode_program(&mut memory).unwrap();
        assert_eq!(memory[..], [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_param_modes() {
        // (position4)33 * (immediate)3, store result in position 4
        let mut memory = parse_intcode_program("1002,4,3,4,33").unwrap();
        run_intcode_program(&mut memory).unwrap();
        println!("{:?}", memory);
        assert_eq!(memory[..], [1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_negative_int() {
        // find 100 + -1, store the result in position 4
        let mut memory = parse_intcode_program("1101,100,-1,4,0").unwrap();
        run_intcode_program(&mut memory).unwrap();
        println!("{:?}", memory);
        assert_eq!(memory[..], [1101, 100, -1, 4, 99]);
    }
}
