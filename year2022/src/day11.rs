use std::collections::HashMap;

use anyhow::{Ok, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char as character, digit1, line_ending},
    combinator::{map, map_res, opt},
    multi::{many1, separated_list1},
    sequence::{delimited, terminated, tuple},
    IResult,
};

const INPUT: &str = include_str!("inputs/day11.txt");

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
struct Monkey {
    name: usize,
    items: Vec<usize>,
    operation: Expression,
    test_div_by: usize,
    truthy: usize,
    falsy: usize,
    inspected: usize,
}

impl Monkey {
    fn inspect(&mut self, item: usize) -> usize {
        self.inspected += 1;
        self.operation.evaluate(item)
    }
}

#[derive(Debug, PartialEq)]
enum Expression {
    Add(Value, Value),
    Mul(Value, Value),
}

impl Expression {
    fn evaluate(&self, item: usize) -> usize {
        match self {
            Expression::Add(l, r) => l.get(item) + r.get(item),
            Expression::Mul(l, r) => l.get(item) * r.get(item),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Value {
    Const(usize),
    Old,
}

impl Value {
    fn get(&self, item: usize) -> usize {
        match self {
            Value::Const(c) => *c,
            Value::Old => item,
        }
    }
}

type Parsed = HashMap<usize, Monkey>;

fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

fn operator_value(input: &str) -> IResult<&str, Value> {
    map_res(alt((tag("old"), digit1)), |v| {
        if v == "old" {
            Ok(Value::Old)
        } else {
            Ok(Value::Const(str::parse(v)?))
        }
    })(input)
}

fn expression(input: &str) -> IResult<&str, Expression> {
    let operator = alt((character('*'), character('+')));
    map(
        tuple((
            terminated(operator_value, character(' ')),
            terminated(operator, character(' ')),
            operator_value,
        )),
        |(l, o, r)| match o {
            '*' => Expression::Mul(l, r),
            '+' => Expression::Add(l, r),
            _ => unreachable!(),
        },
    )(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let number_list = separated_list1(tag(", "), number);

    let name = delimited(tag("Monkey "), number, tag(":\n"));
    let starting = delimited(tag("  Starting items: "), number_list, line_ending);
    let operation = delimited(tag("  Operation: new = "), expression, line_ending);
    let test = delimited(tag("  Test: divisible by "), number, line_ending);
    let truthy = delimited(tag("    If true: throw to monkey "), number, line_ending);
    let falsy = delimited(tag("    If false: throw to monkey "), number, line_ending);
    map(
        terminated(
            tuple((name, starting, operation, test, truthy, falsy)),
            opt(line_ending),
        ),
        |(name, items, operation, test_div_by, truthy, falsy)| Monkey {
            name,
            items,
            operation,
            test_div_by,
            truthy,
            falsy,
            inspected: 0,
        },
    )(input)
}

fn parse(input: &str) -> Result<Parsed> {
    let (_, res) = many1(parse_monkey)(input).map_err(|e| e.to_owned())?;
    Ok(res.into_iter().fold(HashMap::new(), |mut acc, m| {
        acc.insert(m.name, m);
        acc
    }))
}

fn part_a(mut monkeys: Parsed) -> Result<usize> {
    for _round in 0..20 {
        for turn in 0..monkeys.len() {
            let tm;
            let fm;
            let mut truthy: Vec<usize> = Vec::new();
            let mut falsy: Vec<usize> = Vec::new();
            {
                let monkey = monkeys.get_mut(&turn).unwrap();
                tm = monkey.truthy;
                fm = monkey.falsy;
                monkey.items.reverse();
                while let Some(item) = monkey.items.pop() {
                    let worry = monkey.inspect(item) / 3;
                    match worry % monkey.test_div_by == 0 {
                        true => truthy.push(worry),
                        false => falsy.push(worry),
                    }
                }
            }
            {
                let monkey = monkeys.get_mut(&tm).unwrap();
                monkey.items.extend(truthy);
            }
            {
                let monkey = monkeys.get_mut(&fm).unwrap();
                monkey.items.extend(falsy);
            }
        }
    }
    let mut inspections = monkeys.iter().map(|(_, m)| m.inspected).collect::<Vec<_>>();
    inspections.sort();
    inspections.reverse();
    Ok(inspections.iter().take(2).product())
}

fn part_b(mut monkeys: Parsed) -> Result<usize> {
    let common: usize = monkeys.iter().map(|(_, m)| m.test_div_by).product();
    for _round in 0..10_000 {
        for turn in 0..monkeys.len() {
            let tm;
            let fm;
            let mut truthy: Vec<usize> = Vec::new();
            let mut falsy: Vec<usize> = Vec::new();
            {
                let monkey = monkeys.get_mut(&turn).unwrap();
                tm = monkey.truthy;
                fm = monkey.falsy;
                monkey.items.reverse();
                while let Some(item) = monkey.items.pop() {
                    let worry = monkey.inspect(item) % common;
                    match worry % monkey.test_div_by == 0 {
                        true => truthy.push(worry),
                        false => falsy.push(worry),
                    }
                }
            }
            {
                let monkey = monkeys.get_mut(&tm).unwrap();
                monkey.items.extend(truthy);
            }
            {
                let monkey = monkeys.get_mut(&fm).unwrap();
                monkey.items.extend(falsy);
            }
        }
    }
    let mut inspections = monkeys.iter().map(|(_, m)| m.inspected).collect::<Vec<_>>();
    inspections.sort();
    inspections.reverse();
    Ok(inspections.iter().take(2).product())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1

";

    #[test]
    fn test_parse() {
        let mut map = HashMap::new();
        map.insert(
            0,
            Monkey {
                name: 0,
                items: vec![79, 98],
                operation: Expression::Mul(Value::Old, Value::Const(19)),
                test_div_by: 23,
                truthy: 2,
                falsy: 3,
                inspected: 0,
            },
        );
        map.insert(
            1,
            Monkey {
                name: 1,
                items: vec![54, 65, 75, 74],
                operation: Expression::Add(Value::Old, Value::Const(6)),
                test_div_by: 19,
                truthy: 2,
                falsy: 0,
                inspected: 0,
            },
        );
        map.insert(
            2,
            Monkey {
                name: 2,
                items: vec![79, 60, 97],
                operation: Expression::Mul(Value::Old, Value::Old),
                test_div_by: 13,
                truthy: 1,
                falsy: 3,
                inspected: 0,
            },
        );
        map.insert(
            3,
            Monkey {
                name: 3,
                items: vec![74],
                operation: Expression::Add(Value::Old, Value::Const(3)),
                test_div_by: 17,
                truthy: 0,
                falsy: 1,
                inspected: 0,
            },
        );
        assert_eq!(parse(TEST_INPUT).unwrap(), map);
    }

    #[test]
    fn test_part_a() {
        let parsed = parse(TEST_INPUT).unwrap();
        assert_eq!(part_a(parsed).unwrap(), 10605);
    }

    #[test]
    fn test_part_b() {
        let parsed = parse(TEST_INPUT).unwrap();
        assert_eq!(part_b(parsed).unwrap(), 2713310158);
    }
}
