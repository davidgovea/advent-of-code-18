use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::{self, Read, Write};

struct WireSegment {
    x: isize,
    y: isize,
}
fn parse_wire_segment(segment: &str) -> Result<WireSegment, Box<dyn std::error::Error>> {
    let (dir, number) = segment.split_at(1);
    let amount: isize = number.parse()?;
    match dir {
        "L" => Ok(WireSegment { x: -amount, y: 0 }),
        "R" => Ok(WireSegment { x: amount, y: 0 }),
        "U" => Ok(WireSegment { x: 0, y: amount }),
        "D" => Ok(WireSegment { x: 0, y: -amount }),
        _ => Err("Bad segment".into()),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Advent of Code 2019 -- Day 3 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut grid_cells = BTreeMap::<(usize, isize, isize), usize>::new();

    let all_wires_bitflag = input
        .lines()
        .enumerate()
        .fold(0, |all_wires_flag, (i, wire)| {
            let (mut x, mut y) = (0isize, 0isize);
            let bitflag = 1 << i;

            wire.split(",").for_each(|s| {
                let segment = parse_wire_segment(s).unwrap();
                let mag = (segment.x.abs() + segment.y.abs()) as usize;
                let dx = segment.x.signum();
                let dy = segment.y.signum();
                for _ in 1..=mag {
                    x += 1 * dx;
                    y += 1 * dy;
                    let grid_key = ((x.abs() + y.abs()) as usize, x, y);
                    grid_cells.insert(grid_key, grid_cells.get(&grid_key).unwrap_or(&0) | bitflag);
                }
            });
            all_wires_flag | bitflag
        });

    let ((nearest_intersection, _, _), _) = grid_cells
        .iter()
        .find(|(_, &val)| val & all_wires_bitflag == all_wires_bitflag)
        .unwrap();

    writeln!(io::stdout(), "result {:?}", nearest_intersection)?;

    Ok(*nearest_intersection)
}

fn part2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut grid_cells = BTreeMap::<(usize, isize, isize), (usize, usize)>::new();

    let all_wires_bitflag = input
        .lines()
        .enumerate()
        .fold(0, |all_wires_flag, (i, wire)| {
            let (mut x, mut y) = (0isize, 0isize);
            let bitflag = 1 << i;
            let mut steps = 0;

            wire.split(",").for_each(|s| {
                let segment = parse_wire_segment(s).unwrap();
                let mag = (segment.x.abs() + segment.y.abs()) as usize;
                let dx = segment.x.signum();
                let dy = segment.y.signum();
                for _ in 1..=mag {
                    steps += 1;
                    x += 1 * dx;
                    y += 1 * dy;
                    let grid_key = ((x.abs() + y.abs()) as usize, x, y);
                    let (flag, step_total) = grid_cells.get(&grid_key).unwrap_or(&(0usize, 0usize));
                    if flag & bitflag == 0 {
                        let new_flag = flag | bitflag;
                        let new_steps = step_total + steps;
                        grid_cells.insert(grid_key, (new_flag, new_steps));
                    }
                }
            });
            all_wires_flag | bitflag
        });

    let intersection_times = grid_cells
        .iter()
        .filter(|(_, &(flag, _))| flag & all_wires_bitflag == all_wires_bitflag)
        .map(|(_, &(_, steps))| steps)
        .collect::<BTreeSet<_>>();

    let earliest_intersection = intersection_times.iter().nth(0).unwrap();

    writeln!(io::stdout(), "result {:?}", earliest_intersection)?;
    Ok(*earliest_intersection)
}

#[cfg(test)]
mod tests {
    use super::*;

    static MOCK_INPUT_1: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";

    static MOCK_INPUT_2: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn test_nearest_intersection() {
        assert_eq!(part1(MOCK_INPUT_1).unwrap(), 159);
        assert_eq!(part1(MOCK_INPUT_2).unwrap(), 135);
    }

    #[test]
    fn test_earliest_intersection() {
        assert_eq!(part2(MOCK_INPUT_1).unwrap(), 610);
        assert_eq!(part2(MOCK_INPUT_2).unwrap(), 410);
    }
}
