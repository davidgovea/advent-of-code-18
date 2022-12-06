type Layout = Vec<Vec<char>>;

fn parse_input(input: &str) -> [&str; 2] {
    input
        .split("\n\n")
        .collect::<Vec<_>>()
        .try_into()
        .ok()
        .unwrap()
}

fn parse_layout(layout: &str) -> Layout {
    let mut layout_iter = layout.lines().rev();
    let columns = layout_iter.next().unwrap().split_whitespace().count();

    let mut state = (0..columns).map(|_| Vec::new()).collect::<Vec<_>>();

    layout_iter.for_each(|row| {
        let chars = row.as_bytes();
        (0..columns).for_each(|i| {
            let target_index = i * 4 + 1;
            match chars.get(target_index) {
                Some(c) if !(*c as char).is_whitespace() => {
                    state.get_mut(i).unwrap().push(*c as char)
                }
                _ => (),
            };
        });
    });

    state
}

fn perform_move(count: usize, from: usize, to: usize, layout: &mut Layout) -> () {
    let origin = &layout.as_slice()[from - 1];
    let move_index = origin.len() - count;

    let to_move = &origin[move_index..].to_owned();

    let target = layout.get_mut(to - 1).unwrap();
    target.extend(to_move.iter().rev());

    let from_stack = layout.get_mut(from - 1).unwrap();
    from_stack.truncate(move_index);
}

fn parse_instruction(line: &str) -> [usize; 3] {
    let words = line.split_whitespace().collect::<Vec<_>>();
    [words.get(1), words.get(3), words.get(5)].map(|d| d.unwrap().parse().unwrap())
}

fn print_top_crates(layout: Layout) -> String {
    layout
        .into_iter()
        .map(|stack| stack.last().unwrap().to_owned())
        .fold(String::new(), |mut res, c| {
            res.push(c);
            res
        })
}

pub fn part_one(input: &str) -> Option<String> {
    let [initial_layout, instructions] = parse_input(input);
    let mut layout = parse_layout(initial_layout);

    for instruction in instructions.lines() {
        let [count, from, to] = parse_instruction(instruction);

        perform_move(count, from, to, &mut layout);
    }

    Some(print_top_crates(layout))
}

fn perform_move_over9000(count: usize, from: usize, to: usize, layout: &mut Layout) -> () {
    let origin = &layout.as_slice()[from - 1];
    let move_index = origin.len() - count;

    let to_move = &origin[move_index..].to_owned();

    let target = layout.get_mut(to - 1).unwrap();
    target.extend(to_move.iter());

    let from_stack = layout.get_mut(from - 1).unwrap();
    from_stack.truncate(move_index);
}

pub fn part_two(input: &str) -> Option<String> {
    let [initial_layout, instructions] = parse_input(input);
    let mut layout = parse_layout(initial_layout);

    for instruction in instructions.lines() {
        let [count, from, to] = parse_instruction(instruction);

        perform_move_over9000(count, from, to, &mut layout);
    }

    Some(print_top_crates(layout))
}

fn main() {
    let input = &aoc2022::read_file("inputs", 5);
    aoc2022::solve!(1, part_one, input);
    aoc2022::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc2022::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = aoc2022::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
