use crate::Solutions;
use lib_aoc::prelude::*;

#[derive(Debug)]
pub struct Game {
    id: usize,
    hands: Vec<(usize, usize, usize)>,
}

impl Solution<DAY_02> for Solutions {
    type Input<'i> = Vec<Game>;
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        puzzle
            .lines()
            .map(|line| {
                let game: Vec<_> = line.split(": ").collect();
                let id = game.first().unwrap().get(5..).unwrap().parse().unwrap();
                let hands = game
                    .last()
                    .unwrap()
                    .split("; ")
                    .map(|hand| {
                        hand.split(", ")
                            .map(|cube| {
                                let parts: Vec<_> = cube.split(' ').collect();
                                (
                                    parts.first().unwrap().parse().unwrap(),
                                    parts.last().unwrap().to_owned(),
                                )
                            })
                            .fold(
                                (0usize, 0usize, 0usize),
                                |acc, (count, color)| match color {
                                    "red" => (count, acc.1, acc.2),
                                    "green" => (acc.0, count, acc.2),
                                    "blue" => (acc.0, acc.1, count),
                                    _ => unreachable!(),
                                },
                            )
                    })
                    .collect();
                Game { id, hands }
            })
            .collect()
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        input
            .into_iter()
            .filter_map(|game| {
                if game
                    .hands
                    .iter()
                    .any(|hand| hand.0 > 12 || hand.1 > 13 || hand.2 > 14)
                {
                    None
                } else {
                    Some(game.id)
                }
            })
            .sum()
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        input
            .into_iter()
            .map(|game| {
                let min_cubes = game
                    .hands
                    .iter()
                    .fold((0usize, 0usize, 0usize), |acc, (r, g, b)| {
                        (acc.0.max(*r), acc.1.max(*g), acc.2.max(*b))
                    });
                min_cubes.0 * min_cubes.1 * min_cubes.2
            })
            .sum()
    }
}

impl Test<DAY_02> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 8,
            PART_TWO => 2286,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solutions;
    use lib_aoc::prelude::*;

    derive_tests!(Solutions, DAY_02);
}
