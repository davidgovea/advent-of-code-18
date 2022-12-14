#[derive(Debug)]
enum Instruction {
    NoOp,
    AddX(isize),
}

const CRT_ROWS: usize = 6;
const CRT_COLS: usize = 40;

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
    let mut register_x = 1;
    let mut tick: usize = 0;
    let mut instruction_pointer = 0;
    let mut complete_at: Option<usize> = None;
    let mut signal_strength: isize = 0;

    loop {
        // Capture signal strength at start of each tick
        if tick == 20 || (((tick as isize) - 20) % 40) == 0 {
            signal_strength += register_x * (tick as isize);
        }

        match (complete_at, instructions.get(instruction_pointer)) {
            (None, _) => {
                // First cycle
                complete_at = get_instruction_length(instructions.get(instruction_pointer));
            }
            (Some(t), Some(current_instruction)) if t == tick => {
                instruction_pointer += 1;

                match current_instruction {
                    Instruction::AddX(x) => {
                        register_x += x;
                    }
                    _ => (),
                };

                // Determine when the next instruction will complete
                match get_instruction_length(instructions.get(instruction_pointer)) {
                    Some(length) => {
                        complete_at = Some(tick + length);
                    }
                    None => break, // HALT: no more instructions
                };
            }
            _ => (/* clock tick with no state change */),
        }

        tick += 1;
    }

    Some(signal_strength)
}

pub fn part_two(input: &str) -> Option<isize> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
