use anyhow::{Error, Result};

const INPUT: &str = include_str!("inputs/day02.txt");

pub fn run_part_a() -> Result<()> {
    let i = parse_a(INPUT)?;
    println!("{}", part_a(i)?);
    Ok(())
}

pub fn run_part_b() -> Result<()> {
    let i = parse_b(INPUT)?;
    println!("{}", part_b(i)?);
    Ok(())
}

#[derive(Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Choice {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Choice::Rock),
            'B' | 'Y' => Ok(Choice::Paper),
            'C' | 'Z' => Ok(Choice::Scissors),
            _ => Err(Error::msg("Character does not match known choice")),
        }
    }
}

impl Choice {
    fn into_score(&self) -> isize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

type ParsedA = Vec<(Choice, Choice)>;

fn parse_a(input: &str) -> Result<ParsedA> {
    Ok(input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }
            let round = l.chars().collect::<Vec<_>>();
            Some((round[0].try_into().unwrap(), round[2].try_into().unwrap()))
        })
        .collect())
}

fn part_a(rounds: ParsedA) -> Result<isize> {
    Ok(rounds
        .into_iter()
        .map(|round| {
            round.1.into_score()
                + match (round.0, round.1) {
                    (Choice::Rock, Choice::Rock)
                    | (Choice::Paper, Choice::Paper)
                    | (Choice::Scissors, Choice::Scissors) => Resolution::Draw.into_score(),
                    (Choice::Rock, Choice::Paper)
                    | (Choice::Paper, Choice::Scissors)
                    | (Choice::Scissors, Choice::Rock) => Resolution::Win.into_score(),
                    (Choice::Rock, Choice::Scissors)
                    | (Choice::Paper, Choice::Rock)
                    | (Choice::Scissors, Choice::Paper) => Resolution::Lose.into_score(),
                }
        })
        .sum())
}

#[derive(Debug, PartialEq)]
enum Resolution {
    Win,
    Lose,
    Draw,
}

impl TryFrom<char> for Resolution {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Resolution::Lose),
            'Y' => Ok(Resolution::Draw),
            'Z' => Ok(Resolution::Win),
            _ => Err(Error::msg("Resolution not tied to character")),
        }
    }
}

impl Resolution {
    fn into_score(&self) -> isize {
        match self {
            Resolution::Win => 6,
            Resolution::Lose => 0,
            Resolution::Draw => 3,
        }
    }
}

type ParsedB = Vec<(Choice, Resolution)>;

fn parse_b(input: &str) -> Result<ParsedB> {
    Ok(input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }
            let round = l.chars().collect::<Vec<_>>();
            Some((round[0].try_into().unwrap(), round[2].try_into().unwrap()))
        })
        .collect())
}

fn part_b(rounds: ParsedB) -> Result<isize> {
    Ok(rounds
        .into_iter()
        .map(|round| {
            round.1.into_score()
                + match (round.0, round.1) {
                    (i, Resolution::Draw) => i.into_score(),
                    (Choice::Rock, Resolution::Win) | (Choice::Scissors, Resolution::Lose) => {
                        Choice::Paper.into_score()
                    }
                    (Choice::Rock, Resolution::Lose) | (Choice::Paper, Resolution::Win) => {
                        Choice::Scissors.into_score()
                    }
                    (Choice::Paper, Resolution::Lose) | (Choice::Scissors, Resolution::Win) => {
                        Choice::Rock.into_score()
                    }
                }
        })
        .sum())
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    const TEST_INPUT: &str = "A Y
B X
C Z

";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_a(TEST_INPUT).unwrap(),
            vec![
                (Choice::Rock, Choice::Paper),
                (Choice::Paper, Choice::Rock),
                (Choice::Scissors, Choice::Scissors)
            ]
        );
        assert_eq!(
            parse_b(TEST_INPUT).unwrap(),
            vec![
                (Choice::Rock, Resolution::Draw),
                (Choice::Paper, Resolution::Lose),
                (Choice::Scissors, Resolution::Win)
            ]
        );
    }

    #[test]
    fn test_part_a() {
        let parsed = parse_a(TEST_INPUT).unwrap();
        assert_eq!(part_a(parsed).unwrap(), 15);
    }

    #[test]
    fn test_part_b() {
        let parsed = parse_b(TEST_INPUT).unwrap();
        assert_eq!(part_b(parsed).unwrap(), 12);
    }
}
