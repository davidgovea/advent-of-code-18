#[derive(Debug)]
enum Instruction {
    NoOp,
    AddX(isize),
}

const CRT_COLS: usize = 40;

struct WalkieVM {
    program: Vec<Instruction>,
    register_x: isize,
    tick: usize,
    instruction_pointer: usize,
    complete_at: Option<usize>,
}

impl WalkieVM {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            program: program,
            register_x: 1,
            tick: 0,
            instruction_pointer: 0,
            complete_at: None,
        }
    }
}

struct WalkieVMState {
    tick: usize,
    register_x: isize,
}

impl Iterator for WalkieVM {
    type Item = WalkieVMState;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.complete_at, self.program.get(self.instruction_pointer)) {
            (None, _) => {
                // First cycle
                self.complete_at =
                    get_instruction_length(self.program.get(self.instruction_pointer));
            }
            (Some(t), Some(current_instruction)) if t == self.tick => {
                self.instruction_pointer += 1;

                match current_instruction {
                    Instruction::AddX(x) => {
                        self.register_x += x;
                    }
                    _ => (),
                };

                // Determine when the next instruction will complete
                match get_instruction_length(self.program.get(self.instruction_pointer)) {
                    Some(length) => {
                        self.complete_at = Some(self.tick + length);
                    }
                    None => {
                        return None;
                    }
                };
            }
            _ => (/* clock tick with no state change */),
        }

        self.tick += 1;
        Some(WalkieVMState {
            tick: self.tick,
            register_x: self.register_x,
        })
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            // Parse a line like "noop" into NoOp
            let mut parts = line.split_whitespace();
            let instruction = parts.next().unwrap();

            match instruction {
                "noop" => Instruction::NoOp,
                "addx" => Instruction::AddX(parts.next().unwrap().parse().unwrap()),
                _ => panic!("Invalid instruction"),
            }
        })
        .collect()
}

fn get_instruction_length(instruction: Option<&Instruction>) -> Option<usize> {
    match instruction {
        Some(Instruction::NoOp) => Some(1),
        Some(Instruction::AddX(_)) => Some(2),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let instructions = parse_input(input);
    let mut vm = WalkieVM::new(instructions);

    let mut signal_strength: isize = 0;

    for state in vm {
        if state.tick == 20 || (((state.tick as isize) - 20) % 40) == 0 {
            signal_strength += state.register_x * (state.tick as isize);
        }
    }

    Some(signal_strength)
}

#[derive(Debug, Clone)]
enum CRTPixel {
    Lit,
    Dark,
}

pub fn part_two(input: &str) -> Option<String> {
    let program = parse_input(input);
    let mut vm = WalkieVM::new(program);
    let mut crt_buffer: Vec<CRTPixel> = vec![];

    for state in vm {
        let drawing_pixel = (state.tick - 1) % CRT_COLS;
        let sprite_location = state.register_x;
        let is_lit = (drawing_pixel as isize - sprite_location).abs() <= 1;
        match is_lit {
            true => crt_buffer.push(CRTPixel::Lit),
            false => crt_buffer.push(CRTPixel::Dark),
        };
    }

    let display = crt_buffer
        .chunks(CRT_COLS)
        .map(|row| {
            row.iter()
                .map(|pixel| match pixel {
                    CRTPixel::Lit => "#",
                    CRTPixel::Dark => ".",
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    Some(display.join("\n"))
}

fn main() {
    let input = &aoc2022::read_file("inputs", 10);
    aoc2022::solve!(1, part_one, input);
    aoc2022::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc2022::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = aoc2022::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(
                "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
                .trim()
                .to_string()
            )
        );
    }
}
