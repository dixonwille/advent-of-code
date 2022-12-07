use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{anychar, char as character, digit1, line_ending},
    combinator::{all_consuming, map, map_parser, opt},
    multi::{fold_many1, many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

const INPUT: &str = include_str!("inputs/day05.txt");

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
struct Parsed {
    stack: Vec<Vec<char>>,
    moves: Vec<(usize, usize, usize)>,
}

fn fold_cargo_row(mut acc: Vec<Vec<char>>, row: Vec<Option<char>>) -> Vec<Vec<char>> {
    if acc.is_empty() {
        acc.resize(row.len(), Vec::new());
    }
    for (i, c) in row.into_iter().enumerate() {
        if c.is_none() {
            continue;
        }
        acc[i].push(c.unwrap())
    }
    acc
}

fn start_stack(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let empty_cargo = map(tag("   "), |_| None);
    let cargo = map(delimited(character('['), anychar, character(']')), |c| {
        Some(c)
    });
    let maybe_cargo = map_parser(take(3u8), alt((cargo, empty_cargo)));
    let cargo_row = terminated(separated_list1(character(' '), maybe_cargo), line_ending);

    let stack_num = delimited(character(' '), digit1, character(' '));
    let stack_num_row = terminated(separated_list1(character(' '), stack_num), line_ending);

    map(
        terminated(
            fold_many1(cargo_row, Vec::new, fold_cargo_row),
            stack_num_row,
        ),
        |res| {
            res.into_iter()
                .map(|mut stack| {
                    stack.reverse();
                    stack
                })
                .collect()
        },
    )(input)
}

fn arrange_instructions(input: &str) -> IResult<&str, Vec<(usize, usize, usize)>> {
    let arrange_move = delimited(tag("move "), digit1::<&str, _>, character(' '));
    let arrange_from = delimited(tag("from "), digit1, character(' '));
    let arrange_to = preceded(tag("to "), digit1);

    let arrange_instruction = terminated(
        map(
            tuple((arrange_move, arrange_from, arrange_to)),
            |(m, f, t)| {
                (
                    m.parse().unwrap(),
                    f.parse::<usize>().unwrap() - 1,
                    t.parse::<usize>().unwrap() - 1,
                )
            },
        ),
        line_ending,
    );

    many1(arrange_instruction)(input)
}

fn parse(input: &str) -> Result<Parsed> {
    let (_, res) = map(
        all_consuming(tuple((
            terminated(start_stack, line_ending),
            terminated(arrange_instructions, opt(line_ending)),
        ))),
        |(stack, moves)| Parsed { stack, moves },
    )(input)
    .map_err(|e| {
        dbg!(&e);
        e.to_owned()
    })?;
    Ok(res)
}

fn part_a(mut cargo: Parsed) -> Result<String> {
    for (how_many, from, to) in cargo.moves {
        for _i in 0..how_many {
            let c = cargo.stack[from].pop();
            cargo.stack[to].push(c.unwrap());
        }
    }
    Ok(cargo
        .stack
        .into_iter()
        .filter_map(|s| match s.last() {
            Some(&c) => Some(c),
            None => None,
        })
        .collect::<String>())
}

fn part_b(mut cargo: Parsed) -> Result<String> {
    for (how_many, from, to) in cargo.moves {
        let from_len = cargo.stack[from].len();
        let pop = cargo.stack[from]
            .splice(from_len - how_many..from_len, Vec::new())
            .collect::<Vec<_>>();
        cargo.stack[to].extend(pop);
    }
    Ok(cargo
        .stack
        .into_iter()
        .filter_map(|s| match s.last() {
            Some(&c) => Some(c),
            None => None,
        })
        .collect::<String>())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse(TEST_INPUT).unwrap(),
            Parsed {
                stack: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
                moves: vec![(1, 1, 0), (3, 0, 2), (2, 1, 0), (1, 0, 1)]
            }
        )
    }

    #[test]
    fn test_part_a() {
        let parsed = parse(TEST_INPUT).unwrap();
        assert_eq!(part_a(parsed).unwrap(), "CMZ");
    }

    #[test]
    fn test_part_b() {
        let parsed = parse(TEST_INPUT).unwrap();
        assert_eq!(part_b(parsed).unwrap(), "MCD");
    }
}
