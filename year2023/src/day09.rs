use crate::Solutions;
use lib_aoc::prelude::*;

fn get_diffs(input: &Vec<isize>) -> Vec<isize> {
    input
        .windows(2)
        .map(|w| w.last().unwrap() - w.first().unwrap())
        .collect()
}

fn predict_next(input: &Vec<isize>) -> isize {
    let mut diffs = vec![input.to_owned()];
    while !diffs.last().unwrap().iter().all(|d| d == &0isize) {
        diffs.push(get_diffs(diffs.last().unwrap()));
    }
    diffs.reverse();
    let mut prev: isize = 0;
    for diff in diffs.iter() {
        prev = diff.last().unwrap() + prev;
    }
    prev
}

fn predict_prev(input: &Vec<isize>) -> isize {
    let mut diffs = vec![input.to_owned()];
    while !diffs.last().unwrap().iter().all(|d| d == &0isize) {
        diffs.push(get_diffs(diffs.last().unwrap()));
    }
    diffs.reverse();
    let mut prev: isize = 0;
    for diff in diffs.iter() {
        prev = diff.first().unwrap() - prev;
    }
    prev
}

impl Solution<DAY_09> for Solutions {
    type Input<'i> = Vec<Vec<isize>>;
    type Output = isize;

    fn parse(puzzle: &str) -> Vec<Vec<isize>> {
        puzzle
            .lines()
            .map(|l| l.split(" ").map(|c| c.parse::<isize>().unwrap()).collect())
            .collect()
    }

    fn part_one(input: &Vec<Vec<isize>>) -> isize {
        input.iter().map(|l| predict_next(l)).sum()
    }

    fn part_two(input: &Vec<Vec<isize>>) -> isize {
        input.iter().map(|l| predict_prev(l)).sum()
    }
}

impl Test<DAY_09> for Solutions {
    fn expected(part: bool) -> isize {
        match part {
            PART_ONE => 114,
            PART_TWO => 2,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solutions;
    use lib_aoc::prelude::*;

    derive_tests!(Solutions, DAY_09);
}
