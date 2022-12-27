use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Coord = (usize, usize);
#[derive(Debug)]
struct Map {
    grid: HashMap<Coord, u32>,
    start: Coord,
    end: Coord,
}

fn parse_map(input: &str) -> Map {
    let mut grid = HashMap::new();
    let mut start: Coord = (0, 0);
    let mut end: Coord = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = (x, y);
                    grid.insert((x, y), 'a' as u32);
                }
                'E' => {
                    end = (x, y);
                    grid.insert((x, y), 'z' as u32);
                }
                _ => {
                    grid.insert((x, y), c as u32);
                }
            };
        }
    }
    Map { grid, start, end }
}

fn find_possible_moves(grid: &HashMap<Coord, u32>, current: Coord) -> Vec<Coord> {
    let mut moves = Vec::new();
    let current_value = grid.get(&current).unwrap();
    for (x, y) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let new_coord = (current.0 as i32 + x, current.1 as i32 + y);
        let new_coord = (new_coord.0 as usize, new_coord.1 as usize);
        if let Some(value) = grid.get(&new_coord) {
            if value <= &(current_value + &1) {
                moves.push(new_coord);
            }
        }
    }
    moves
}

fn perform_search(grid: HashMap<Coord, u32>, start: Coord, end: Coord) -> Option<u32> {
    let mut distances: HashMap<Coord, u32> = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    distances.insert(start, 0);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        let current_distance = *distances.get(&current).unwrap();
        if current == end {
            return Some(current_distance);
        }

        for neighbor in find_possible_moves(&grid, current) {
            if !distances.contains_key(&neighbor) {
                queue.push_back(neighbor);
                distances.insert(neighbor, current_distance + 1);
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_map(input);
    perform_search(map.grid, map.start, map.end)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc2022::read_file("inputs", 12);
    aoc2022::solve!(1, part_one, input);
    aoc2022::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc2022::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = aoc2022::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
