use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<usize> {
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

    let visible = grid.iter().filter(|((row, col), height)| {
        if *row == 0 || *row == max_row_index || *col == 0 || *col == max_column_index {
            return true;
        }
        let visible_l = (0..*row).all(|x| grid.get(&(x, *col)).unwrap() < height);
        let visible_r = (row + 1..=max_row_index).all(|x| grid.get(&(x, *col)).unwrap() < height);
        let visible_u = (0..*col).all(|y| grid.get(&(*row, y)).unwrap() < height);
        let visible_d =
            (col + 1..=max_column_index).all(|y| grid.get(&(*row, y)).unwrap() < height);

        visible_l || visible_r || visible_u || visible_d
    });
    let a = visible.collect::<Vec<_>>();

    Some(a.iter().count())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
