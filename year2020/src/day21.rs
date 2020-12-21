/// https://adventofcode.com/2020/day/21
use aoc_runner_derive::{aoc, aoc_generator};
use pest::Parser;

#[derive(Parser)]
#[grammar = "day21.pest"]
struct InputParser;

#[aoc_generator(day21)]
fn parse_input(input: &str) -> usize{
    let _rules = InputParser::parse(Rule::file, input).expect("unable to parse input");
    0
}

#[aoc(day21, part1)]
fn part1(_listing: &usize) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;
    static LISTING: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn parsing_input() {
        parse_input(LISTING);
    }

    #[test]
    fn running_part1() {
        let listing = parse_input(LISTING);
        assert_eq!(part1(&listing), 5);
    }
}