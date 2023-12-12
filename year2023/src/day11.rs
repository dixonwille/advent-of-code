use crate::Solutions;
use lib_aoc::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    Galaxy,
    Space,
}

#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize,
}

struct GalaxyPairs<'a> {
    pair_a: usize,
    pair_b: usize,
    num_pairs: usize,
    galaxies: &'a Vec<Galaxy>,
}

impl<'a> GalaxyPairs<'a> {
    fn iter(galaxies: &'a Vec<Galaxy>) -> Self {
        let n = galaxies.len();
        let num_pairs = (n * (n - 1)) / 2;
        GalaxyPairs {
            pair_a: n - 1,
            pair_b: n - 2,
            num_pairs,
            galaxies,
        }
    }
}

impl<'a> Iterator for GalaxyPairs<'a> {
    type Item = (&'a Galaxy, &'a Galaxy);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pair_a == self.pair_b {
            return None;
        }
        let pairs = (
            self.galaxies.get(self.pair_a).unwrap(),
            self.galaxies.get(self.pair_b).unwrap(),
        );
        if self.pair_b == 0 {
            self.pair_a -= 1;
            if self.pair_a == 0 {
                self.pair_b = 0;
            } else {
                self.pair_b = self.pair_a - 1;
            }
        } else {
            self.pair_b -= 1;
        }
        Some(pairs)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.num_pairs))
    }
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Galaxy),
            '.' => Ok(Tile::Space),
            _ => Err("Unkonwn Tile"),
        }
    }
}

fn expansion(tiles: &Vec<Vec<Tile>>) -> (Vec<usize>, Vec<usize>) {
    let empty_rows: Vec<_> = tiles
        .iter()
        .enumerate()
        .filter_map(|(y, row)| {
            if row.iter().all(|col| col == &Tile::Space) {
                Some(y)
            } else {
                None
            }
        })
        .collect();
    let mut empty_cols = Vec::new();
    for x in 0..tiles[0].len() {
        let mut all_space = true;
        for y in 0..tiles.len() {
            if tiles[y][x] == Tile::Galaxy {
                all_space = false;
                break;
            }
        }
        if all_space {
            empty_cols.push(x);
        }
    }
    (empty_rows, empty_cols)
}

fn distance(
    a: &Galaxy,
    b: &Galaxy,
    (exp_y, exp_x): &(Vec<usize>, Vec<usize>),
    rate: usize,
) -> usize {
    let (mut ax, mut ay) = (a.x, a.y);
    let (mut bx, mut by) = (b.x, b.y);
    if ax < bx {
        let temp = ax;
        ax = bx;
        bx = temp;
    }
    if ay < by {
        let temp = ay;
        ay = by;
        by = temp;
    }
    let x_exp = exp_x.iter().filter(|x| (bx..=ax).contains(x)).count();
    let y_exp = exp_y.iter().filter(|y| (by..=ay).contains(y)).count();
    (ax - bx + (x_exp * (rate - 1))) + (ay - by + (y_exp * (rate - 1)))
}

fn into_galaxies(tiles: &Vec<Vec<Tile>>) -> Vec<Galaxy> {
    tiles
        .iter()
        .enumerate()
        .filter_map(|(y, row)| {
            let r = row
                .iter()
                .enumerate()
                .filter_map(|(x, tile)| match tile {
                    Tile::Space => None,
                    Tile::Galaxy => Some(Galaxy { x, y }),
                })
                .collect::<Vec<_>>();
            if r.len() == 0 {
                None
            } else {
                Some(r)
            }
        })
        .flatten()
        .collect()
}

impl Solution<DAY_11> for Solutions {
    type Input<'i> = Vec<Vec<Tile>>;
    type Output = usize;

    fn parse(puzzle: &str) -> Vec<Vec<Tile>> {
        puzzle
            .lines()
            .into_iter()
            .map(|l| l.chars().map(|c| Tile::try_from(c)).collect())
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }

    fn part_one(input: &Vec<Vec<Tile>>) -> usize {
        let expansion = expansion(input);
        let galaxies = into_galaxies(input);
        GalaxyPairs::iter(&galaxies)
            .map(|(a, b)| distance(a, b, &expansion, 2))
            .sum()
    }

    fn part_two(input: &Vec<Vec<Tile>>) -> usize {
        let expansion = expansion(input);
        let galaxies = into_galaxies(input);
        let rate = if input.len() < 20 { 100 } else { 1_000_000 };
        GalaxyPairs::iter(&galaxies)
            .map(|(a, b)| distance(a, b, &expansion, rate))
            .sum()
    }
}

impl Test<DAY_11> for Solutions {
    fn expected(part: bool) -> usize {
        match part {
            PART_ONE => 374,
            PART_TWO => 8410,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solutions;
    use lib_aoc::prelude::*;

    derive_tests!(Solutions, DAY_11);
}
