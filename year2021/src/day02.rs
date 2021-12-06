/// https://adventofcode.com/2021/day/2

use nom::{
    character::complete::{alpha1, digit1, line_ending, multispace1},
    combinator::{all_consuming, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Move {
    Forward(usize),
    Down(usize),
    Up(usize),
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<Move> {
    let (_, moves) = all_consuming(separated_list1(line_ending, parse_move))(input)
        .expect("Could not parse input");
    moves
}

fn from_number(input: &str) -> Result<usize, std::num::ParseIntError> {
    input.parse::<usize>()
}

fn from_move((m, x): (&str, usize)) -> Result<Move, &'static str> {
    match m.to_lowercase().as_str() {
        "forward" => Ok(Move::Forward(x)),
        "down" => Ok(Move::Down(x)),
        "up" => Ok(Move::Up(x)),
        _ => Err("Could not find move"),
    }
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    map_res(
        separated_pair(alpha1, multispace1, map_res(digit1, from_number)),
        from_move,
    )(input)
}

#[aoc(day2, part1)]
fn part1(moves: &[Move]) -> Option<usize> {
    let pos = moves.iter().fold((0, 0), |acc, m| match m {
        Move::Forward(x) => (acc.0 + x, acc.1),
        Move::Down(x) => (acc.0, acc.1 + x),
        Move::Up(x) => (acc.0, acc.1 - x),
    });
    Some(pos.0 * pos.1)
}

#[aoc(day2, part2)]
fn part2(moves: &[Move]) -> Option<usize> {
    let pos = moves.iter().fold((0, 0, 0), |acc, m| match m {
        Move::Forward(x) => (acc.0 + x, acc.1 + (acc.2 * x), acc.2),
        Move::Down(x) => (acc.0, acc.1, acc.2 + x),
        Move::Up(x) => (acc.0, acc.1, acc.2 - x),
    });
    Some(pos.0 * pos.1)
}

#[cfg(test)]
mod test {
    use super::*;

    static MOVES: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn parsing_input_test() {
        assert_eq!(
            parse_input(MOVES),
            vec![
                Move::Forward(5),
                Move::Down(5),
                Move::Forward(8),
                Move::Up(3),
                Move::Down(8),
                Move::Forward(2)
            ]
        )
    }

    #[test]
    fn part1_test() {
        let moves = parse_input(MOVES);
        assert_eq!(part1(&moves), Some(150))
    }

    #[test]
    fn part2_test() {
        let moves = parse_input(MOVES);
        assert_eq!(part2(&moves), Some(900))
    }
}
