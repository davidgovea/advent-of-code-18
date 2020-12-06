
#[derive(Debug)]
enum OpCode {
    Add,
    Multiply,
    Halt,
}

fn resolve_parameters(
    instruction_pointer: usize,
    program_memory: &Vec<i32>,
) -> Result<(i32, i32, usize), Box<dyn std::error::Error>> {
    let (operand_1_pointer, operand_2_pointer, dest_pointer) =
        get_parameter_pointers(&program_memory[instruction_pointer + 1..instruction_pointer + 4])?;
    Ok((
        program_memory[operand_1_pointer],
        program_memory[operand_2_pointer],
        dest_pointer,
    ))
}

fn get_parameter_pointers(input: &[i32]) -> Result<(usize, usize, usize), Box<dyn std::error::Error>> {
    match input {
        [a, b, c] => Ok(((*a) as usize, (*b) as usize, (*c) as usize)),
        _ => Err("Invalid parameters".into()),
    }
}

fn parse_op_code(input: i32) -> Result<OpCode, Box<dyn std::error::Error>> {
    match input {
        1 => Ok(OpCode::Add),
        2 => Ok(OpCode::Multiply),
        99 => Ok(OpCode::Halt),
        _ => Err("Bad opcode".into()),
    }
}

pub fn parse_intcode_program(input: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    Ok(input
        .split(",")
        .map(|n| n.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>())
}

pub fn run_intcode_program(program_memory: &mut Vec<i32>) -> Result<&Vec<i32>, Box<dyn std::error::Error>> {
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
        println!("{:?}", memory);
        assert_eq!(memory[..], [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn test_sample_2() {
        let mut memory = parse_intcode_program("1,0,0,0,99").unwrap();
        run_intcode_program(&mut memory).unwrap();
        println!("{:?}", memory);
        assert_eq!(memory[..], [2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_sample_3() {
        let mut memory = parse_intcode_program("2,3,0,3,99").unwrap();
        run_intcode_program(&mut memory).unwrap();
        println!("{:?}", memory);
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
        println!("{:?}", memory);
        assert_eq!(memory[..], [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
