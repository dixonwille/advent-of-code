/// https://adventofcode.com/2020/day/8
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Eq, PartialEq)]
struct Output;

#[aoc_generator(day8)]
fn parse_input(_input: &str) -> Output {
    unimplemented!()
}

#[aoc(day8, part1)]
fn part1(_input: &Output) -> usize {
    unimplemented!()
}

#[aoc(day8, part2)]
fn part2(_input: &Output) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test{
    use super::*;
    static INPUT: &str = "";

    #[test]
    fn parsring_input() {
        assert_eq!(parse_input(INPUT), Output);
    }

    #[test]
    fn running_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn running_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 0);
    }
}