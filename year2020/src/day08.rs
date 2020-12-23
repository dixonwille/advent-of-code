/// https://adventofcode.com/2020/day/8
use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char as c, digit1, one_of},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

use pathfinding::prelude::bfs;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Operation {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl Operation {
    // modifies instruction on the next thing to run and accumulator for this operation
    fn next(&self, (instruction, accumulator): &mut (isize, isize), flip: Option<isize>) {
        match (self, flip) {
            (Operation::Acc(inc), _) => {
                *accumulator += inc;
                *instruction += 1;
            }
            (Operation::Jmp(_), Some(f)) if f == *instruction => *instruction += 1,
            (Operation::Jmp(offset), None) | (Operation::Jmp(offset), Some(_)) => {
                *instruction += offset
            }
            (Operation::Nop(offset), Some(f)) if f == *instruction => *instruction += offset,
            (Operation::Nop(_), None) | (Operation::Nop(_), Some(_)) => *instruction += 1,
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
        |(op, (sign, num))| match op {
            "acc" => Operation::Acc((sign.to_string() + num).parse().unwrap()),
            "jmp" => Operation::Jmp((sign.to_string() + num).parse().unwrap()),
            "nop" => Operation::Nop((sign.to_string() + num).parse().unwrap()),
            _ => unreachable!("parsing on only the accepted values"),
        },
    )(input)
}

fn parse_input_nom(input: &str) -> IResult<&str, Vec<Operation>> {
    all_consuming(separated_list1(c('\n'), parse_operation))(input)
}

#[aoc(day8, part1)]
fn part1(program: &[Operation]) -> isize {
    match run_program(program, None) {
        Ok(acc) => acc,
        Err((_, acc)) => acc,
    }
}

fn run_program(program: &[Operation], flip: Option<isize>) -> Result<isize, (isize, isize)> {
    let mut instructions_seen = HashSet::new();
    let mut registers: (isize, isize) = (0, 0);
    let prog_length = program.len();

    loop {
        if instructions_seen.contains(&registers.0) || registers.0 < 0 {
            return Err(registers);
        }
        if registers.0 as usize >= prog_length {
            break;
        }
        instructions_seen.insert(registers.0);
        program[registers.0 as usize].next(&mut registers, flip);
    }
    Ok(registers.1)
}

#[aoc(day8, part2)]
fn part2(program: &[Operation]) -> Option<isize> {
    let mut graph = build_graph(program);
    let path = bfs(
        &(0, 0),
        |(depth, counter)| graph.entry((*depth, *counter)).or_default().clone(),
        |(depth, counter)| *depth == 1 && *counter == (program.len() - 1) as isize,
    );

    // figure out which instruction was flipped, AKA the last 0 depth entry
    let mut last = 0;
    for (depth, counter) in path? {
        if depth == 0 {
            last = counter;
        } else {
            break;
        }
    }

    // Run the program with that item flipped
    match run_program(program, Some(last)) {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

// Build a graph where the 0 depth is with no alteration and 1 depth is with a single alteration
// Since we only care about the case where we can only flip one value that is far as we need to go
// We can then use BFS to find the program execution path in which only one of the values change
fn build_graph(program: &[Operation]) -> HashMap<(usize, isize), Vec<(usize, isize)>> {
    let mut graph: HashMap<(usize, isize), Vec<(usize, isize)>> = HashMap::new();
    for (counter, operation) in program.iter().enumerate() {
        let next = counter as isize
            + match operation {
                Operation::Nop(_) | Operation::Acc(_) => 1,
                Operation::Jmp(offset) => *offset,
            };
        // If we are not in the first depth follow this path
        graph
            .entry((0, counter as isize))
            .or_default()
            .push((0, next));
        // If we are in the second depth, we still need to follow this path
        graph
            .entry((1, counter as isize))
            .or_default()
            .push((1, next));

        let flipped = match operation {
            Operation::Jmp(_) => Some(1),
            Operation::Nop(offset) => Some(*offset),
            Operation::Acc(_) => None,
        };

        if let Some(next) = flipped {
            // Graph that this node can go to the next depth by flipping it from Jmp to Nop or vice versa
            graph
                .entry((0, counter as isize))
                .or_default()
                .push((1, counter as isize + next));
        }
    }
    graph
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
    fn parsing_input() {
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
        assert_eq!(part2(&input), Some(8));
    }
}
