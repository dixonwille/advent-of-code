use crate::Solutions;
use itertools::Itertools;
use lib_aoc::prelude::*;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Card {
    id: usize,
    winning: Vec<usize>,
    picked: Vec<usize>,
}

impl Card {
    fn num_matches(&self) -> usize {
        let combined = &self.winning.iter().chain(&self.picked).unique().count();
        let total = &self.winning.len() + &self.picked.len();
        total - combined
    }
}

impl Solution<DAY_04> for Solutions {
    type Input<'i> = Vec<Card>;
    type Output = usize;

    fn parse(puzzle: &str) -> Vec<Card> {
        puzzle
            .lines()
            .map(|line| {
                let card: Vec<_> = line.split(": ").collect();
                let id = card
                    .first()
                    .unwrap()
                    .get(5..)
                    .unwrap()
                    .trim()
                    .parse()
                    .unwrap();
                let numbers: Vec<Vec<_>> = card
                    .last()
                    .unwrap()
                    .split(" | ")
                    .map(|nums| {
                        nums.split(" ")
                            .filter_map(|num| {
                                if num == "" {
                                    None
                                } else {
                                    Some(num.parse::<usize>().unwrap())
                                }
                            })
                            .collect()
                    })
                    .collect();
                Card {
                    id,
                    winning: numbers.first().unwrap().to_owned(),
                    picked: numbers.last().unwrap().to_owned(),
                }
            })
            .collect()
    }

    fn part_one(input: &Vec<Card>) -> usize {
        input
            .iter()
            .filter_map(|card| match card.num_matches() {
                0 => None,
                num => Some(2usize.pow(num as u32 - 1)),
            })
            .sum()
    }

    fn part_two(input: &Vec<Card>) -> usize {
        input
            .into_iter()
            .enumerate()
            .map(|(i, card)| (i, (1..=card.num_matches()).map(move |j| j + i)))
            .fold(vec![1usize; input.len()], |mut acc, (i, copies)| {
                for copy in copies {
                    acc[copy] += acc[i];
                }
                acc
            })
            .iter()
            .sum()
    }
}

impl Test<DAY_04> for Solutions {
    fn expected(part: bool) -> usize {
        match part {
            PART_ONE => 13,
            PART_TWO => 30,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solutions;
    use lib_aoc::prelude::*;

    derive_tests!(Solutions, DAY_04);
}
