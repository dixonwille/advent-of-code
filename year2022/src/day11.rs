use std::collections::HashMap;

use anyhow::Result;
use pest_consume::{match_nodes, Parser};

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

enum Operator {
    Add,
    Mul,
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

#[derive(Parser)]
#[grammar = "pegs/day11.pest"]
struct Day11Parser;

type Node<'i> = pest_consume::Node<'i, Rule, ()>;
type PResult<T> = std::result::Result<T, pest_consume::Error<Rule>>;

#[pest_consume::parser]
impl Day11Parser {
    fn file(input: Node) -> PResult<Parsed> {
        let monkeys = match_nodes!(input.into_children();
            [monkeys(mks), EOI(_)] => mks
        );
        Ok(monkeys.into_iter().fold(HashMap::new(), |mut acc, m| {
            acc.insert(m.name, m);
            acc
        }))
    }

    fn monkeys(input: Node) -> PResult<Vec<Monkey>> {
        Ok(match_nodes!(input.into_children();
            [monkey(mk)..] => mk.collect()
        ))
    }

    fn monkey(input: Node) -> PResult<Monkey> {
        Ok(match_nodes!(input.into_children();
            [monkey_name(name),
             monkey_items(items),
             monkey_operation(operation),
             monkey_test(test_div_by),
             monkey_truthy(truthy),
             monkey_falsy(falsy)] => Monkey {
                name,
                items,
                operation,
                test_div_by,
                truthy,
                falsy,
                inspected: 0
            }
        ))
    }

    fn monkey_name(input: Node) -> PResult<usize> {
        Ok(match_nodes!(input.into_children();
            [number(n)] => n
        ))
    }

    fn monkey_items(input: Node) -> PResult<Vec<usize>> {
        Ok(match_nodes!(input.into_children();
            [number_list(l)] => l
        ))
    }

    fn monkey_operation(input: Node) -> PResult<Expression> {
        Ok(match_nodes!(input.into_children();
            [op_expression(e)] => e
        ))
    }

    fn monkey_test(input: Node) -> PResult<usize> {
        Ok(match_nodes!(input.into_children();
            [number(n)] => n
        ))
    }

    fn monkey_truthy(input: Node) -> PResult<usize> {
        Ok(match_nodes!(input.into_children();
            [number(n)] => n
        ))
    }

    fn monkey_falsy(input: Node) -> PResult<usize> {
        Ok(match_nodes!(input.into_children();
            [number(n)] => n
        ))
    }

    fn number_list(input: Node) -> PResult<Vec<usize>> {
        Ok(match_nodes!(input.into_children();
            [number(n)..] => n.collect()
        ))
    }

    fn op_expression(input: Node) -> PResult<Expression> {
        Ok(match_nodes!(input.into_children();
            [op_value(l), op_operator(o), op_value(r)] => match o {
                Operator::Add => Expression::Add(l, r),
                Operator::Mul => Expression::Mul(l, r),
            }
        ))
    }

    fn op_value(input: Node) -> PResult<Value> {
        Ok(match_nodes!(input.into_children();
            [old(v)] => v,
            [number(n)] => Value::Const(n)
        ))
    }

    fn old(_input: Node) -> PResult<Value> {
        Ok(Value::Old)
    }

    fn op_operator(input: Node) -> PResult<Operator> {
        Ok(match_nodes!(input.into_children();
            [add(o)] => o,
            [mult(o)] => o
        ))
    }

    fn add(_input: Node) -> PResult<Operator> {
        Ok(Operator::Add)
    }

    fn mult(_input: Node) -> PResult<Operator> {
        Ok(Operator::Mul)
    }

    fn number(input: Node) -> PResult<usize> {
        input.as_str().parse().map_err(|e| input.error(e))
    }

    fn EOI(_input: Node) -> PResult<()> {
        Ok(())
    }
}

fn parse(input: &str) -> Result<Parsed> {
    let inputs = Day11Parser::parse(Rule::file, input)?;
    let input = inputs.single()?;
    Day11Parser::file(input).map_err(|e| e.into())
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
