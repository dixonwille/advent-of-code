/// https://adventofcode.com/2020/day/18
use std::fmt;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    character::complete::{char as c, digit1, multispace0, one_of},
    combinator::{cut, map, map_res},
    multi::many0,
    sequence::{delimited, pair},
    IResult,
};

enum Operation {
    Addition,
    Multiplication,
}

enum ExpressionType {
    Constante(u64),
    Addition(Expression, Expression),
    Multiplication(Expression, Expression),
}

struct Expression {
    expression: Box<ExpressionType>,
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self.expression {
            ExpressionType::Constante(ref val) => write!(f, "{}", val),
            ExpressionType::Addition(ref left, ref right) => {
                write!(f, "({:?} + {:?})", left, right)
            }
            ExpressionType::Multiplication(ref left, ref right) => {
                write!(f, "({:?} * {:?})", left, right)
            }
        }
    }
}

impl Expression {
    fn new(t: ExpressionType) -> Self {
        Expression {
            expression: Box::new(t),
        }
    }

    fn evaluate(&self) -> u64 {
        match *self.expression {
            ExpressionType::Constante(val) => val,
            ExpressionType::Addition(ref left, ref right) => left.evaluate() + right.evaluate(),
            ExpressionType::Multiplication(ref left, ref right) => {
                left.evaluate() * right.evaluate()
            }
        }
    }
}

fn parse_constant(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse())(input)
}

fn parse_operation_am(input: &str) -> IResult<&str, Operation> {
    map(one_of("*+"), |c| {
        if c == '+' {
            Operation::Addition
        } else {
            Operation::Multiplication
        }
    })(input)
}

fn parse_operation_add(input: &str) -> IResult<&str, Operation> {
    map(c('+'), |_| Operation::Addition)(input)
}

fn parse_operation_mult(input: &str) -> IResult<&str, Operation> {
    map(c('*'), |_| Operation::Multiplication)(input)
}

fn fold(acc: Expression, (op, val): (Operation, Expression)) -> Expression {
    match op {
        Operation::Addition => Expression::new(ExpressionType::Addition(acc, val)),
        Operation::Multiplication => Expression::new(ExpressionType::Multiplication(acc, val)),
    }
}

fn parse_parens_p1(input: &str) -> IResult<&str, Expression> {
    delimited(c('('), cut(parse_expression_p1), cut(c(')')))(input)
}

fn parse_parens_p2(input: &str) -> IResult<&str, Expression> {
    delimited(c('('), cut(parse_expression_p2), cut(c(')')))(input)
}

fn parse_factor_p1(input: &str) -> IResult<&str, Expression> {
    alt((
        delimited(multispace0, parse_parens_p1, multispace0),
        map(delimited(multispace0, parse_constant, multispace0), |d| {
            Expression::new(ExpressionType::Constante(d))
        }),
    ))(input)
}

fn parse_factor_p2(input: &str) -> IResult<&str, Expression> {
    alt((
        delimited(multispace0, parse_parens_p2, multispace0),
        map(delimited(multispace0, parse_constant, multispace0), |d| {
            Expression::new(ExpressionType::Constante(d))
        }),
    ))(input)
}

fn parse_term_p2(input: &str) -> IResult<&str, Expression> {
    let (input, mut init) = parse_factor_p2(input)?;
    let (input, rights) = many0(pair(parse_operation_add, cut(parse_factor_p2)))(input)?;
    for right in rights {
        init = fold(init, right)
    }
    Ok((input, init))
}

fn parse_expression_p1(input: &str) -> IResult<&str, Expression> {
    let (input, mut init) = parse_factor_p1(input)?;
    let (input, rights) = many0(pair(parse_operation_am, cut(parse_factor_p1)))(input)?;
    for right in rights {
        init = fold(init, right)
    }
    Ok((input, init))
}

fn parse_expression_p2(input: &str) -> IResult<&str, Expression> {
    let (input, mut init) = parse_term_p2(input)?;
    let (input, rights) = many0(pair(parse_operation_mult, cut(parse_term_p2)))(input)?;
    for right in rights {
        init = fold(init, right)
    }
    Ok((input, init))
}
#[aoc_generator(day18, part1)]
fn parse_input_p1(input: &str) -> Vec<Expression> {
    input
        .lines()
        .map(|l| {
            let (_, exp) = parse_expression_p1(l).unwrap();
            exp
        })
        .collect()
}

#[aoc_generator(day18, part2)]
fn parse_input_p2(input: &str) -> Vec<Expression> {
    input
        .lines()
        .map(|l| {
            let (_, exp) = parse_expression_p2(l).unwrap();
            exp
        })
        .collect()
}

#[aoc(day18, part1)]
fn part1(homework: &[Expression]) -> u64 {
    homework.iter().map(|e| e.evaluate()).sum()
}

#[aoc(day18, part2)]
fn part2(homework: &[Expression]) -> u64 {
    homework.iter().map(|e| e.evaluate()).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static HOMEWORK: &str = "1 + 2 * 3 + 4 * 5 + 6
1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn parsing_input() {
        assert_eq!(
            parse_input_p1(HOMEWORK)
                .iter()
                .map(|exp| format!("{:?}", exp))
                .collect::<Vec<_>>(),
            vec![
                "(((((1 + 2) * 3) + 4) * 5) + 6)",
                "((1 + (2 * 3)) + (4 * (5 + 6)))",
                "((2 * 3) + (4 * 5))",
                "(5 + (((((8 * 3) + 9) + 3) * 4) * 3))",
                "((5 * 9) * (((((7 * 3) * 3) + 9) * 3) + ((8 + 6) * 4)))",
                "(((((((2 + 4) * 9) * (((6 + 9) * 8) + 6)) + 6) + 2) + 4) * 2)"
            ]
        );
        assert_eq!(
            parse_input_p2(HOMEWORK)
                .iter()
                .map(|exp| format!("{:?}", exp))
                .collect::<Vec<_>>(),
            vec![
                "(((1 + 2) * (3 + 4)) * (5 + 6))",
                "((1 + (2 * 3)) + (4 * (5 + 6)))",
                "(2 * (3 + (4 * 5)))",
                "(5 + (((8 * ((3 + 9) + 3)) * 4) * 3))",
                "((5 * 9) * (((7 * 3) * (3 + 9)) * (3 + ((8 + 6) * 4))))",
                "((((((2 + 4) * 9) * (((6 + 9) * (8 + 6)) + 6)) + 2) + 4) * 2)"
            ]
        );
    }

    #[test]
    fn running_part1() {
        let homework = parse_input_p1(HOMEWORK);
        assert_eq!(part1(&homework), 26457)
    }

    #[test]
    fn running_part2() {
        let homework = parse_input_p2(HOMEWORK);
        assert_eq!(part2(&homework), 694173)
    }
}
