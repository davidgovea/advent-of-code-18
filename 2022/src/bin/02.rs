#[derive(Debug, PartialEq, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum Result {
    Draw = 0,
    Win = 1,
    Lose = 2,
}

fn play_round(opponent_play: Play, my_play: Play) -> Result {
    // Use cyclic nature of R-P-S to calculate winner without hardcodes
    let result = (my_play as isize + 3 - opponent_play as isize) % 3;
    match result {
        0 => Result::Draw, // plays are equal
        1 => Result::Win,  // we played the "next" move -- coupled to enum definition order
        _ => Result::Lose,
    }
}

fn score_round(my_play: Play, result: Result) -> u32 {
    let play_score = match my_play {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    };
    let result_score = match result {
        Result::Lose => 0,
        Result::Draw => 3,
        Result::Win => 6,
    };

    play_score + result_score
}

fn parse_play(c: &str) -> Play {
    match c {
        // Opponent moves
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissors,
        // My moves (part 1 only)
        "X" => Play::Rock,
        "Y" => Play::Paper,
        "Z" => Play::Scissors,
        _ => panic!("Invalid string: {}", c),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_score = input
        .lines()
        .map(|round| {
            // Such a hullabaloo to get the parsed plays!
            let [opponent_play, my_play]: [Play; 2] = round
                .split_whitespace()
                .map(parse_play)
                .collect::<Vec<Play>>()
                .try_into()
                .ok() // I trust my inputs <3
                .unwrap();

            let result = play_round(opponent_play, my_play);
            score_round(my_play, result)
        })
        .sum();

    Some(total_score)
}

fn parse_desired_result(r: &str) -> Result {
    match r {
        "X" => Result::Lose,
        "Y" => Result::Draw,
        "Z" => Result::Win,
        _ => panic!("Invalid string: {}", r),
    }
}

fn determine_play(opponent_play: Play, desired_result: Result) -> Play {
    match (opponent_play as isize + desired_result as isize) % 3 {
        0 => Play::Rock,
        1 => Play::Paper,
        2 => Play::Scissors,
        _ => panic!("Invalid play"),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let total_score = input
        .lines()
        .map(|round| {
            let [opponent_symbol, result_symbol]: [&str; 2] = round
                .split_whitespace()
                .collect::<Vec<&str>>()
                .try_into()
                .ok() // I trust my inputs <3
                .unwrap();

            let opponent_play = parse_play(opponent_symbol);
            let desired_result = parse_desired_result(result_symbol);
            let my_play = determine_play(opponent_play, desired_result);

            let result = play_round(opponent_play, my_play);
            score_round(my_play, result)
        })
        .sum();

    Some(total_score)
}

fn main() {
    let input = &aoc2022::read_file("inputs", 2);
    aoc2022::solve!(1, part_one, input);
    aoc2022::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc2022::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = aoc2022::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
