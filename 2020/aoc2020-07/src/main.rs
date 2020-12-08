#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Advent of Code 2020 -- Day 7 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

type RuleMap = HashMap<String, Option<Vec<(String, usize)>>>;
type ParentMap = HashMap<String, Vec<String>>;

fn parse_rules(input: &str) -> Result<(RuleMap, ParentMap), Box<dyn std::error::Error>> {
    lazy_static! {
        static ref RULE_EXTRACTOR: Regex = Regex::new(r"(?P<count>\d+)\s+(?P<bag>.*)").unwrap();
    }

    Ok(input.lines().fold(
        (HashMap::new(), HashMap::new()),
        |(mut rules_map, mut containers_map), line| {
            let mut split_line = line.split(" bags contain ");
            let outer_bag = split_line.next().unwrap();
            let rule_string = split_line.next().unwrap();
            match rule_string.contains("no other") {
                true => {
                    rules_map.insert(outer_bag.into(), None);
                }
                false => {
                    rule_string.split(", ").for_each(|r| {
                        let rule = r.split(" bag").next().unwrap();

                        let data = RULE_EXTRACTOR.captures(rule).unwrap();
                        let bag: String = data["bag"].to_string();
                        let count = data["count"].parse::<usize>().unwrap();
                        match rules_map.get_mut(outer_bag) {
                            Some(Some(vec)) => vec.push((bag.clone(), count)),
                            _ => {
                                rules_map
                                    .insert(outer_bag.into(), Some(vec![(bag.clone(), count)]));
                            }
                        }

                        match containers_map.get_mut(&bag) {
                            Some(vec) => vec.push(outer_bag.into()),
                            _ => {
                                containers_map.insert(bag, vec![outer_bag.into()]);
                            }
                        }
                    });
                }
            }
            // println!("Bag: {}, Rules: {:?}", outer_bag, inner_rules);

            (rules_map, containers_map)
        },
    ))
}

fn valid_parents(
    name: &str,
    parent_map: &ParentMap,
) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let mut parent_set: HashSet<String> = HashSet::new();
    if let Some(parents) = parent_map.get(name) {
        for parent in parents {
            parent_set.insert(parent.clone());
            for upper_parents in valid_parents(parent, parent_map) {
                upper_parents.iter().for_each(|p| {
                    parent_set.insert(p.clone());
                });
            }
        }
    }

    Ok(parent_set)
}

fn part1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let (_, parents_map) = parse_rules(input).unwrap();
    let possible_parents = valid_parents("shiny gold", &parents_map)?;

    writeln!(io::stdout(), "result {:?}", possible_parents.len())?;

    Ok(possible_parents.len())
}

fn inner_bags(name: &str, rules_map: &RuleMap) -> Result<usize, Box<dyn std::error::Error>> {
    let mut inner_count = 0;
    if let Some(Some(rules)) = rules_map.get(name) {
        for (bag, count) in rules {
            inner_count += count + count * inner_bags(bag, rules_map)?;
        }
    };
    Ok(inner_count)
}

fn part2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let (rules_map, _) = parse_rules(input).unwrap();
    let child_bags = inner_bags("shiny gold", &rules_map)?;

    writeln!(io::stdout(), "result {:?}", child_bags)?;

    Ok(child_bags)
}

#[cfg(test)]
mod tests {
    use super::*;

    static MOCK_DATA: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn test_parents() {
        let (_, parents_map) = parse_rules(MOCK_DATA).unwrap();
        assert_eq!(valid_parents("shiny gold", &parents_map).unwrap().len(), 4);
    }

    #[test]
    fn test_children() {
        let (rules_map, _) = parse_rules(MOCK_DATA).unwrap();
        assert_eq!(inner_bags("shiny gold", &rules_map).unwrap(), 32);
    }
}
