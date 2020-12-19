/// https://adventofcode.com/2020/day/19
use aoc_runner_derive::{aoc, aoc_generator};

use pest::Parser;

#[derive(Parser)]
#[grammar = "day19.pest"]
pub struct InputParser;

#[aoc_generator(day19)]
fn parse_input(input: &str) -> usize {
    let inputs = InputParser::parse(Rule::file, input).expect("could not parse input");
    println!("{:#?}", inputs);
    0
}

#[aoc(day19, part1)]
fn part1(_input: &usize) -> usize {
    unimplemented!()
}

#[aoc(day19, part2)]
fn part2(_input: &usize) -> usize {
    unimplemented!()
}


#[cfg(test)]
mod test {
    use super::*;
     
    static INPUT: &str = r###"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"###;

    #[test]
    fn testing() {
        parse_input(INPUT);
    }

}