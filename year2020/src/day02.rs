/// https://adventofcode.com/2020/day/2
use std::ops::Deref;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char as c, digit1, line_ending},
    combinator::{all_consuming, map, map_res},
    multi::separated_list1,
    IResult,
};

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
    // We can use u8 for this because of our data set.
    // real world, correct way, would be to use char or glyphs
    // but that is less performant due to nature of how it has to be figured out
    value: Vec<u8>,
}

impl Password {
    pub fn new(req1: usize, req2: usize, character: char, value: Vec<u8>) -> Self {
        Password {
            policy: Policy::new(req1, req2, character),
            value,
        }
    }
}

impl Deref for Password {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Password> {
    let (_, passwords) = parse_input_nom(input).unwrap();
    passwords
}

#[inline]
fn take_digits(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse())(input)
}

#[inline]
fn take_alpha_bytes(input: &str) -> IResult<&str, Vec<u8>> {
    map(alpha1, |s: &str| s.as_bytes().to_owned())(input)
}

#[inline]
fn take_password(input: &str) -> IResult<&str, Password> {
    let (input, req1) = take_digits(input)?;
    let (input, _) = c('-')(input)?;
    let (input, req2) = take_digits(input)?;
    let (input, _) = c(' ')(input)?;
    let (input, character) = anychar(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, value) = take_alpha_bytes(input)?;
    Ok((input, Password::new(req1, req2, character, value)))
}

#[inline]
fn parse_input_nom(input: &str) -> IResult<&str, Vec<Password>> {
    all_consuming(separated_list1(line_ending, take_password))(input)
}

#[aoc(day2, part1)]
fn part1(passwords: &[Password]) -> usize {
    passwords.iter().fold(0, |valid, password| {
        let count = password.iter().fold(0, |count, c| {
            if *c as char == password.policy.character {
                return count + 1;
            }
            count
        });
        if count >= password.policy.req1 && count <= password.policy.req2 {
            return valid + 1;
        }
        valid
    })
}

#[aoc(day2, part2)]
fn part2(passwords: &[Password]) -> usize {
    passwords.iter().fold(0, |valid, password| {
        let v1 = password.get(password.policy.req1 - 1);
        let v2 = password.get(password.policy.req2 - 1);
        match (v1, v2) {
            (None, None) => valid,
            (Some(r1), Some(r2)) => {
                if *r1 as char == password.policy.character
                    && *r2 as char == password.policy.character
                {
                    valid // don't increment as this is invalid
                } else if *r1 as char == password.policy.character
                    || *r2 as char == password.policy.character
                {
                    valid + 1
                } else {
                    valid // don't increment as this is invalid
                }
            }
            (Some(r1), None) => {
                if *r1 as char == password.policy.character {
                    valid + 1
                } else {
                    valid // don't increment as this is invalid
                }
            }
            (None, Some(r2)) => {
                if *r2 as char == password.policy.character {
                    valid + 1
                } else {
                    valid // don't increment as this is invalid
                }
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    static PASSWORDS: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn parsing_input() {
        assert_eq!(
            parse_input(PASSWORDS),
            vec![
                Password::new(1, 3, 'a', b"abcde".to_vec()),
                Password::new(1, 3, 'b', b"cdefg".to_vec()),
                Password::new(2, 9, 'c', b"ccccccccc".to_vec()),
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
