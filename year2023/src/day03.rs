use std::ops::{Index, RangeInclusive};

use crate::Solutions;
use itertools::Itertools;
use lib_aoc::prelude::*;

#[derive(Debug)]
pub struct Engine {
    x: usize,
    y: usize,
    parts: Vec<Part>,
}

impl Engine {
    fn get_cells_around<'e>(&'e self, pos: usize) -> impl Iterator<Item = usize> + 'e {
        // Bound checks
        let (mut lx, mut ux, mut ly, mut uy) = (-1isize, 1isize, -1isize, 1isize);
        if pos % self.x == 0 {
            lx = 0;
        } else if pos % self.x == self.x - 1 {
            ux = 0;
        }
        if pos / self.x == 0 {
            ly = 0;
        } else if pos / self.x == self.y - 1 {
            uy = 0;
        }
        // Cells to check
        (ly..=uy)
            .map(move |y| {
                (lx..=ux).filter_map(move |x| {
                    if x == 0 && y == 0 {
                        None
                    } else {
                        Some((y * (self.x as isize) + x + pos as isize) as usize)
                    }
                })
            })
            .flatten()
    }

    fn get_part_number(&self, pos: usize) -> (usize, RangeInclusive<usize>) {
        let lower = pos - (pos % self.x);
        let upper = lower + self.x - 1;
        let mut prev: Vec<_> = (lower..pos)
            .rev()
            .take_while(|&cell| matches!(self.parts.index(cell), Part::Digit(_)))
            .map(|cell| match self.parts.index(cell) {
                Part::Digit(pd) => pd,
                _ => unreachable!(),
            })
            .collect();
        prev.reverse();
        let prev_len = prev.len();
        let mut next: Vec<_> = (pos..=upper)
            .take_while(|&cell| matches!(self.parts.index(cell), Part::Digit(_)))
            .map(|cell| match self.parts.index(cell) {
                Part::Digit(pd) => pd,
                _ => unreachable!(),
            })
            .collect();
        let next_len = next.len();
        prev.append(&mut next);
        (
            prev.into_iter().collect::<String>().parse().unwrap(),
            RangeInclusive::new(pos - prev_len, pos + next_len - 1),
        )
    }
}

#[derive(Debug)]
enum Part {
    Digit(char),
    Symbol(char),
    Space,
}

impl Solution<DAY_03> for Solutions {
    type Input<'i> = Engine;
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        let engine = puzzle
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        if c.is_digit(10) {
                            Part::Digit(c)
                        } else if c == '.' {
                            Part::Space
                        } else {
                            Part::Symbol(c)
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .fold((0usize, 0usize, Vec::new()), |(_, ay, mut av), line| {
                let x = line.len();
                av.extend(line);
                (x, ay + 1, av)
            });
        Engine {
            x: engine.0,
            y: engine.1,
            parts: engine.2,
        }
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        let mut next_i = 0;
        input
            .parts
            .iter()
            .enumerate()
            .filter_map(|(i, part)| match part {
                Part::Digit(_) => {
                    if i < next_i
                        || !input
                            .get_cells_around(i)
                            .any(|cell| matches!(input.parts.index(cell), Part::Symbol(_)))
                    {
                        return None;
                    }
                    let (part, span) = input.get_part_number(i);
                    next_i = span.last().unwrap() + 1;
                    Some(part)
                }
                _ => None,
            })
            .sum()
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        input
            .parts
            .iter()
            .enumerate()
            .filter_map(|(i, part)| match part {
                Part::Symbol('*') => {
                    let part_nums: Vec<_> = input
                        .get_cells_around(i)
                        .filter_map(|cell| match input.parts.index(cell) {
                            Part::Digit(_) => Some(input.get_part_number(cell)),
                            _ => None,
                        })
                        .unique()
                        .collect();
                    if part_nums.len() != 2 {
                        return None;
                    }
                    Some(part_nums.first().unwrap().0 * part_nums.last().unwrap().0)
                }
                _ => None,
            })
            .sum()
    }
}

impl Test<DAY_03> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 4361,
            PART_TWO => 467835,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solutions;
    use lib_aoc::prelude::*;

    derive_tests!(Solutions, DAY_03);
}
