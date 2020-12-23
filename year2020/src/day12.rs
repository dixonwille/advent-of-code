/// https://adventofcode.com/2020/day/12
use nom::{
    bytes::complete::take,
    character::complete::{char as c, digit1},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<Direction> {
    let (_, directions) = parse_input_nom(input).unwrap();
    directions
}

fn parse_input_nom(input: &str) -> IResult<&str, Vec<Direction>> {
    all_consuming(separated_list1(
        c('\n'),
        map(tuple((take(1 as usize), digit1)), |(dir, num)| match dir {
            "N" => Direction::North(num.parse().unwrap()),
            "S" => Direction::South(num.parse().unwrap()),
            "E" => Direction::East(num.parse().unwrap()),
            "W" => Direction::West(num.parse().unwrap()),
            "L" => Direction::Left(num.parse().unwrap()),
            "R" => Direction::Right(num.parse().unwrap()),
            "F" => Direction::Forward(num.parse().unwrap()),
            _ => unreachable!(),
        }),
    ))(input)
}

#[aoc(day12, part1)]
fn part1(directions: &[Direction]) -> isize {
    let mut pos = (0, 0);
    let mut dir = (1, 0);
    for d in directions {
        match d {
            Direction::North(num) => pos.1 += num,
            Direction::South(num) => pos.1 -= num,
            Direction::East(num) => pos.0 += num,
            Direction::West(num) => pos.0 -= num,
            Direction::Left(num) => match num {
                90 => {
                    std::mem::swap(&mut dir.0, &mut dir.1);
                    dir.0 *= -1;
                }
                180 => {
                    dir.0 *= -1;
                    dir.1 *= -1;
                }
                270 => {
                    std::mem::swap(&mut dir.0, &mut dir.1);
                    dir.1 *= -1;
                }
                _ => unreachable!(),
            },
            Direction::Right(num) => match num {
                90 => {
                    std::mem::swap(&mut dir.0, &mut dir.1);
                    dir.1 *= -1;
                }
                180 => {
                    dir.0 *= -1;
                    dir.1 *= -1;
                }
                270 => {
                    std::mem::swap(&mut dir.0, &mut dir.1);
                    dir.0 *= -1;
                }
                _ => unreachable!(),
            },
            Direction::Forward(num) => {
                pos.0 += dir.0 * *num;
                pos.1 += dir.1 * *num;
            }
        }
    }
    pos.0.abs() + pos.1.abs()
}

#[aoc(day12, part2)]
fn part2(directions: &[Direction]) -> isize {
    let mut ship = (0, 0);
    let mut way = (10, 1);
    for d in directions {
        match d {
            Direction::North(num) => way.1 += num,
            Direction::South(num) => way.1 -= num,
            Direction::East(num) => way.0 += num,
            Direction::West(num) => way.0 -= num,
            Direction::Left(num) => match num {
                90 => {
                    std::mem::swap(&mut way.0, &mut way.1);
                    way.0 *= -1;
                }
                180 => {
                    way.0 *= -1;
                    way.1 *= -1;
                }
                270 => {
                    std::mem::swap(&mut way.0, &mut way.1);
                    way.1 *= -1;
                }
                _ => unreachable!(),
            },
            Direction::Right(num) => match num {
                90 => {
                    std::mem::swap(&mut way.0, &mut way.1);
                    way.1 *= -1;
                }
                180 => {
                    way.0 *= -1;
                    way.1 *= -1;
                }
                270 => {
                    std::mem::swap(&mut way.0, &mut way.1);
                    way.0 *= -1;
                }
                _ => unreachable!(),
            },
            Direction::Forward(num) => {
                ship.0 += way.0 * num;
                ship.1 += way.1 * num;
            }
        }
    }
    ship.0.abs() + ship.1.abs()
}

#[cfg(test)]
mod test {
    use super::*;

    static DIRECTIONS: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn parsing_input() {
        assert_eq!(
            parse_input(DIRECTIONS),
            vec![
                Direction::Forward(10),
                Direction::North(3),
                Direction::Forward(7),
                Direction::Right(90),
                Direction::Forward(11)
            ]
        )
    }

    #[test]
    fn running_part1() {
        let input = parse_input(DIRECTIONS);
        assert_eq!(part1(&input), 25);
    }

    #[test]
    fn running_part2() {
        let input = parse_input(DIRECTIONS);
        assert_eq!(part2(&input), 286);
    }
}
