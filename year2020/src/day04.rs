/// https://adventofcode.com/2020/day/4
use pest::Parser;
use std::{collections::HashMap, str::FromStr};

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

impl FromStr for PassportField {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

impl Passport {
    fn is_valid(&self) -> bool {
        // Must have all these fields
        if self.birth_year.is_none()
            || self.issue_year.is_none()
            || self.expiration_year.is_none()
            || self.height.is_none()
            || self.hair_color.is_none()
            || self.eye_color.is_none()
            || self.passport_id.is_none()
        {
            return false;
        }

        let birth_year = self.birth_year.unwrap();
        if birth_year < 1920 || birth_year > 2002 {
            return false;
        }

        let issue_year = self.issue_year.unwrap();
        if issue_year < 2010 || issue_year > 2020 {
            return false;
        }

        let expiration_year = self.expiration_year.unwrap();
        if expiration_year < 2020 || expiration_year > 2030 {
            return false;
        }

        let height = self.height.as_ref().unwrap();
        if height.ends_with("in") {
            if let Ok(h) = height.replace("in", "").parse::<usize>() {
                if h < 59 || h > 76 {
                    return false;
                }
            } else {
                return false;
            }
        } else if height.ends_with("cm") {
            if let Ok(h) = height.replace("cm", "").parse::<usize>() {
                if h < 150 || h > 193 {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }

        let hair_color = self.hair_color.as_ref().unwrap();
        if hair_color.starts_with('#') {
            let hair_color = hair_color.replace("#", "");
            if hair_color.len() != 6 {
                return false;
            }
            if u8::from_str_radix(&hair_color[0..2], 16).is_err()
                || u8::from_str_radix(&hair_color[2..4], 16).is_err()
                || u8::from_str_radix(&hair_color[4..6], 16).is_err()
            {
                return false;
            }
        } else {
            return false;
        }

        let eye_color = self.eye_color.as_ref().unwrap();
        if eye_color.len() == 3 {
            let valid_eye = matches!(
                eye_color.as_ref(),
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
            );
            if !valid_eye {
                return false;
            }
        } else {
            return false;
        }

        let passport_id = self.passport_id.as_ref().unwrap();
        if passport_id.len() == 9 {
            if passport_id.parse::<usize>().is_err() {
                return false;
            }
        } else {
            return false;
        }
        true
    }
}

#[derive(Parser)]
#[grammar = "pest/day04.pest"]
struct InputParser;

#[aoc_generator(day4, nom)]
fn parse_input(input: &str) -> Vec<Passport> {
    InputParser::parse(Rule::file, input)
        .expect("could not parse input")
        .filter(|r| r.as_rule() == Rule::passport)
        .map(|p| {
            let fields: HashMap<_, _> = p
                .into_inner()
                .map(|f| {
                    let mut field = f.into_inner();
                    let key = field
                        .next()
                        .unwrap()
                        .as_str()
                        .parse::<PassportField>()
                        .unwrap();
                    let value = field.next().unwrap().as_str();
                    (key, value)
                })
                .collect();
            Passport {
                birth_year: fields
                    .get(&PassportField::BirthYear)
                    .map(|s| s.parse().unwrap()),
                issue_year: fields
                    .get(&PassportField::IssueYear)
                    .map(|s| s.parse().unwrap()),
                expiration_year: fields
                    .get(&PassportField::ExpirationYear)
                    .map(|s| s.parse().unwrap()),
                height: fields.get(&PassportField::Height).map(|s| s.to_string()),
                hair_color: fields.get(&PassportField::HairColor).map(|s| s.to_string()),
                eye_color: fields.get(&PassportField::EyeColor).map(|s| s.to_string()),
                passport_id: fields
                    .get(&PassportField::PassportId)
                    .map(|s| s.to_string()),
                country_id: fields
                    .get(&PassportField::CountryId)
                    .map(|s| s.parse().unwrap()),
            }
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(passports: &[Passport]) -> usize {
    passports.iter().fold(0, |valid, passport| {
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

#[aoc(day4, part2)]
fn part2(passports: &[Passport]) -> usize {
    passports.iter().fold(0, |valid, passport| {
        if passport.is_valid() {
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
iyr:2011 ecl:brn hgt:59in";

    static MORE_PASSPORTS: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

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

    #[test]
    fn running_part2() {
        let passports = parse_input(MORE_PASSPORTS);
        assert_eq!(part2(&passports), 4);
    }
}
