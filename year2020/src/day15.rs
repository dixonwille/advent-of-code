/// https://adventofcode.com/2020/day/15
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
fn parse_input(_input: &str) -> usize {
    unimplemented!()
}

#[aoc(day15, part1)]
fn part1(_input: &usize) -> usize {
    unimplemented!()
}

#[aoc(day15, part2)]
fn part2(_input: &usize) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test{
    use super::*;

    static INPUT: &str = "";

    #[test]
    fn parsing_input() {
        assert_eq!(parse_input(INPUT), 0);
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