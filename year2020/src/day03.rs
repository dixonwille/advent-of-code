/// https://adventofcode.com/2020/day/3

#[derive(Debug, PartialEq, Eq)]
struct Map {
    rows: usize,
    cols: usize,
    data: Vec<bool>,
}

impl Map {
    fn iter(&self, down: usize, right: usize) -> MapIter<'_> {
        MapIter::new(self, down, right)
    }
    fn get_from_row_col(&self, row: usize, col: usize) -> Option<&bool> {
        self.data.get(row * self.cols + col)
    }
}

struct MapIter<'a> {
    map: &'a Map,
    down: usize,
    right: usize,
    next_row: usize, // zero based
    next_col: usize, // zero based
}

impl<'a> MapIter<'a> {
    fn new(map: &'a Map, down: usize, right: usize) -> Self {
        MapIter {
            map,
            down,
            right,
            next_row: 0,
            next_col: 0,
        }
    }
}

impl<'a> Iterator for MapIter<'a> {
    type Item = &'a bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_row > self.map.rows - 1 {
            return None;
        }
        let tree = self.map.get_from_row_col(self.next_row, self.next_col);
        self.next_row += self.down;
        self.next_col = (self.next_col + self.right) % self.map.cols; // map is infinite to the right repeating
        tree
    }
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Map {
    let data = input
        .lines()
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|b| match b {
                    b'.' => false,
                    b'#' => true,
                    _ => unreachable!("only matches on . or #"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Map {
        rows: data.len(),
        cols: data[0].len(),
        data: data.into_iter().flatten().collect(),
    }
}

#[aoc(day3, part1)]
fn part1(map: &Map) -> usize {
    map.iter(1, 3).fold(0, |count, tree| match tree {
        true => count + 1,
        false => count,
    })
}

// (down, right)
static SLOPES: [(usize, usize); 5] = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

#[aoc(day3, part2)]
fn part2(map: &Map) -> usize {
    SLOPES.iter().fold(1, |mult, (down, right)| {
        mult * map.iter(*down, *right).fold(0, |count, tree| match tree {
            true => count + 1,
            false => count,
        })
    })
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
        assert_eq!(
            parse_input(MAP),
            Map {
                rows: 11,
                cols: 11,
                data: vec![
                    false, false, true, true, false, false, false, false, false, false, false,
                    true, false, false, false, true, false, false, false, true, false, false,
                    false, true, false, false, false, false, true, false, false, true, false,
                    false, false, true, false, true, false, false, false, true, false, true, false,
                    true, false, false, false, true, true, false, false, true, false, false, false,
                    true, false, true, true, false, false, false, false, false, false, true, false,
                    true, false, true, false, false, false, false, true, false, true, false, false,
                    false, false, false, false, false, false, true, true, false, true, true, false,
                    false, false, true, false, false, false, true, false, false, false, true, true,
                    false, false, false, false, true, false, true, false, false, true, false,
                    false, false, true, false, true
                ],
            }
        );
    }

    #[test]
    fn running_part1() {
        let map = parse_input(MAP);
        assert_eq!(part1(&map), 7);
    }

    #[test]
    fn running_part2() {
        let map = parse_input(MAP);
        assert_eq!(part2(&map), 336);
    }
}
