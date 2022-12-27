use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, one_of, u64},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult, Parser,
};

#[derive(Debug, Clone)]
struct Monkey {
    pub items_inspected: u64,
    pub items: Vec<u64>,
    pub operation: Operation,
    pub divisor: u64,
    pub receiver_if_true: usize,
    pub receiver_if_false: usize,
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add(Term, Term),
    Mul(Term, Term),
}

impl Operation {
    pub fn eval(self, old: u64) -> u64 {
        match self {
            Operation::Add(l, r) => l.eval(old) + r.eval(old),
            Operation::Mul(l, r) => l.eval(old) * r.eval(old),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Term {
    Old,
    Constant(u64),
}

impl Term {
    pub fn eval(self, old: u64) -> u64 {
        match self {
            Term::Old => old,
            Term::Constant(c) => c,
        }
    }
}

fn parse_monkey(input: &str) -> IResult<&str, u64> {
    let (input, monkey) = delimited(tag("Monkey "), u64, tag(":"))(input)?;

    Ok((input, monkey))
}

fn parse_starting_items(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("Starting items: ")(input)?;
    let (input, items) = separated_list1(tag(", "), u64)(input)?;

    Ok((input, items))
}

fn parse_term(input: &str) -> IResult<&str, Term> {
    alt((
        tag("old").map(|_| Term::Old),
        u64.map(|d| Term::Constant(d)),
    ))(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("Operation: new = ")(input)?;

    let (input, (l, op, r)) = tuple((
        parse_term,
        delimited(multispace1, one_of("*+"), multispace1),
        parse_term,
    ))(input)?;
    let op = match op {
        '*' => Operation::Mul(l, r),
        '+' => Operation::Add(l, r),
        _ => unreachable!(),
    };
    Ok((input, op))
}

fn parse_test(input: &str) -> IResult<&str, (u64, u64, u64)> {
    let (input, _) = multispace1(input)?;

    let (input, _) = tag("Test: divisible by ")(input)?;
    let (input, divisible_by) = u64(input)?;

    let (input, _) = multispace1(input)?;
    let (input, _) = tag("If true: throw to monkey ")(input)?;
    let (input, true_branch) = u64(input)?;

    let (input, _) = multispace1(input)?;
    let (input, _) = tag("If false: throw to monkey ")(input)?;
    let (input, false_branch) = u64(input)?;

    Ok((input, (divisible_by, true_branch, false_branch)))
}

fn parse_monkey_block(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = multispace0(input)?;
    let (input, (_, starting_items, operation, (divisible_by, true_branch, false_branch))) =
        tuple((
            parse_monkey,
            parse_starting_items,
            parse_operation,
            parse_test,
        ))(input)?;

    let monkey = Monkey {
        items_inspected: 0,
        items: starting_items,
        operation,
        divisor: divisible_by,
        receiver_if_true: true_branch as usize,
        receiver_if_false: false_branch as usize,
    };

    Ok((input, monkey))
}

// Use the `separated_list1` combinator to parse a list of `Monkey` structs from the input
fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, monkeys) = separated_list1(tag("\n\n"), parse_monkey_block)(input)?;

    Ok((input, monkeys))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, mut monkeys) = parse_input(input).unwrap();

    for _round in 0..20 {
        for monkey_index in 0..monkeys.len() {
            for item_index in 0..monkeys[monkey_index].items.len() {
                let new_worry;
                let receiver_index;
                {
                    let monkey = monkeys.get_mut(monkey_index).unwrap();
                    let item = monkey.items[item_index];
                    new_worry = (monkey.operation.eval(item) as f64 / 3.0).floor() as u64;
                    receiver_index = match new_worry % monkey.divisor {
                        0 => monkey.receiver_if_true,
                        _ => monkey.receiver_if_false,
                    };
                }
                let receiver = monkeys.get_mut(receiver_index).unwrap();
                receiver.items.push(new_worry);
            }

            let monkey = monkeys.get_mut(monkey_index).unwrap();
            monkey.items_inspected += monkey.items.len() as u64;
            monkey.items.clear();
        }
    }

    // sort monkeys by items_inspected
    monkeys.sort_by(|a, b| b.items_inspected.cmp(&a.items_inspected));
    Some(
        monkeys
            .iter()
            .take(2)
            .map(|m| m.items_inspected)
            .fold(1, |acc, x| acc * x),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, mut monkeys) = parse_input(input).unwrap();

    let test_product: u64 = monkeys.iter().map(|m| m.divisor).product();

    for _round in 0..10000 {
        for monkey_index in 0..monkeys.len() {
            for item_index in 0..monkeys[monkey_index].items.len() {
                let new_worry;
                let receiver_index;
                {
                    let monkey = monkeys.get_mut(monkey_index).unwrap();
                    let item = monkey.items[item_index];
                    new_worry = monkey.operation.eval(item) % test_product;
                    receiver_index = match new_worry % monkey.divisor {
                        0 => monkey.receiver_if_true,
                        _ => monkey.receiver_if_false,
                    };
                }
                let receiver = monkeys.get_mut(receiver_index).unwrap();
                receiver.items.push(new_worry);
            }

            let monkey = monkeys.get_mut(monkey_index).unwrap();
            monkey.items_inspected += monkey.items.len() as u64;
            monkey.items.clear();
        }
    }

    // sort monkeys by items_inspected
    monkeys.sort_by(|a, b| b.items_inspected.cmp(&a.items_inspected));
    Some(
        monkeys
            .iter()
            .take(2)
            .map(|m| m.items_inspected)
            .fold(1, |acc, x| acc * x),
    )
}

fn main() {
    let input = &aoc2022::read_file("inputs", 11);
    aoc2022::solve!(1, part_one, input);
    aoc2022::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc2022::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = aoc2022::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
