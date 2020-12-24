use once_cell::sync::Lazy;
/// https://adventofcode.com/2020/day/24
use pest::Parser;
use std::{collections::HashMap, str::FromStr};

static NEIGHBORS: Lazy<Vec<Pos>> = Lazy::new(|| vec![
    Direction::NorthEast.get_delta(),
    Direction::NorthWest.get_delta(),
    Direction::SouthEast.get_delta(),
    Direction::SouthWest.get_delta(),
    Direction::East.get_delta(),
    Direction::West.get_delta(),
]);

#[derive(Debug)]
enum Direction {
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    East,
    West,
}

impl Direction {
    fn get_delta(&self) -> Pos {
        match self{
            Direction::NorthEast => Pos(1,0,-1),
            Direction::NorthWest => Pos(0,1,-1),
            Direction::SouthEast => Pos(0,-1,1),
            Direction::SouthWest => Pos(-1,0,1),
            Direction::East => Pos(1,-1,0),
            Direction::West => Pos(-1,1,0),
        }
    }
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ne" => Ok(Direction::NorthEast),
            "nw" => Ok(Direction::NorthWest),
            "se" => Ok(Direction::SouthEast),
            "sw" => Ok(Direction::SouthWest),
            "e" => Ok(Direction::East),
            "w" => Ok(Direction::West),
            _ => Err("unknown character"),
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
enum Color {
    White,
    Black,
}

impl Color {
    fn flip(&mut self) {
        match self {
            Color::White => *self = Color::Black,
            Color::Black => *self = Color::White,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Pos(isize, isize, isize);

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Default for Pos {
    fn default() -> Self {
        Pos(0, 0, 0)
    }
}

#[derive(Parser)]
#[grammar = "day24.pest"]
struct InputParser;

fn build_floor(directions: &[Vec<Direction>]) -> HashMap<Pos, Color> {
    let mut floor: HashMap<_, Color> = HashMap::new();
    for tile in directions {
        let t = tile.iter().fold(Pos::default(), |t, mv| t + mv.get_delta());
        let e = floor.entry(t).or_default();
        e.flip();
    }
    floor
}

fn conways_game(floor: &mut HashMap<Pos, Color>, iter: usize) {
    for _ in 0..iter {
        let mut neighbors: HashMap<Pos, usize> = HashMap::new();
        for (p, _) in floor.iter().filter(|(_, c)| c == &&Color::Black) {
            neighbors.entry(*p).or_insert(0); // make sure current tile is in map
            for neighbor in Lazy::force(&NEIGHBORS).iter().map(|n| *p + *n) {
                *neighbors.entry(neighbor).or_insert(0) += 1;
            }
        }
        let mut new_floor = floor.clone();
        for (p, black_tiles) in neighbors {
            let color = floor.get(&p);
            match (color, black_tiles) {
                (Some(Color::Black), t) if t == 0 || t > 2 => {
                    new_floor.entry(p).or_insert_with(|| Color::Black).flip();
                }
                (Some(Color::White), t) if t == 2 => {
                   new_floor.entry(p).or_insert_with(|| Color::White).flip();
                }
                (None, t) if t == 2 => {
                    new_floor.entry(p).or_insert_with(|| Color::White).flip();
                }
                _ => {}
            }
        }
        *floor = new_floor;
    }
}

#[aoc_generator(day24)]
fn parse_input(input: &str) -> Vec<Vec<Direction>> {
    InputParser::parse(Rule::file, input)
        .expect("unable to parse input")
        .filter(|p| p.as_rule() == Rule::moves)
        .map(|m| {
            m.into_inner()
                .map(|d| d.as_str().parse::<Direction>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day24, part1)]
fn part1(directions: &[Vec<Direction>]) -> usize {
    let floor = build_floor(directions);
    floor.into_iter().filter(|(_, c)| c == &Color::Black).count()
}
#[aoc(day24, part2)]
fn part2(directions: &[Vec<Direction>]) -> usize {
    let mut floor = build_floor(directions);
    conways_game(&mut floor, 100);
    floor.into_iter().filter(|(_, c)| c == &Color::Black).count()
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn parsing_input() {
        parse_input(INPUT);
    }

    #[test]
    fn running_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 10);
    }

    #[test]
    fn running_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 2208);
    }
}
