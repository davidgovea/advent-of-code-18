use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

lazy_static! {
    static ref OP_CODES: HashMap<usize, (OpCode, usize)> = hashmap! {
        1 => (OpCode::Add, 3),
        2 => (OpCode::Multiply, 3),
        3 => (OpCode::Input, 1),
        4 => (OpCode::Output, 1),
        5 => (OpCode::JumpIfTrue, 2),
        6 => (OpCode::JumpIfFalse, 2),
        7 => (OpCode::LessThan, 3),
        8 => (OpCode::Equals, 3),
        99 => (OpCode::Halt, 0),
    };
}

pub struct IntcodeVM<'a> {
    pub program_memory: &'a mut Vec<i32>,
    pub handle_input: Option<fn(Option<&i32>) -> i32>,
    instruction_pointer: usize,
    input: Option<i32>,
}

impl<'a> IntcodeVM<'a> {
    pub fn new(
        mem: &'a mut std::vec::Vec<i32>,
        handle_input: Option<fn(Option<&i32>) -> i32>,
    ) -> Self {
        Self {
            program_memory: mem,
            handle_input: handle_input,
            instruction_pointer: 0,
            input: None,
        }
    }
    pub fn run(&mut self) -> Vec<i32> {
        let mut outputs: Vec<i32> = Vec::new();
        while let Some(interrupt) = self.next() {
            match interrupt {
                Some(out) => outputs.push(out),
                None => match self.handle_input {
                    Some(handler) => {
                        self.input = Some(handler(outputs.last()));
                    }
                    None => {
                        panic!("Input requested, but no handler specified");
                    }
                },
            };
        }
        outputs
    }
}

impl Iterator for IntcodeVM<'_> {
    type Item = Option<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (op_code, param_modes) =
                parse_op_code(self.program_memory[self.instruction_pointer]).unwrap();
            let argument_count = param_modes.len();
            let parameters = get_parameters(
                self.instruction_pointer,
                argument_count,
                &self.program_memory,
            )
            .unwrap();

            match op_code {
                OpCode::Halt => break,
                OpCode::Add => {
                    let operand_1 = resolve_parameter(
                        parameters.get(0).unwrap(),
                        param_modes.get(0).unwrap(),
                        &self.program_memory,
                    )
                    .unwrap();
                    let operand_2 = resolve_parameter(
                        parameters.get(1).unwrap(),
                        param_modes.get(1).unwrap(),
                        &self.program_memory,
                    )
                    .unwrap();
                    let dest_pointer = *parameters.get(2).unwrap() as usize;
                    self.program_memory[dest_pointer] = operand_1 + operand_2;
                }
                OpCode::Multiply => {
                    let operand_1 = resolve_parameter(
                        parameters.get(0).unwrap(),
                        param_modes.get(0).unwrap(),
                        &self.program_memory,
                    )
                    .unwrap();
                    let operand_2 = resolve_parameter(
                        parameters.get(1).unwrap(),
                        param_modes.get(1).unwrap(),
                        &self.program_memory,
                    )
                    .unwrap();
                    let dest_pointer = *parameters.get(2).unwrap() as usize;
                    self.program_memory[dest_pointer] = operand_1 * operand_2;
                }
                OpCode::Input => {
                    match self.input {
                        Some(val) => {
                            let target = *parameters.get(0).unwrap() as usize;
                            // println!("INPUT={} AT: {}", val, target);
                            self.input = None;
                            self.program_memory[target] = val;
                        }
                        None => return Some(None),
                    }
                }
                OpCode::Output => {
                    let output = resolve_parameter(
                        parameters.get(0).unwrap(),
                        param_modes.get(0).unwrap(),
                        &self.program_memory,
                    )
                    .unwrap();
                    // println!("OUTPUT: {}", output);
                    self.instruction_pointer += 2;

                    return Some(Some(output));
                }
                OpCode::JumpIfTrue => {
                    let condition = resolve_parameter(
                        parameters.get(0).unwrap(),
                        param_modes.get(0).unwrap(),
                        &self.program_memory,
                    )
                    .unwrap();
                    let target = resolve_parameter(
                        parameters.get(1).unwrap(),
                        param_modes.get(1).unwrap(),
                        &self.program_memory,
                    )
                    .unwrap() as usize;
                    if condition != 0 {
                        self.instruction_pointer = target;
                        continue;
                    }
                }
                OpCode::JumpIfFalse => {
                    let condition = resolve_parameter(
                        parameters.get(0).unwrap(),
                        param_modes.get(0).unwrap(),
                        &self.program_memory,
                    )
                    .unwrap();
                    let target = resolve_parameter(
                        parameters.get(1).unwrap(),
                        param_modes.get(1).unwrap(),
                        &self.program_memory,
                    )
                    .unwrap() as usize;
                    if condition == 0 {
                        self.instruction_pointer = target;
                        continue;
                    }
                }
                OpCode::LessThan => {
                    let first = resolve_parameter(
                        parameters.get(0).unwrap(),
                        param_modes.get(0).unwrap(),
                        &self.program_memory,
                    )
                    .unwrap();
                    let second = resolve_parameter(
                        parameters.get(1).unwrap(),
                        param_modes.get(1).unwrap(),
                        &self.program_memory,
                    )
                    .unwrap();
                    let target = *parameters.get(2).unwrap() as usize;

                    match first < second {
                        true => self.program_memory[target] = 1,
                        false => self.program_memory[target] = 0,
                    }
                }
                OpCode::Equals => {
                    let first = resolve_parameter(
                        parameters.get(0).unwrap(),
                        param_modes.get(0).unwrap(),
                        &self.program_memory,
                    )
                    .unwrap();
                    let second = resolve_parameter(
                        parameters.get(1).unwrap(),
                        param_modes.get(1).unwrap(),
                        &self.program_memory,
                    )
                    .unwrap();
                    let target = *parameters.get(2).unwrap() as usize;

                    match first == second {
                        true => self.program_memory[target] = 1,
                        false => self.program_memory[target] = 0,
                    }
                }
            }

            self.instruction_pointer += 1 + argument_count;
        }
        None
    }
}

