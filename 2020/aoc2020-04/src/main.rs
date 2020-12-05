#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
use regex::Regex;
use std::collections::HashSet;
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Advent of Code 2020 -- Day 4 --\n");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut passports: Vec<String> = Vec::new();
    let mut streaming_buffer: Vec<String> = Vec::new();
    for line in input.lines() {
        if line != "" {
            streaming_buffer.push(String::from(line));
        } else {
            passports.push(streaming_buffer.join(" "));
            streaming_buffer.clear();
        }
    }
    if streaming_buffer.len() > 0 {
        passports.push(streaming_buffer.join(" "));
    }

    return Ok(passports);
}

fn validate_passport(passport: &String) -> Result<bool, Box<dyn std::error::Error>> {
    lazy_static! {
        static ref FIELDS: HashSet<String> = hashset! {
        "byr".into(),
        "iyr".into(),
        "eyr".into(),
        "hgt".into(),
        "hcl".into(),
        "ecl".into(),
        "pid".into(),
        "cid".into()
        };
    }
    let missing_fields = passport
        .split(' ')
        .fold(FIELDS.clone(), |mut observed, field_data| {
            let field = field_data.split(":").nth(0).unwrap().into();
            if observed.contains(field) {
                observed.remove(field);
            }
            observed
        });
    Ok(missing_fields.len() == 0 || (missing_fields.len() == 1 && missing_fields.contains("cid")))
}

fn part1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let valid = parse_input(input)?
        .iter()
        .filter(|passport| validate_passport(passport).unwrap())
        .count();

    writeln!(io::stdout(), "result {:?}", valid)?;
    Ok(valid)
}

fn validate_passport2(passport: &String) -> Result<bool, Box<dyn std::error::Error>> {
    lazy_static! {
        static ref FIELDS: HashSet<String> = hashset! {
        "byr".into(),
        "iyr".into(),
        "eyr".into(),
        "hgt".into(),
        "hcl".into(),
        "ecl".into(),
        "pid".into(),
        "cid".into()
        };
        static ref YEAR_MATCHER: Regex = Regex::new(r"^(?P<year>\d{4})$").unwrap();
        static ref HEIGHT_MATCHER: Regex = Regex::new(r"^(?P<val>\d+)(?P<unit>in|cm)$").unwrap();
        static ref COLOR_MATCHER: Regex = Regex::new(r"^#(?P<color>[0-9a-f]{6})$").unwrap();
        static ref EYE_MATCHER: Regex =
            Regex::new(r"^(?P<color>amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        static ref PASSPORT_MATCHER: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    let invalid_fields =
        passport
            .split(' ')
            .try_fold(FIELDS.clone(), |mut observed, field_data| {
                let name_value = field_data.split(":").take(2).collect::<Vec<_>>();
                let valid = match &name_value[0..2] {
                    ["byr", value] => match YEAR_MATCHER.captures(value) {
                        Some(data) => {
                            let year = data["year"].parse::<usize>().unwrap();
                            year >= 1920 && year <= 2002
                        }
                        _ => false,
                    },
                    ["iyr", value] => match YEAR_MATCHER.captures(value) {
                        Some(data) => {
                            let year = data["year"].parse::<usize>().unwrap();
                            year >= 2010 && year <= 2020
                        }
                        _ => false,
                    },
                    ["eyr", value] => match YEAR_MATCHER.captures(value) {
                        Some(data) => {
                            let year = data["year"].parse::<usize>().unwrap();
                            year >= 2020 && year <= 2030
                        }
                        _ => false,
                    },
                    ["hgt", value] => match HEIGHT_MATCHER.captures(value) {
                        Some(data) => {
                            let unit = &data["unit"];
                            let val = &data["val"].parse::<usize>();
                            match (unit, val) {
                                ("in", Ok(val_in)) => *val_in >= 59 && *val_in <= 76,
                                ("cm", Ok(val_cm)) => *val_cm >= 150 && *val_cm <= 193,
                                _ => false,
                            }
                        }
                        _ => false,
                    },
                    ["hcl", value] => COLOR_MATCHER.is_match(value),
                    ["ecl", value] => EYE_MATCHER.is_match(value),
                    ["pid", value] => PASSPORT_MATCHER.is_match(value),
                    [field_name, _value] => FIELDS.contains(*field_name),
                    _ => return Err("Bad field format"),
                };
                if valid {
                    observed.remove(name_value[0]);
                }
                Ok(observed)
            })?;
    Ok(invalid_fields.len() == 0 || (invalid_fields.len() == 1 && invalid_fields.contains("cid")))
}

fn part2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let valid = parse_input(input)?
        .iter()
        .filter(|passport| validate_passport2(passport).unwrap())
        .count();

    writeln!(io::stdout(), "result {:?}", valid)?;
    Ok(valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    static MOCK_PASSPORTS: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_passport_grouping() {
        assert_eq!(parse_input(MOCK_PASSPORTS).unwrap().len(), 4);
    }

    #[test]
    fn test_passport_validity() {
        let passports = parse_input(MOCK_PASSPORTS).unwrap();
        assert_eq!(validate_passport(passports.get(0).unwrap()).unwrap(), true);
        assert_eq!(validate_passport(passports.get(1).unwrap()).unwrap(), false);
        assert_eq!(validate_passport(passports.get(2).unwrap()).unwrap(), true);
    }

    static INVALID_DATA: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    static VALID_DATA: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn test_passport_validity2() {
        let invalid_passports = parse_input(INVALID_DATA).unwrap();
        assert_eq!(
            invalid_passports
                .iter()
                .filter(|p| validate_passport2(p).unwrap())
                .count(),
            0
        );

        let valid_passports = parse_input(VALID_DATA).unwrap();
        assert_eq!(
            valid_passports
                .iter()
                .map(|p| validate_passport2(p).unwrap())
                .count(),
            4
        );
    }
}
