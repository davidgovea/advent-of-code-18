use std::collections::hash_map::Keys;
use aoc2019::intcode::IntcodeVM;
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::cell::RefCell;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Advent of Code 2019 -- Day 6 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

struct Orbits<'a> {
    map: OrbitsByPlanetMap<'a>,
}

struct OrbitIterator<'a> {
    map: &'a OrbitsByPlanetMap<'a>,
    current: &'a str,
    root: Option<&'a str>,
}

impl<'a> Orbits<'a> {
    pub fn new(map: OrbitsByPlanetMap<'a>) -> Self {
        Self { map }
    }
    pub fn from(input: &'a str) -> Self {
        Self::new(parse_orbit_map(input))
    }
    pub fn iter(&self, starting_planet: &'a str) -> OrbitIterator {
        OrbitIterator {
            map: &self.map,
            current: starting_planet,
            root: None,
        }
    }
    pub fn iter_until(&self, starting_planet: &'a str, root_body: &'a str) -> OrbitIterator {
        OrbitIterator {
            map: &self.map,
            current: starting_planet,
            root: Some(root_body),
        }
    }
    pub fn iter_bodies(&mut self) -> Keys<'_, &'a str, &'a str> {
        (&self.map).keys()
    }

    pub fn count_all_orbits(&mut self) -> usize {
        let self_ref = RefCell::new(self);
        let mut mut_self = self_ref.borrow_mut();
        let bodies = mut_self.iter_bodies().collect::<Vec<_>>();
        let total_orbits = bodies.iter().fold(0, |sum, planet| {
            // sum + &self.iter(planet).count()
            sum + self_ref.borrow().iter(planet).count()
            
        });
        total_orbits
    }
}

impl<'a, 'b: 'a> Iterator for OrbitIterator<'b> {
    type Item = &'b str;
    fn next(&mut self) -> Option<Self::Item> {
        let root = match self.root {
            Some(root) => root,
            _ => "COM",
        };
        match self.current {
            body if body != root => Some(self.map.get(body).unwrap()),
            _ => None,
        }
    }
}

type OrbitsByPlanetMap<'a> = HashMap<&'a str, &'a str>;

fn count_orbits(
    orbit_map: &OrbitsByPlanetMap,
    starting_body: &str,
    root_body: Option<&str>,
) -> usize {
    let root = match root_body {
        Some(root) => root,
        _ => "COM",
    };
    match orbit_map.get(starting_body) {
        Some(parent) if *parent == root => 1,
        Some(parent) => 1 + count_orbits(orbit_map, parent, root_body),
        None => 0,
    }
}

fn parse_orbit_map(input: &str) -> OrbitsByPlanetMap {
    input.lines().fold(HashMap::new(), |mut map, line| {
        let mut planet_iter = line.split(")");
        let parent_planet = planet_iter.next().unwrap();
        let planet = planet_iter.next().unwrap();
        map.insert(planet, parent_planet);
        map
    })
}

fn part1(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut orbits = Orbits::from(input);
    let total_orbits = orbits.count_all_orbits();

    writeln!(io::stdout(), "outputs: {:?}", total_orbits)?;

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    // writeln!(io::stdout(), "outputs: {:?}", outputs)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static MOCK_INPUT_1: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

    static MOCK_INPUT_2: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

    #[test]
    fn test_validity() {
        let orbit_map = parse_orbit_map(MOCK_INPUT_1);
        assert_eq!(count_orbits(&orbit_map, &"D", None), 3);
        let total_orbits = orbit_map.iter().fold(0, |sum, (planet, _)| {
            sum + count_orbits(&orbit_map, planet, None)
        });
        assert_eq!(total_orbits, 42);
    }

    #[test]
    fn test_path() {
        let orbit_map = parse_orbit_map(MOCK_INPUT_2);
        assert_eq!(count_orbits(&orbit_map, &"D", None), 3);
        let total_orbits = orbit_map.iter().fold(0, |sum, (planet, _)| {
            sum + count_orbits(&orbit_map, planet, None)
        });
        assert_eq!(total_orbits, 42);
    }
}
