/// https://adventofcode.com/2020/day/15
use std::{collections::HashMap, fmt::Debug, hash::Hash};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<usize> {
    input.split(',').map(|i| i.parse().unwrap()).collect()
}

#[derive(Debug)]
struct Mem<V> {
    map: HashMap<V, usize>,
    cache: Option<(V, usize)>,
}


// TODO do I need this "cache" layer or is there a smarter way to get what I want on insert.
impl<V> Mem<V>
where
    V: Copy + Eq + Hash,
{
    fn new(starting: &[V]) -> Self {
        let mut mem = Mem {
            map: HashMap::new(),
            cache: None,
        };
        for (i, v) in starting.iter().enumerate() {
            mem.insert(*v, i);
        }
        mem
    }
}

impl<V> Mem<V>
where
    V: Eq + Hash,
{
    fn insert(&mut self, k: V, v: usize) {
        if let Some(c) = self.cache.take() {
            self.map.insert(c.0, c.1);
        }
        self.cache = Some((k, v));
    }

    fn get(&self, k: &V) -> Option<&usize> {
        self.map.get(k)
    }
}

fn nth_word_spoken(starting: &[usize], nth: usize) -> usize {
    let mut mem = Mem::new(starting);
    let mut query = *starting.last().unwrap();
    for i in starting.len()..nth {
        match mem.get(&query) {
            Some(last) => {
                let ans = i - 1 - last;
                query = ans;
                mem.insert(ans, i);
            }
            None => {
                query = 0;
                mem.insert(0, i);
            }
        }
    }
    query
}

#[aoc(day15, part1)]
fn part1(starting: &[usize]) -> usize {
    nth_word_spoken(starting, 2020)
}

#[aoc(day15, part2)]
fn part2(starting: &[usize]) -> usize {
    nth_word_spoken(starting, 30_000_000)
}

#[cfg(test)]
mod test {
    use super::*;

    static STARTING1: &str = "0,3,6";
    static STARTING2: &str = "1,3,2";
    static STARTING3: &str = "2,1,3";
    static STARTING4: &str = "1,2,3";
    static STARTING5: &str = "2,3,1";
    static STARTING6: &str = "3,2,1";
    static STARTING7: &str = "3,1,2";

    #[test]
    fn parsing_input() {
        assert_eq!(parse_input(STARTING1), vec![0, 3, 6]);
        assert_eq!(parse_input(STARTING2), vec![1, 3, 2]);
        assert_eq!(parse_input(STARTING3), vec![2, 1, 3]);
        assert_eq!(parse_input(STARTING4), vec![1, 2, 3]);
        assert_eq!(parse_input(STARTING5), vec![2, 3, 1]);
        assert_eq!(parse_input(STARTING6), vec![3, 2, 1]);
        assert_eq!(parse_input(STARTING7), vec![3, 1, 2]);
    }

    #[test]
    fn running_part1() {
        let starting = parse_input(STARTING1);
        assert_eq!(part1(&starting), 436);
        let starting = parse_input(STARTING2);
        assert_eq!(part1(&starting), 1);
        let starting = parse_input(STARTING3);
        assert_eq!(part1(&starting), 10);
        let starting = parse_input(STARTING4);
        assert_eq!(part1(&starting), 27);
        let starting = parse_input(STARTING5);
        assert_eq!(part1(&starting), 78);
        let starting = parse_input(STARTING6);
        assert_eq!(part1(&starting), 438);
        let starting = parse_input(STARTING7);
        assert_eq!(part1(&starting), 1836);
    }

    #[test]
    #[ignore = "takes long time to run (longer than a minute)"]
    fn running_part2() {
        let starting = parse_input(STARTING1);
        assert_eq!(part2(&starting), 175594);
        let starting = parse_input(STARTING2);
        assert_eq!(part2(&starting), 2578);
        let starting = parse_input(STARTING3);
        assert_eq!(part2(&starting), 3544142);
        let starting = parse_input(STARTING4);
        assert_eq!(part2(&starting), 261214);
        let starting = parse_input(STARTING5);
        assert_eq!(part2(&starting), 6895259);
        let starting = parse_input(STARTING6);
        assert_eq!(part2(&starting), 18);
        let starting = parse_input(STARTING7);
        assert_eq!(part2(&starting), 362);
    }
}
