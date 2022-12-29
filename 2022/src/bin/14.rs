use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Formatter},
};

use nom::{
    bytes::complete::tag,
    character::complete::{char, u32},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

type Coord = (u32, u32);

#[derive(Debug, PartialEq)]
enum Material {
    Sand,
    Rock,
}

fn parse_coord(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, (x, _, y)) = tuple((u32, char(','), u32))(input)?;

    Ok((input, (x, y)))
}

fn parse_rocks(input: &str) -> HashSet<Coord> {
    input.lines().fold(HashSet::new(), |mut set, l| {
        let (_, coords) = separated_list1(tag(" -> "), parse_coord)(l).unwrap();
        coords.windows(2).for_each(|coords| {
            let (x1, y1) = coords[0];
            let (x2, y2) = coords[1];

            match x1 == x2 {
                true => (std::cmp::min(y1, y2)..=std::cmp::max(y1, y2)).for_each(|y| {
                    set.insert((x1, y));
                }),
                false => (std::cmp::min(x1, x2)..=std::cmp::max(x1, x2)).for_each(|x| {
                    set.insert((x, y1));
                }),
            }
        });
        set
    })
}

fn parse_map(input: &str) -> (HashMap<Coord, Material>, u32) {
    let mut map = HashMap::new();
    let mut lowest_level = 0;

    for rock in parse_rocks(input) {
        map.insert(rock, Material::Rock);
        lowest_level = std::cmp::max(lowest_level, rock.1);
    }
    (map, lowest_level)
}

struct World {
    map: HashMap<Coord, Material>,
    active_grain: Option<Coord>,
    complete: bool,
    lowest_level: u32,
    floor_level: u32,
}

const STARTING_POINT: Coord = (500, 0);

impl World {
    fn new(input: &str) -> Self {
        let (map, lowest_level) = parse_map(input);
        Self {
            map,
            active_grain: None,
            complete: false,
            lowest_level,
            floor_level: lowest_level + 2,
        }
    }
    fn get_falling_target(&self, coord: Coord) -> Option<Coord> {
        let (x, y) = coord;

        match self.map.get(&(x, y + 1)) {
            // Sitting on the floor
            None if y + 1 == self.floor_level => return None,
            // Air below, fall
            None => return Some((x, y + 1)),
            // Sand or rock below, check diagonals
            Some(_) => {
                if self.map.get(&(x - 1, y + 1)).is_none() {
                    return Some((x - 1, y + 1));
                }

                if self.map.get(&(x + 1, y + 1)).is_none() {
                    return Some((x + 1, y + 1));
                }
            }
        }

        None
    }

    fn _step(&mut self) -> Result<(), String> {
        if self.complete {
            return Err("World is complete".to_string());
        }

        if self.active_grain.is_none() {
            self.active_grain = Some(STARTING_POINT);
        }

        let active_grain = self.active_grain.unwrap();

        match self.get_falling_target(active_grain) {
            Some(target) => {
                self.active_grain = Some(target);
            }
            None => {
                self.map.insert(active_grain, Material::Sand);
                self.active_grain = None;
                // dbg!(&self);
            }
        }
        Ok(())
    }

    fn step_v1(&mut self) {
        self._step().unwrap();
        match self.active_grain {
            Some(coord) if coord.1 > self.lowest_level => {
                self.complete = true;
            }
            _ => (),
        };
    }

    fn step_v2(&mut self) -> Result<(), String> {
        self._step()?;
        match self.map.get(&STARTING_POINT) {
            Some(Material::Sand) => self.complete = true,
            _ => (),
        };
        Ok(())
    }

    fn count_grains(&self) -> u32 {
        self.map.values().filter(|m| match m {
            Material::Sand => true,
            _ => false,
        }).count() as u32
    }
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (min_x, max_x) = self.map.keys().fold(
            (STARTING_POINT.0, STARTING_POINT.0),
            |(min, max), (x, _)| (std::cmp::min(min, *x), std::cmp::max(max, *x)),
        );
        writeln!(f)?;
        for y in 0..=(self.lowest_level + 1) {
            for x in min_x - 1..=max_x + 1 {
                let c = match self.map.get(&(x, y)) {
                    Some(Material::Sand) => "o",
                    Some(Material::Rock) => "#",
                    None => ".",
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }

        for _ in min_x - 1..=max_x + 1 {
            write!(f, "#")?;
        }
        writeln!(f)?;

        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut world = World::new(input);

    while !world.complete {
        world.step_v1();
    }

    Some(world.count_grains())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut world = World::new(input);

    loop {
        match world.step_v2() {
            Ok(_) => (),
            Err(_) => break,
        }
    }

    Some(world.count_grains())
}

fn main() {
    let input = &aoc2022::read_file("inputs", 14);
    aoc2022::solve!(1, part_one, input);
    aoc2022::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc2022::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = aoc2022::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
