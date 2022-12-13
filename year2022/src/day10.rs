use std::fmt::Write;

use anyhow::Result;
use pest_consume::{match_nodes, Parser};

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

#[derive(Parser)]
#[grammar = "pegs/day10.pest"]
struct Day10Parser;

type Node<'i> = pest_consume::Node<'i, Rule, ()>;
type PResult<T> = std::result::Result<T, pest_consume::Error<Rule>>;

#[pest_consume::parser]
impl Day10Parser {
    fn file(input: Node) -> PResult<Parsed> {
        Ok(match_nodes!(input.into_children();
            [commands(cmds), EOI(_)] => cmds
        ))
    }

    fn commands(input: Node) -> PResult<Parsed> {
        Ok(match_nodes!(input.into_children();
            [cmd(c)..] => c.collect()
        ))
    }

    fn cmd(input: Node) -> PResult<Command> {
        Ok(match_nodes!(input.into_children();
            [cmd_addx(cmd)] => cmd,
            [cmd_noop(cmd)] => cmd
        ))
    }

    fn cmd_addx(input: Node) -> PResult<Command> {
        Ok(match_nodes!(input.into_children();
            [number(n)] => Command::Addx(n)
        ))
    }

    fn cmd_noop(_input: Node) -> PResult<Command> {
        Ok(Command::Noop)
    }

    fn number(input: Node) -> PResult<isize> {
        input.as_str().parse().map_err(|e| input.error(e))
    }

    fn EOI(_input: Node) -> PResult<()> {
        Ok(())
    }
}

fn parse(input: &str) -> Result<Parsed> {
    let inputs = Day10Parser::parse(Rule::file, input)?;
    let input = inputs.single()?;
    Day10Parser::file(input).map_err(|e| e.into())
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
