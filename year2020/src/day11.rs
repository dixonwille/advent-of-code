/// https://adventofcode.com/2020/day/10
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    character::complete::char as c,
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Position {
    Empty,
    Occupied,
    Floor,
}

#[derive(Debug, Eq, PartialEq)]
struct FloorPlan {
    rows: usize,
    cols: usize,
    plan: Vec<Position>,
}

impl FloorPlan {
    fn new(raw: Vec<Vec<Position>>) -> Self {
        FloorPlan{
            rows: raw.len(),
            cols: raw[0].len(),
            plan: raw.into_iter().flatten().collect()
        }
    }
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> FloorPlan {
    let (_, seats) = parse_input_nom(input).unwrap();
    seats
}

fn parse_input_nom(input: &str) -> IResult<&str, FloorPlan> {
    map(
        all_consuming(separated_list1(
            c('\n'),
            many1(alt((
                value(Position::Floor, c('.')),
                value(Position::Empty, c('L')),
            ))),
        )),
        FloorPlan::new,
    )(input)
}

#[aoc(day11, part1)]
fn part1(_input: &FloorPlan) -> usize {
    unimplemented!()
}

#[aoc(day11, part2)]
fn part2(_input: &FloorPlan) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn parsing_input() {
        use Position::*;
        assert_eq!(
            parse_input(INPUT),
            FloorPlan {
                rows: 10,
                cols: 10,
                plan: vec![
                    Empty, Floor, Empty, Empty, Floor, Empty, Empty, Floor, Empty, Empty, Empty,
                    Empty, Empty, Empty, Empty, Empty, Empty, Floor, Empty, Empty, Empty, Floor,
                    Empty, Floor, Empty, Floor, Floor, Empty, Floor, Floor, Empty, Empty, Empty,
                    Empty, Floor, Empty, Empty, Floor, Empty, Empty, Empty, Floor, Empty, Empty,
                    Floor, Empty, Empty, Floor, Empty, Empty, Empty, Floor, Empty, Empty, Empty,
                    Empty, Empty, Floor, Empty, Empty, Floor, Floor, Empty, Floor, Empty, Floor,
                    Floor, Floor, Floor, Floor, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
                    Empty, Empty, Empty, Empty, Floor, Empty, Empty, Empty, Empty, Empty, Empty,
                    Floor, Empty, Empty, Floor, Empty, Empty, Empty, Empty, Empty, Floor, Empty,
                    Empty
                ]
            }
        );
    }

    #[test]
    fn running_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 37);
    }

    #[test]
    fn running_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 0);
    }
}
