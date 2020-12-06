use std::collections::BTreeMap;
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

fn part1(input: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(io::stdout(), "result {:?}", ())?;
    Ok(())
}
