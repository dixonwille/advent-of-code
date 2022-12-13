use anyhow::Result;
use pest_consume::{match_nodes, Parser};

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

#[derive(Parser)]
#[grammar = "pegs/day05.pest"]
struct Day05Parser;

type Node<'i> = pest_consume::Node<'i, Rule, ()>;
type PResult<T> = std::result::Result<T, pest_consume::Error<Rule>>;

#[pest_consume::parser]
impl Day05Parser {
    fn file(input: Node) -> PResult<Parsed> {
        Ok(match_nodes!(input.into_children();
            [initial_header(stack), instructions(moves), EOI(_)] => Parsed{stack, moves}
        ))
    }

    fn instructions(input: Node) -> PResult<Vec<(usize, usize, usize)>> {
        Ok(match_nodes!(input.into_children();
            [instruction(inst)..] => inst.collect()
        ))
    }

    fn instruction(input: Node) -> PResult<(usize, usize, usize)> {
        Ok(match_nodes!(input.into_children();
            [number(count), number(from), number(to)] => (count, from-1, to-1)
        ))
    }

    fn number(input: Node) -> PResult<usize> {
        input.as_str().parse().map_err(|e| input.error(e))
    }

    fn initial_header(input: Node) -> PResult<Vec<Vec<char>>> {
        let cargo_rows = match_nodes!(input.into_children();
            [cargo_rows(rows), stack_row(_)] => rows
        );
        Ok(cargo_rows
            .into_iter()
            .fold(Vec::new(), fold_cargo_row)
            .into_iter()
            .map(|mut stack| {
                stack.reverse();
                stack
            })
            .collect())
    }

    fn cargo_rows(input: Node) -> PResult<Vec<Vec<Option<char>>>> {
        Ok(match_nodes!(input.into_children();
            [cargo_row(row)..] => row.collect()
        ))
    }

    fn cargo_row(input: Node) -> PResult<Vec<Option<char>>> {
        Ok(match_nodes!(input.into_children();
            [cargo(cargo)..] => cargo.collect()
        ))
    }

    fn cargo(input: Node) -> PResult<Option<char>> {
        Ok(match_nodes!(input.into_children();
            [cargo_name(cargo)] => cargo,
            [empty_cargo(cargo)] => cargo
        ))
    }

    fn cargo_name(input: Node) -> PResult<Option<char>> {
        Ok(Some(
            input
                .as_str()
                .chars()
                .next()
                .ok_or(anyhow::Error::msg("cargo_name should have one character"))
                .map_err(|e| input.error(e))?,
        ))
    }

    fn empty_cargo(input: Node) -> PResult<Option<char>> {
        Ok(None)
    }

    fn stack_row(_input: Node) -> PResult<()> {
        Ok(())
    }

    fn EOI(_input: Node) -> PResult<()> {
        Ok(())
    }
}

fn parse(input: &str) -> Result<Parsed> {
    let inputs = Day05Parser::parse(Rule::file, input)?;
    let input = inputs.single()?;
    Day05Parser::file(input).map_err(|e| e.into())
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
        );
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
