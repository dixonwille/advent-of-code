/// https://adventofcode.com/2020/day/2
use std::ops::Deref;

use aoc_runner_derive::{aoc, aoc_generator};
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
struct Policy {
    req1: usize,
    req2: usize,
    character: char,
}

impl Policy {
    fn new(req1: usize, req2: usize, character: char) -> Self {
        Policy {
            req1,
            req2,
            character,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Password {
    policy: Policy,
    value: String,
}

impl Password {
    pub fn new(req1: usize, req2: usize, character: char, value: String) -> Self {
        Password {
            policy: Policy::new(req1, req2, character),
            value,
        }
    }
}

impl Deref for Password {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

// #-# C: STRING
// ##-## C: LONGERSTRING
static PASSWORD_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?P<req1>[[:digit:]]+)-(?P<req2>[[:digit:]]+) (?P<character>[[:alpha:]]): (?P<value>[[:alpha:]]+)$").expect("valid regular expression")
});

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|l| {
            let caps = PASSWORD_REGEX.captures(l).expect("input in proper format");
            Password::new(
                caps.name("req1").unwrap().as_str().parse().unwrap(),
                caps.name("req2").unwrap().as_str().parse().unwrap(),
                caps.name("character").unwrap().as_str().parse().unwrap(),
                caps.name("value").unwrap().as_str().into(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(passwords: &Vec<Password>) -> usize {
    let mut valid = 0;
    for password in passwords {
        let mut count = 0;
        for c in password.chars() {
            if c == password.policy.character {
                count += 1;
            }
        }
        if count >= password.policy.req1 && count <= password.policy.req2 {
            valid += 1;
        }
    }
    valid
}

#[aoc(day2, part2)]
pub fn part2(passwords: &Vec<Password>) -> usize {
    let mut valid = 0;
    for password in passwords {
        let characters = password.chars().collect::<Vec<char>>();
        let v1 = characters.get(password.policy.req1-1);
        let v2 = characters.get(password.policy.req2-1);
        match (v1, v2) {
            (None, None) => continue,
            (Some(r1), Some(r2))=> {
                if r1 == &password.policy.character && r2 == &password.policy.character {
                    continue;
                }else if r1 == &password.policy.character || r2 == &password.policy.character {
                    valid += 1;
                }else {
                    continue;
                }
            },
            (Some(r1), None) =>{
                if r1 == &password.policy.character {
                    valid += 1;
                } else {
                    continue;
                }
            },
            (None, Some(r2)) => {
                if r2 == &password.policy.character {
                    valid += 1;
                } else {
                    continue;
                }
            }
        }
    }
    valid
}

#[allow(dead_code, unused_imports)]
mod test {
    use std::vec;

    use super::*;
    static PASSWORDS: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn parsing_input() {
        assert_eq!(
            parse_input(PASSWORDS),
            vec![
                Password::new(1, 3, 'a', "abcde".to_string()),
                Password::new(1, 3, 'b', "cdefg".to_string()),
                Password::new(2, 9, 'c', "ccccccccc".to_string()),
            ]
        );
    }

    #[test]
    fn running_part1() {
        let passwords = parse_input(PASSWORDS);
        assert_eq!(part1(&passwords), 2);
    }

    #[test]
    fn running_part2() {
        let passwords = parse_input(PASSWORDS);
        assert_eq!(part2(&passwords), 1);
    }
}
