/// https://adventofcode.com/2020/day/17
use aoc_runner_derive::{aoc, aoc_generator};
use once_cell::sync::Lazy;

use std::{collections::{HashMap, HashSet}, hash::Hash};

#[aoc_generator(day17)]
fn parse_input(input: &str) -> HashSet<(i32, i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|&(_, b)| b == '#')
                .map(move |(x, _)| (x as i32, y as i32, 0))
        })
        .collect()
}

static NEIGHBORS3D: Lazy<Vec<(i32, i32, i32)>> = Lazy::new(neighbors3d);

fn neighbors3d() -> Vec<(i32, i32, i32)> {
    let max = (3 as i32).pow(3);
    (0..max)
        .filter(|&i| i != max / 2)
        .map(|i| (i % 3 - 1, (i / 3) % 3 - 1, i / 9 - 1))
        .collect()
}

fn count_neighbors3d(active: &HashSet<(i32, i32, i32)>) -> HashMap<(i32, i32, i32), usize> {
    let mut neig = HashMap::new();
    for (x, y, z) in active {
        for (dx, dy, dz) in Lazy::force(&NEIGHBORS3D) {
            *neig.entry((x + dx, y + dy, z + dz)).or_insert(0) += 1;
        }
    }
    neig
}

static NEIGHBORS4D: Lazy<Vec<(i32, i32, i32, i32)>> = Lazy::new(neighbors4d);

fn neighbors4d() -> Vec<(i32, i32, i32, i32)> {
    let max = (3 as i32).pow(4);
    (0..max)
        .filter(|&i| i != max / 2)
        .map(|i| {
            (
                i % 3 - 1,
                (i / 3) % 3 - 1,
                (i / 9) % 3 - 1,
                (i / 27) % 3 - 1,
            )
        })
        .collect()
}

fn count_neighbors4d(active: &HashSet<(i32, i32, i32, i32)>) -> HashMap<(i32, i32, i32, i32), usize> {
    let mut neig = HashMap::new();
    for (x, y, z, a) in active {
        for (dx, dy, dz, da) in Lazy::force(&NEIGHBORS4D) {
            *neig.entry((x + dx, y + dy, z + dz, a + da)).or_insert(0) += 1;
        }
    }
    neig
}

fn simulate<P, F>(mut active: HashSet<P>, count_neighbors: F) -> usize where
    P: Hash + Eq + Copy,
    F: Fn(&HashSet<P>) -> HashMap<P, usize>
{
    for _ in 0..6 {
        active = count_neighbors(&active)
            .iter()
            .filter(|(pos, n)| matches!((active.contains(pos),n), (true, 2) | (_, 3)))
            .map(|(&pos, _)| pos)
            .collect();
    }
    active.len()
}

#[aoc(day17, part1)]
fn part1(init: &HashSet<(i32, i32, i32)>) -> usize {
    simulate(init.clone(), count_neighbors3d)
}

#[aoc(day17, part2)]
fn part2(init: &HashSet<(i32, i32, i32)>) -> usize {
    let init = init.clone().into_iter().map(|(x,y, z)| (x, y, z, 0)).collect();
    simulate(init, count_neighbors4d)
}

#[cfg(test)]
mod test {
    use super::*;
    static INIT: &str = ".#.
..#
###";

    #[test]
    fn parsing_input() {
        let mut set = HashSet::new();
        set.insert((1, 0, 0));
        set.insert((2, 1, 0));
        set.insert((0, 2, 0));
        set.insert((1, 2, 0));
        set.insert((2, 2, 0));
        assert_eq!(parse_input(INIT), set)
    }

    #[test]
    fn running_part1() {
        let init = parse_input(INIT);
        assert_eq!(part1(&init), 112);
    }

    #[test]
    fn running_part2() {
        let init = parse_input(INIT);
        assert_eq!(part2(&init), 848);
    }
}
