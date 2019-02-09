#[macro_use] extern crate lazy_static;
use std::io::{self, Read, Write};
use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<(), Box<std::error::Error>> {
    println!("-- Advent of Code 2018 -- Day 3 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
            \[  (?P<timestamp>.*)  \]  \s+
            (?P<first_word>\w+)  \s+
            \#?
            (?P<guard_id>\d+)?
        ").unwrap();
    }

    let mut events = input.lines().map(|l| {
        let caps = RE.captures(l).unwrap();
        let event_type = match &caps["first_word"] {
            "wakes" => EventType::WakeUp,
            "Guard" => EventType::NewGuard(caps["guard_id"].parse().unwrap()),
            _ => EventType::FallAsleep,
        };
        GuardEvent {
            timestamp: caps["timestamp"].to_string(),
            event_type: event_type
        } 
    }).collect::<Vec<_>>();

    events.sort_by_key(|e| e.timestamp.clone()); // TODO -- how do i not clone here

    part1(&events)?;
    part2(&events)?;

    Ok(())
}

#[derive(Debug)]
enum EventType {
    NewGuard(u32),
    FallAsleep,
    WakeUp,
}

#[derive(Debug)]
struct GuardEvent {
    timestamp: String,
    event_type: EventType,
}

fn part1(sorted_events: &Vec<GuardEvent>) -> Result<(), Box<std::error::Error>> {
    
    // Group events by guard
    let mut events_by_guard: HashMap<u32, Vec<&GuardEvent>> = HashMap::new();
    let mut current_id: u32 = Default::default();
    for event in sorted_events {
        match event.event_type {
            EventType::NewGuard(id) => {
                current_id = id;
            },
            _ => {
                events_by_guard.entry(current_id).or_default().push(event);
            }
        }
    }

    // Create a per-minute time card for each guard
    let mut punchcard_map: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    for (guard_id, event_list) in events_by_guard.iter() {
        let mut sleeping_at: Option<u32> = None;
        for event in event_list {

            lazy_static! {
                static ref MINUTE_EXTRACT: Regex = Regex::new(r":(\d{2})").unwrap();

            }
            let caps = MINUTE_EXTRACT.captures(&event.timestamp).unwrap();

            let mut minute: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
            match event.event_type {
                EventType::WakeUp => {
                    for min in sleeping_at.unwrap()..minute {
                        *punchcard_map.entry(*guard_id).or_default().entry(min).or_default() += 1;
                    }
                    sleeping_at = None;
                    // mark sleep minutes
                }, 
                _ => {
                    sleeping_at = Some(minute);
                }
            }
        }
    }

    // Sum all timecards, and sort ascending
    let mut minute_totals: Vec<(u32, u32)> = punchcard_map
        .iter()
        .map(|(guard_id, punchcard)| {
            (*guard_id, punchcard.values().sum())
        })
        .collect::<Vec<_>>();
    minute_totals.sort_by_key(|t| t.1);

    // Determine sleepiest guard
    let most_sleepy_guard_id = minute_totals.last().unwrap().0;

    // Profile the guard's slept minutes - sort
    let mut minutes_worked = punchcard_map.get(&most_sleepy_guard_id).unwrap()
        .iter()
        .collect::<Vec<_>>();
    minutes_worked.sort_by_key(|m| m.1);

    let sleepiest_minute = minutes_worked.last().unwrap().0;
    
    writeln!(io::stdout(), "the sleepiest guard ({}) sleeps most deeply at {} past the stroke of midnight\n\nAnswer: guard * minute =\n{}", most_sleepy_guard_id, sleepiest_minute, most_sleepy_guard_id * sleepiest_minute)?;
    Ok(())
}

fn part2(sorted_events: &Vec<GuardEvent>) -> Result<(), Box<std::error::Error>> {

    // writeln!(io::stdout(), "result {}", ())?;
    Ok(())
}

