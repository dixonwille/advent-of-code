/// https://adventofcode.com/2020/day/8
use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char as c, digit1, one_of},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Operation {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl Operation {
    // modifies instruction on the next thing to run and accumulator for this operation
    fn next(&self, (instruction, accumulator): &mut(isize, isize)) {
        match self {
            Operation::Acc(inc) => {
                *accumulator += inc;
                *instruction += 1;
            }
            Operation::Jmp(offset) => *instruction += offset,
            Operation::Nop(_) => *instruction += 1,
        }
    }
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<Operation> {
    let (_, ops) = parse_input_nom(input).unwrap();
    ops
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    map(
        separated_pair(
            alt((tag("acc"), tag("jmp"), tag("nop"))),
            c(' '),
            tuple((one_of("+-"), digit1)),
        ),
        |(op, (sign, num))| {
            match op {
                "acc" => Operation::Acc((sign.to_string()+num).parse().unwrap()),
                "jmp" => Operation::Jmp((sign.to_string()+num).parse().unwrap()),
                "nop" => Operation::Nop((sign.to_string()+num).parse().unwrap()),
                _ => unreachable!("parsing on only the accepted values")
            }
        },
    )(input)
}

fn parse_input_nom(input: &str) -> IResult<&str, Vec<Operation>> {
    all_consuming(separated_list1(c('\n'), parse_operation))(input)
}

#[aoc(day8, part1)]
fn part1(program: &[Operation]) -> isize {
    match run_program(program) {
        Ok(acc) => acc,
        Err((_, acc)) => acc
    }
}

fn run_program(program: &[Operation]) -> Result<isize, (isize, isize)> {
    let mut instructions_seen = HashSet::new();
    let mut registers:(isize, isize) = (0, 0);
    let prog_length = program.len();

    // TODO this is for debugging purposes only remove when done.
    let mut instructions_order = Vec::new();
    
    loop {
        if instructions_seen.contains(&registers.0) || registers.0 < 0 {
            eprintln!("{:?}", instructions_order);
            return Err(registers);
        }
        if registers.0 as usize >= prog_length {
            break;
        }
        instructions_seen.insert(registers.0);
        instructions_order.push((registers.0, program[registers.0 as usize].clone()));
        program[registers.0 as usize].next(&mut registers);
        
    };
    Ok(registers.1)
}

#[aoc(day8, part2)]
fn part2(program: &[Operation]) -> isize {
    match run_program(program) {
        Ok(acc) => acc,
        Err((inst, acc)) => {
            println!("instruction: {:?}", inst);
            acc
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    static PROGRAM: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn parsring_input() {
        assert_eq!(
            parse_input(PROGRAM),
            vec![
                Operation::Nop(0),
                Operation::Acc(1),
                Operation::Jmp(4),
                Operation::Acc(3),
                Operation::Jmp(-3),
                Operation::Acc(-99),
                Operation::Acc(1),
                Operation::Jmp(-4),
                Operation::Acc(6)
            ]
        );
    }

    #[test]
    fn running_part1() {
        let input = parse_input(PROGRAM);
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn running_part2() {
        let input = parse_input(PROGRAM);
        assert_eq!(part2(&input), 8);
    }
}
