use std::collections::HashMap;

fn parse_map(input: &str) -> (usize, usize, HashMap<(usize, usize), u32>) {
    let mut max_column_index = 0;
    let mut max_row_index = 0;
    let grid: HashMap<(usize, usize), u32> =
        input
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut map, (col, line)| {
                max_column_index = std::cmp::max(max_column_index, col);

                line.chars().enumerate().for_each(|(row, c)| {
                    max_row_index = std::cmp::max(max_row_index, row);
                    map.insert((row, col), c.to_string().parse::<u32>().unwrap());
                });
                map
            });

    (max_row_index + 1, max_column_index + 1, grid)
}

fn paths_to_edge(
    starting_coord: (usize, usize),
    rows: usize,
    columns: usize,
) -> [Vec<(usize, usize)>; 4] {
    let (start_x, start_y) = starting_coord;
    [(0, -1), (1, 0), (0, 1), (-1, 0)].map(|(dx, dy)| {
        let mut x = start_x as isize;
        let mut y = start_y as isize;
        let mut coords = vec![];
        loop {
            if x <= 0 || y <= 0 || x >= rows as isize - 1 || y >= columns as isize - 1 {
                break;
            }
            x += dx;
            y += dy;
            coords.push((x.try_into().unwrap(), y.try_into().unwrap()));
        }
        coords
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let (rows, columns, grid) = parse_map(input);

    let visible = grid.iter().filter(|((row, col), height)| {
        if *row == 0 || *row == rows - 1 || *col == 0 || *col == columns - 1 {
            return true;
        }

        paths_to_edge((*row, *col), rows, columns)
            .iter()
            .any(|coords| {
                coords
                    .iter()
                    .all(|(x, y)| grid.get(&(*x, *y)).unwrap() < height)
            })
    });
    let a = visible.collect::<Vec<_>>();

    Some(a.iter().count())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rows, columns, grid) = parse_map(input);

    let scores = grid.iter().map(|((row, col), height)| {
        if *row == 0 || *row == rows - 1 || *col == 0 || *col == columns - 1 {
            return 0;
        }
        paths_to_edge((*row, *col), rows, columns)
            .iter()
            .fold(1, |score_memo, coords| {
                let mut score = 0;
                for coord in coords {
                    match grid.get(&coord) {
                        None => {
                            break;
                        }
                        Some(h) => {
                            score += 1;
                            if h >= height {
                                break;
                            }
                        }
                    }
                }
                score_memo * score
            })
    });

    scores.max()
}

fn main() {
    let input = &aoc2022::read_file("inputs", 8);
    aoc2022::solve!(1, part_one, input);
    aoc2022::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc2022::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = aoc2022::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
