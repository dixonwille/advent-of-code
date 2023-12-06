use crate::Solutions;
use itertools::Itertools;
use lib_aoc::prelude::*;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, newline, u64},
    error::ParseError,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

fn metric<'a, E: ParseError<&'a str>>(
    m: &'a str,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<u64>, E> {
    preceded(
        terminated(tag(m), multispace1),
        separated_list1(multispace1, u64),
    )
}

fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    separated_pair(metric("Time:"), newline, metric("Distance:"))(input)
}

fn winning_iters(num: &u64, dist: &u64) -> usize {
    let iter = if num % 2 == 0 { num / 2 } else { num / 2 + 1 };
    let count = (0..iter)
        .filter_map(|i| {
            let traveled = (num - i) * i;
            if traveled <= *dist {
                None
            } else {
                Some(traveled)
            }
        })
        .count();
    if num % 2 == 0 {
        count * 2 + 1
    } else {
        count * 2
    }
}

impl Solution<DAY_06> for Solutions {
    type Input<'i> = (Vec<u64>, Vec<u64>);
    type Output = usize;

    fn parse(puzzle: &str) -> (Vec<u64>, Vec<u64>) {
        let (_, p) = parse(puzzle).unwrap();
        p
    }

    fn part_one(input: &(Vec<u64>, Vec<u64>)) -> usize {
        input
            .0
            .iter()
            .zip(&input.1)
            .map(|(t, d)| winning_iters(t, d))
            .product()
    }

    fn part_two(input: &(Vec<u64>, Vec<u64>)) -> usize {
        let time = input
            .0
            .iter()
            .map(|x| x.to_string())
            .join("")
            .parse()
            .unwrap();
        let distance = input
            .1
            .iter()
            .map(|x| x.to_string())
            .join("")
            .parse()
            .unwrap();
        winning_iters(&time, &distance)
    }
}

impl Test<DAY_06> for Solutions {
    fn expected(part: bool) -> usize {
        match part {
            PART_ONE => 288,
            PART_TWO => 71503,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solutions;
    use lib_aoc::prelude::*;

    derive_tests!(Solutions, DAY_06);
}
