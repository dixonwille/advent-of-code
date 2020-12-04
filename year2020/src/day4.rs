/// https://adventofcode.com/2020/day/4
use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::{char as c, newline, one_of},
    combinator::{map_res, opt},
    multi::many1,
    sequence::terminated,
    IResult,
};

#[derive(Eq, PartialEq, Hash)]
enum PassportField {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportId,
    CountryId,
}

impl PassportField {
    fn from_str(s: &str) -> Result<Self, &str> {
        match s {
            "byr" => Ok(PassportField::BirthYear),
            "iyr" => Ok(PassportField::IssueYear),
            "eyr" => Ok(PassportField::ExpirationYear),
            "hgt" => Ok(PassportField::Height),
            "hcl" => Ok(PassportField::HairColor),
            "ecl" => Ok(PassportField::EyeColor),
            "pid" => Ok(PassportField::PassportId),
            "cid" => Ok(PassportField::CountryId),
            _ => Err("not a valid passport field key"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Passport {
    birth_year: Option<usize>,
    issue_year: Option<usize>,
    expiration_year: Option<usize>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<usize>,
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Passport> {
    let (_, passports) = parse_input_nom(input).unwrap();
    passports
}

fn parse_field(input: &str) -> IResult<&str, (PassportField, &str)> {
    let (input, key) = map_res(
        alt((
            tag("byr"),
            tag("iyr"),
            tag("eyr"),
            tag("hgt"),
            tag("hcl"),
            tag("ecl"),
            tag("pid"),
            tag("cid"),
        )),
        |key: &str| PassportField::from_str(key),
    )(input)?;
    let (input, _) = c(':')(input)?;
    let (input, value) = take_till1(|c| c == ' ' || c == '\n')(input)?;
    Ok((input, (key, value)))
}

fn parse_passport(input: &str) -> IResult<&str, Passport> {
    let (input, fields) = many1(terminated(parse_field, one_of(" \n")))(input)?;
    let mut map = HashMap::new();
    for (key, value) in fields {
        map.insert(key, value);
    }
    Ok((
        input,
        Passport {
            birth_year: map
                .get(&PassportField::BirthYear)
                .map(|s| s.parse().unwrap()),
            issue_year: map
                .get(&PassportField::IssueYear)
                .map(|s| s.parse().unwrap()),
            expiration_year: map
                .get(&PassportField::ExpirationYear)
                .map(|s| s.parse().unwrap()),
            height: map.get(&PassportField::Height).map(|s| s.to_string()),
            hair_color: map.get(&PassportField::HairColor).map(|s| s.to_string()),
            eye_color: map.get(&PassportField::EyeColor).map(|s| s.to_string()),
            passport_id: map.get(&PassportField::PassportId).map(|s| s.to_string()),
            country_id: map
                .get(&PassportField::CountryId)
                .map(|s| s.parse().unwrap()),
        },
    ))
}

fn parse_input_nom(input: &str) -> IResult<&str, Vec<Passport>> {
    many1(terminated(parse_passport, opt(newline)))(input)
}

#[aoc(day4, part1)]
fn part1(passports: &Vec<Passport>) -> usize {
    println!("{:?}", passports.len());
    passports.into_iter().fold(0, |valid, passport| {
        if passport.birth_year.is_some()
            && passport.issue_year.is_some()
            && passport.expiration_year.is_some()
            && passport.height.is_some()
            && passport.hair_color.is_some()
            && passport.eye_color.is_some()
            && passport.passport_id.is_some()
        {
            valid + 1
        } else {
            valid
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    static PASSPORTS: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";

    #[test]
    fn parsing_input() {
        assert_eq!(
            parse_input(PASSPORTS),
            vec![
                Passport {
                    birth_year: Some(1937),
                    issue_year: Some(2017),
                    expiration_year: Some(2020),
                    height: Some("183cm".to_owned()),
                    hair_color: Some("#fffffd".to_owned()),
                    eye_color: Some("gry".to_owned()),
                    passport_id: Some("860033327".to_owned()),
                    country_id: Some(147)
                },
                Passport {
                    birth_year: Some(1929),
                    issue_year: Some(2013),
                    expiration_year: Some(2023),
                    height: None,
                    hair_color: Some("#cfa07d".to_owned()),
                    eye_color: Some("amb".to_owned()),
                    passport_id: Some("028048884".to_owned()),
                    country_id: Some(350)
                },
                Passport {
                    birth_year: Some(1931),
                    issue_year: Some(2013),
                    expiration_year: Some(2024),
                    height: Some("179cm".to_owned()),
                    hair_color: Some("#ae17e1".to_owned()),
                    eye_color: Some("brn".to_owned()),
                    passport_id: Some("760753108".to_owned()),
                    country_id: None
                },
                Passport {
                    birth_year: None,
                    issue_year: Some(2011),
                    expiration_year: Some(2025),
                    height: Some("59in".to_owned()),
                    hair_color: Some("#cfa07d".to_owned()),
                    eye_color: Some("brn".to_owned()),
                    passport_id: Some("166559648".to_owned()),
                    country_id: None
                },
            ]
        )
    }

    #[test]
    fn running_part1() {
        let passports = parse_input(PASSPORTS);
        assert_eq!(part1(&passports), 2)
    }
}