fn get_parameters(
    instruction_pointer: usize,
    count: usize,
    program_memory: &Vec<i32>,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    for param_i in 1..=count {
        let param_value = program_memory[instruction_pointer + param_i];
        results.push(param_value);
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

    let op_code_value = tens * 10 + ones;
    // println!("OPCODE {} {:?}", op_code_value, input);
    let (op_code, argument_count) = OP_CODES.get(&op_code_value).unwrap();
    Ok((
        *op_code,
        build_parameter_modes(*argument_count, param_mode_data)?,
    ))
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
    let mut vm = IntcodeVM::new(program_memory, None);
    vm.run();
    Ok(vm.program_memory)
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

    let mut vm = IntcodeVM::new(&mut program_memory, None);
    vm.run();

    Ok(vm.program_memory[0])
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
    fn test_sample_1_struct() {
        let mut memory = parse_intcode_program(MOCK_INPUT_1).unwrap();
        let mut vm = IntcodeVM::new(&mut memory, None);
        vm.run();
        assert_eq!(
            vm.program_memory[..],
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
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

    #[test]
    fn test_eq_8_position() {
        let mut memory = parse_intcode_program("3,9,8,9,10,9,4,9,99,-1,8").unwrap();
        let mut vm = IntcodeVM::new(&mut memory, Some(|_| 8));
        let outputs = vm.run();
        assert_eq!(outputs[..], [1]);

        let mut vm = IntcodeVM::new(&mut memory, Some(|_| 7));
        let outputs = vm.run();
        assert_eq!(outputs[..], [0]);
    }

    #[test]
    fn test_eq_8_immediate() {
        let mut memory = parse_intcode_program("3,3,1108,-1,8,3,4,3,99").unwrap();
        let mut vm = IntcodeVM::new(&mut memory, Some(|_| 8));
        let outputs = vm.run();
        assert_eq!(outputs[..], [1]);

        let mut vm = IntcodeVM::new(&mut memory, Some(|_| 7));
        let outputs = vm.run();
        assert_eq!(outputs[..], [0]);
    }

    #[test]
    fn test_lt_8_position() {
        let mut memory = parse_intcode_program("3,9,7,9,10,9,4,9,99,-1,8").unwrap();
        let mut vm = IntcodeVM::new(&mut memory, Some(|_| 2));
        let outputs = vm.run();
        assert_eq!(outputs[..], [1]);

        let mut vm = IntcodeVM::new(&mut memory, Some(|_| 9));
        let outputs = vm.run();
        assert_eq!(outputs[..], [0]);
    }

    #[test]
    fn test_lt_8_immediate() {
        let mut memory = parse_intcode_program("3,3,1107,-1,8,3,4,3,99").unwrap();
        let mut vm = IntcodeVM::new(&mut memory, Some(|_| 3));
        let outputs = vm.run();
        assert_eq!(outputs[..], [1]);

        let mut vm = IntcodeVM::new(&mut memory, Some(|_| 10));
        let outputs = vm.run();
        assert_eq!(outputs[..], [0]);
    }
}
