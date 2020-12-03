/// https://adventofcode.com/2020/day/3
use aoc_runner_derive::{aoc, aoc_generator};

struct Map {
    rows: usize,
    cols: usize,
    data: Vec<bool>,
}

impl Map {
    fn iter(&self, down: usize, right: usize) -> MapIter<'_> {
        MapIter::new(self, down, right)
    }
}

struct MapIter<'a> {
    map: &'a Map,
    down: usize,
    right: usize,
    cur_row: usize, // zero based
    cur_col: usize, // zero based
}

impl<'a> MapIter<'a> {
    fn new(map: &'a Map, down: usize, right: usize) -> Self {
        MapIter{
            map,
            down,
            right,
            cur_row: 0,
            cur_col: 0,
        }
    }
}

impl Iterator for MapIter<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("impliment rise over run algorithm for map")
    }
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    static MAP: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn parsing_input() {
        assert_eq!(parse_input(MAP), 0);
    }
}