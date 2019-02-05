use std::io::{self, Read, Write};
use std::collections::HashMap;

fn main() -> Result<(), Box<std::error::Error>> {
    println!("-- Advent of Code 2018 -- Day 2 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<(), Box<std::error::Error>> {

    let (two_repeat_count, three_repeat_count) = input
        .lines()
        .fold((0, 0), |(mut two_repeat_count, mut three_repeat_count), box_id| {
            let mut seen_chars = HashMap::new();
            for c in box_id.chars() {
                match seen_chars.contains_key(&c) {
                    true => seen_chars.insert(c, seen_chars.get(&c).unwrap() + 1),
                    false => seen_chars.insert(c, 1),
                };
            }

            if seen_chars.iter().find(|(_c, n)| n == &&2).is_some() {
                two_repeat_count += 1;
            }
            if seen_chars.iter().find(|(_c, n)| n == &&3).is_some() {
                three_repeat_count += 1;
            }

            (two_repeat_count, three_repeat_count)
        });
    
    let checksum = two_repeat_count * three_repeat_count;

    writeln!(io::stdout(), "wow, that's a lot of boxes. good thing I counted this way. carry the one, andd.. checksum is {}", checksum)?;
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<std::error::Error>> {

    let mut box_ids = input.lines().collect::<Vec<_>>();
    box_ids.sort();

    let fabric_boxes = box_ids
        .windows(2)
        .find(|pair| {
            pair[0]
                .chars()
                .zip(pair[1].chars())
                .try_fold(0, |different, (c1, c2)| {
                    match c1 != c2 {
                        true if different == 0 => Ok(1),
                        false => Ok(different),
                        _ => Err("differs by more than one character")
                    }
                }) == Ok(1)
        }).unwrap();
    
    writeln!(io::stdout(), "looks like these two are the right ones. let's see if the fabric is inside!\n{}\n{}\n", fabric_boxes[0], fabric_boxes[1])?;

    let shared_chars = fabric_boxes[0]
        .chars()
        .zip(fabric_boxes[1].chars())
        .fold(String::new(), |shared, (c1, c2)| {
            match c1 == c2 {
                true => shared + &c1.to_string(),
                false => shared
            }
        });

    writeln!(io::stdout(), "the two boxes share the letters:\n{}", shared_chars)?;
    Ok(())
}

