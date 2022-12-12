use std::fmt::Write;

use anyhow::{Ok, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char as character, digit1, line_ending},
    combinator::{map, opt},
    multi::many1,
    sequence::{pair, separated_pair, terminated},
    IResult,
};

const INPUT: &str = include_str!("inputs/day10.txt");

pub fn run_part_a() -> Result<()> {
    let i = parse(INPUT)?;
    println!("{}", part_a(i)?);
    Ok(())
}

pub fn run_part_b() -> Result<()> {
    let i = parse(INPUT)?;
    println!("{}", part_b(i)?);
    Ok(())
}

#[derive(Debug, PartialEq)]
enum Command {
    Noop,
    Addx(isize),
}

type Parsed = Vec<Command>;

fn command(input: &str) -> IResult<&str, Command> {
    let noop = map(tag("noop"), |_| Command::Noop);
    let addx = map(
        separated_pair(
            tag("addx"),
            character::<&str, _>(' '),
            pair(opt(character('-')), digit1),
        ),
        |(_, (neg, v))| {
            let mut val = v.parse::<isize>().unwrap();
            if neg.is_some() {
                val *= -1;
            }
            Command::Addx(val)
        },
    );
    terminated(alt((noop, addx)), line_ending)(input)
}

fn parse(input: &str) -> Result<Parsed> {
    let (_, res) = many1(command)(input).map_err(|e| e.to_owned())?;
    Ok(res)
}

fn part_a(commands: Parsed) -> Result<isize> {
    let mut states = commands
        .into_iter()
        .scan((1isize, 0usize), |(reg_x, cycle), c| match c {
            Command::Noop => {
                *cycle += 1;
                Some((*cycle, *reg_x))
            }
            Command::Addx(r) => {
                *cycle += 2;
                *reg_x += r;
                Some((*cycle, *reg_x))
            }
        })
        .collect::<Vec<_>>();
    states.reverse();
    Ok(vec![
        states.iter().find(|s| s.0 < 20).unwrap().1 * 20,
        states.iter().find(|s| s.0 < 60).unwrap().1 * 60,
        states.iter().find(|s| s.0 < 100).unwrap().1 * 100,
        states.iter().find(|s| s.0 < 140).unwrap().1 * 140,
        states.iter().find(|s| s.0 < 180).unwrap().1 * 180,
        states.iter().find(|s| s.0 < 220).unwrap().1 * 220,
    ]
    .iter()
    .sum::<isize>())
}

fn part_b(commands: Parsed) -> Result<String> {
    let mut states = commands
        .into_iter()
        .scan((1isize, 0usize), |(reg_x, cycle), c| match c {
            Command::Noop => {
                *cycle += 1;
                Some((*cycle, *reg_x))
            }
            Command::Addx(r) => {
                *cycle += 2;
                *reg_x += r;
                Some((*cycle, *reg_x))
            }
        })
        .collect::<Vec<_>>();
    states.reverse();
    states.push((1, 1));
    let mut s = String::new();
    let mut state = states.pop().unwrap();
    let mut next_state = states.pop();
    for i in 0..240 {
        if i > 0 && i % 40 == 0 {
            s.write_char('\n')?;
        }
        if (state.1 - 1..=state.1 + 1).contains(&((i % 40) as isize)) {
            s.write_char('#')?;
        } else {
            s.write_char('.')?;
        }
        if next_state.is_some() && i + 1 >= next_state.unwrap().0 {
            state = next_state.unwrap();
            next_state = states.pop();
        }
    }
    s.write_char('\n')?;
    Ok(s)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_PARSE: &str = "noop
addx 3
addx -5

";

    const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop

";

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(TEST_INPUT_PARSE).unwrap(),
            vec![Command::Noop, Command::Addx(3), Command::Addx(-5)]
        )
    }

    #[test]
    fn test_part_a() {
        let parsed = parse(TEST_INPUT).unwrap();
        assert_eq!(part_a(parsed).unwrap(), 13140);
    }

    const EXPECTED_OUTPUT: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

    #[test]
    fn test_part_b() {
        let parsed = parse(TEST_INPUT).unwrap();
        assert_eq!(part_b(parsed).unwrap(), EXPECTED_OUTPUT);
    }
}
