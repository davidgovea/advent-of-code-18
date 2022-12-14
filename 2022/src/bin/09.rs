// Enums to match "Up", "Down", "Left", "Right" input strings with values

use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

// Parses input lines into a sequence of Movements
fn parse_input(input: &str) -> Vec<Movement> {
    input
        .lines()
        .flat_map(|line| {
            // Parse a line like "U 4" into Movement::Up(4)
            let mut parts = line.split_whitespace();
            let direction = parts.next().unwrap();
            let value = parts.next().unwrap().parse().unwrap();

            let dir = match direction {
                "U" => Movement::Up,
                "D" => Movement::Down,
                "L" => Movement::Left,
                "R" => Movement::Right,
                _ => panic!("Invalid direction"),
            };
            (0..value).map(move |_| dir)
        })
        .collect()
}

fn perform_move(movement: Movement, coord: (i32, i32)) -> (i32, i32) {
    match movement {
        Movement::Up => (coord.0, coord.1 + 1),
        Movement::Down => (coord.0, coord.1 - 1),
        Movement::Left => (coord.0 - 1, coord.1),
        Movement::Right => (coord.0 + 1, coord.1),
    }
}

fn determine_tail_coord(head_coord: (i32, i32), tail_coord: (i32, i32)) -> (i32, i32) {
    let diff_x = head_coord.0 - tail_coord.0;
    let diff_y = head_coord.1 - tail_coord.1;

    let skip_move = diff_x.abs() < 2 && diff_y.abs() < 2;
    if skip_move {
        return tail_coord;
    }

    let x_move = diff_x.signum();
    let y_move = diff_y.signum();
    (tail_coord.0 + x_move, tail_coord.1 + y_move)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut head_coord = (0, 0);
    let mut tail_coord = (0, 0);
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::from([tail_coord]);

    for command in parse_input(input) {
        head_coord = perform_move(command, head_coord);

        tail_coord = determine_tail_coord(head_coord, tail_coord);
        tail_visited.insert(tail_coord);
    }

    Some(tail_visited.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let num_knots = 10;
    let mut coords: Vec<(i32, i32)> = (0..num_knots).map(|_| (0, 0)).collect();
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    for command in parse_input(input) {
        coords = coords.iter().fold(vec![], |mut new_coords, current| {
            let prev = new_coords.last();

            let next = match prev {
                None => perform_move(command, *current),
                Some(c) => determine_tail_coord(*c, *current),
            };
            new_coords.push(next);
            new_coords
        });

        tail_visited.insert(*coords.last().unwrap());
    }

    Some(tail_visited.len().try_into().unwrap())
}

fn main() {
    let input = &aoc2022::read_file("inputs", 9);
    aoc2022::solve!(1, part_one, input);
    aoc2022::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc2022::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc2022::read_file("examples", 9);

        assert_eq!(part_two(&input), Some(1));
        assert_eq!(part_two(&aoc2022::read_file_custom("examples", 9, Some("-2"))), Some(36));
    }
}
