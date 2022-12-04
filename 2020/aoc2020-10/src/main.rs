// #[macro_use]
// extern crate lazy_static;
// use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Advent of Code 2020 -- Day 10 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let mut lines = input
        .lines()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    lines.sort();

    Ok(lines)
}

fn parse_jolt_gaps(jolts: &Vec<usize>) -> (HashMap<usize, usize>, usize) {
    jolts
        .iter()
        .fold((HashMap::new(), 0), |(mut map, last_jolts), jolt| {
            let gap = jolt - last_jolts;
            map.insert(gap, map.get(&gap).unwrap_or(&0) + 1);
            (map, *jolt)
        })
}

fn parse_jolt_diffs(jolts: &Vec<usize>) -> Vec<usize> {
    let (diffs, _) = jolts
        .iter()
        .fold((Vec::new(), 0), |(mut list, last_jolts), jolt| {
            let gap = jolt - last_jolts;
            list.push(gap);
            (list, *jolt)
        });
    diffs
}

fn count_configurations(jolts: &Vec<usize>) -> Result<usize, Box<dyn std::error::Error>> {
    let (configurations, _, _): (usize, usize, usize) = jolts
        .iter()
        .fold((1, 0, 0), |(config_count, repeating_one_diffs, last_jolt), jolt| {
            let gap = jolt - last_jolt;
            match (gap, repeating_one_diffs) {
                // (1, d) if d > 0 => {
                //     println!(">> on a role ({}): increasing configs from {} to {} (x{})", d, config_count, config_count * (2_usize.pow(d as u32 - 1)), (2_usize.pow(d as u32 - 1)));
                //     (config_count * (2_usize.pow(d as u32 - 1)), d + 1, *jolt)
                // }
                // (1, d) if d > 2 => {
                //     println!(">> on a role ({}): increasing configs from {} to {} (+{})", d, config_count, config_count + (2_usize.pow(d as u32)), (2_usize.pow(d as u32)));
                //     (config_count + 4, d + 1, *jolt)
                // }
                (1, d) if d > 0 => {
                    println!(">> on a role ({}): increasing configs from {} to {} (x2)", d, config_count, config_count * 2);
                    (config_count + 2, d + 1, *jolt)
                }

                // (1, _) => {
                //     println!(">> encountered a one");
                //     (config_count, 1, *jolt)
                // }
                _ => {
                    println!(">> resetting onecount");
                    (config_count, 0, *jolt)
                }
            }            
        });
        Ok(configurations)
}


fn part1(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let jolts = parse_input(input)?;
    let (jolt_gaps, _max_jolts) = parse_jolt_gaps(&jolts);

    let jolt_gap_1 = jolt_gaps.get(&1).unwrap();
    let jolt_gap_3 = jolt_gaps.get(&3).unwrap() + 1;

    let result = jolt_gap_1 * jolt_gap_3;
    writeln!(io::stdout(), "result {:?}", result)?;

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let jolts = parse_input(input).unwrap();
    let diffs = parse_jolt_diffs(&jolts);
    println!("result {:?}",diffs);
    // writeln!(io::stdout(), "result {:?}", result)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static MOCK_DATA: &str = "16
10
15
5
1
11
7
19
6
12
4";

    static MOCK_DATA_2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    // #[test]
    // fn test_gaps() {
    //     let jolts = parse_input(MOCK_DATA_2).unwrap();
    //     let (jolt_gaps, _max_jolts) = parse_jolt_gaps(&jolts);
    //     let jolt_gap_1 = jolt_gaps.get(&1).unwrap();
    //     let jolt_gap_3 = jolt_gaps.get(&3).unwrap() + 1;
    //     let result = jolt_gap_1 * jolt_gap_3;
    //     println!("result {:?}, {:?}", result, jolt_gaps);

    //     assert_eq!(result, 22);
    // }

    // #[test]
    // fn test_diffs() {
    //     let jolts = parse_input(MOCK_DATA_2).unwrap();
    //     let diffs = parse_jolt_diffs(&jolts);
    //     println!("result {:?}",diffs);

    //     assert_eq!(23, 22);
    // }

    #[test]
    fn test_count() {
        let jolts = parse_input(MOCK_DATA).unwrap();
        let configs = count_configurations(&jolts).unwrap();
        assert_eq!(configs, 8);
    }

    #[test]
    fn test_count2() {
        let jolts = parse_input(MOCK_DATA_2).unwrap();
        let configs = count_configurations(&jolts).unwrap();
        assert_eq!(configs, 19208);
    }
}
