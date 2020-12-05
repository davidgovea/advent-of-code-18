use std::collections::HashMap;
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Advent of Code 2020 -- Day 1 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse_input(
    input: &str,
) -> Result<(Vec<HashMap<usize, bool>>, usize), Box<dyn std::error::Error>> {
    let mut map_width: usize = 0;
    let rows: Vec<HashMap<usize, bool>> = input
        .lines()
        .map(|n| {
            if map_width == 0 {
                map_width = n.len();
            }
            n.chars()
                .enumerate()
                .fold(HashMap::new(), |mut map, (i, c)| {
                    if c == '#' {
                        map.insert(i, true);
                    }
                    return map;
                })
        })
        .collect::<Vec<_>>();

    return Ok((rows, map_width));
}

fn part1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let (rows, map_width) = parse_input(input)?;

    let trees = check_slope(3, 1, &rows, map_width)?;

    writeln!(io::stdout(), "result {:?}", trees)?;
    Ok(trees)
}

fn check_slope(
    x_slope: usize,
    y_slope: usize,
    map: &Vec<HashMap<usize, bool>>,
    width: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut trees = 0;

    let mut index = 0;
    while (index * y_slope) < map.len() {
        let x = index * x_slope;
        if *map[index * y_slope].get(&(x % width)).unwrap_or(&false) {
            trees += 1;
        }
        index += 1;
    }

    Ok(trees)
}

fn part2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let (rows, map_width) = parse_input(input)?;

    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let trees_per_slope = slopes
        .iter()
        .map(|(x, y)| check_slope(*x, *y, &rows, map_width).unwrap());

    let tree_product = trees_per_slope.fold(1, |product, count| product * count);

    writeln!(io::stdout(), "result {:?}", tree_product)?;
    Ok(tree_product)
}

#[cfg(test)]
mod tests {
    use super::*;

    static MOCK_INPUT: &str = 
"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_validity() {
        assert_eq!(part1(MOCK_INPUT).unwrap(), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(MOCK_INPUT).unwrap(), 336);
    }

    #[test]
    fn test_check_slope() {
        let (map, width) = parse_input(MOCK_INPUT).unwrap();
        assert_eq!(check_slope(1, 1, &map, width).unwrap(), 2);
        assert_eq!(check_slope(3, 1, &map, width).unwrap(), 7);
        assert_eq!(check_slope(5, 1, &map, width).unwrap(), 3);
        assert_eq!(check_slope(7, 1, &map, width).unwrap(), 4);
        assert_eq!(check_slope(1, 2, &map, width).unwrap(), 2);
    }
}
