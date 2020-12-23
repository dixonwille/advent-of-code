/// https://adventofcode.com/2020/day/20
use pest::{iterators::Pair, Parser};
use std::{collections::HashMap, fmt::Debug, iter::successors, ops::Neg};

enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

enum Flip {
    Horizontal,
    Verticle,
}

#[allow(dead_code)]
enum Rotate {
    ClockWise,
    CounterClockWise,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Tile {
    id: usize,
    rows: usize,
    cols: usize,
    data: Vec<u8>,
}

impl Tile {
    fn new(id: usize, data: Vec<Vec<u8>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        let data = data.into_iter().flatten().collect::<Vec<_>>();
        Tile {
            id,
            rows,
            cols,
            data,
        }
    }

    fn get_edge<'data>(&'data self, edge: Edge) -> Vec<&'data u8> {
        match edge {
            Edge::Top => self
                .data
                .iter()
                .enumerate()
                .filter(|(i, _)| i / self.cols == 0)
                .map(|(_, d)| d)
                .collect(),
            Edge::Bottom => self
                .data
                .iter()
                .enumerate()
                .filter(|(i, _)| i / self.cols == self.rows - 1)
                .map(|(_, d)| d)
                .collect(),
            Edge::Left => self
                .data
                .iter()
                .enumerate()
                .filter(|(i, _)| i % self.cols == 0)
                .map(|(_, d)| d)
                .collect(),
            Edge::Right => self
                .data
                .iter()
                .enumerate()
                .filter(|(i, _)| i % self.cols == self.cols - 1)
                .map(|(_, d)| d)
                .collect(),
        }
    }

    fn flip(&mut self, flip: Flip) {
        match flip {
            Flip::Horizontal => {
                let half = self.data.len() / 2;
                for i in 0..half - (half % self.cols) {
                    let top = i;
                    let bottom = ((self.rows - (i / self.cols) - 1) * self.rows) + i % self.cols;
                    self.data.swap(top, bottom);
                }
            }
            Flip::Verticle => {
                for i in 0..self.data.len() {
                    if i % self.cols >= self.cols / 2 {
                        continue;
                    }
                    let left = i;
                    let right = i + self.cols - (i % self.cols) - 1;
                    self.data.swap(left, right);
                }
            }
        }
    }

    fn transpose(&mut self) {
        for i in 1..self.data.len() - self.cols {
            if i % self.cols <= i / self.cols {
                continue;
            }
            let top_right = i;
            let bottom_left = ((i % self.cols) * self.rows) + (i / self.cols);
            self.data.swap(top_right, bottom_left);
        }
    }

    fn rotate(&mut self, rotate: Rotate) {
        match rotate {
            Rotate::ClockWise => {
                self.transpose();
                self.flip(Flip::Verticle);
            }
            Rotate::CounterClockWise => {
                self.transpose();
                self.flip(Flip::Horizontal);
            }
        }
    }

    fn orientations(&self) -> impl Iterator<Item = Self> {
        let mut flipped = self.clone();
        flipped.flip(Flip::Horizontal);
        successors(Some(self.clone()), |t| {
            let mut t = t.clone();
            t.rotate(Rotate::CounterClockWise);
            Some(t)
        })
        .take(4)
        .chain(successors::<Self, _>(Some(flipped), |t| {
            let mut t = t.clone();
            t.rotate(Rotate::CounterClockWise);
            Some(t)
        }))
        .take(8)
    }
}

struct TileMap {
    map: HashMap<(i32, i32), Tile>,
    left_most: i32,
    right_most: i32,
    top_most: i32,
    bottom_most: i32,
}

static OFFSETS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl TileMap {
    fn new(mut tiles: Vec<Tile>) -> Self {
        let mut tile_map = TileMap {
            map: HashMap::new(),
            left_most: 0,
            right_most: 0,
            top_most: 0,
            bottom_most: 0,
        };
        tile_map.map.insert((0, 0), tiles.pop().unwrap());

        while !tiles.is_empty() {
            'pos: for valid_pos in tile_map.get_valid_positions() {
                for i in 0..tiles.len() {
                    for orientation in tiles[i].orientations() {
                        if tile_map.can_tile_fit(valid_pos, &orientation) {
                            tile_map.map.insert(valid_pos, orientation);
                            tile_map.left_most = tile_map.left_most.min(valid_pos.0);
                            tile_map.right_most = tile_map.right_most.max(valid_pos.0);
                            tile_map.top_most = tile_map.top_most.max(valid_pos.1);
                            tile_map.bottom_most = tile_map.bottom_most.min(valid_pos.1);
                            tiles.swap_remove(i);
                            break 'pos;
                        }
                    }
                }
            }
        }
        tile_map
    }

    fn get_corners(&self) -> Vec<&Tile> {
        vec![
            self.map.get(&(self.right_most, self.top_most)).unwrap(),
            self.map.get(&(self.left_most, self.top_most)).unwrap(),
            self.map.get(&(self.right_most, self.bottom_most)).unwrap(),
            self.map.get(&(self.left_most, self.bottom_most)).unwrap(),
        ]
    }

    fn can_tile_fit(&self, xy: (i32, i32), tile: &Tile) -> bool {
        TileMap::surrounding(xy)
            .filter(|nxy| self.map.contains_key(nxy))
            .all(|nxy| {
                let mapped = self
                    .map
                    .get(&nxy)
                    .expect("filtered on neighbors that exist");
                match (xy.0 - nxy.0, xy.1 - nxy.1) {
                    (-1, 0) => tile.get_edge(Edge::Right) == mapped.get_edge(Edge::Left),
                    (1, 0) => tile.get_edge(Edge::Left) == mapped.get_edge(Edge::Right),
                    (0, -1) => tile.get_edge(Edge::Top) == mapped.get_edge(Edge::Bottom),
                    (0, 1) => tile.get_edge(Edge::Bottom) == mapped.get_edge(Edge::Top),
                    _ => unreachable!("checked souroundings"),
                }
            })
    }

    fn get_valid_positions(&self) -> Vec<(i32, i32)> {
        self.map
            .keys()
            .flat_map(|&xy| TileMap::surrounding(xy))
            .filter(|xy| !self.map.contains_key(xy))
            .collect()
    }

    fn surrounding((x, y): (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
        OFFSETS.iter().map(move |(dx, dy)| (x + dx, y + dy))
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Map {
    data: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl From<TileMap> for Map {
    fn from(tile_map: TileMap) -> Self {
        let random_tile = tile_map.get_corners()[0]; //this is to get the size of the tiles
        let tile_cols = random_tile.cols - 2;
        let tile_rows = random_tile.rows - 2;
        let cols = (1 + (tile_map.left_most - tile_map.right_most).abs() as usize) * tile_cols;
        let rows = (1 + (tile_map.top_most - tile_map.bottom_most).abs() as usize) * tile_rows;
        let mut data: Vec<u8> = Vec::with_capacity(rows * cols);
        for i in 0..rows * cols {
            let tile_x = ((i % cols) / tile_cols) as i32 + tile_map.left_most;
            let tile_y = (((i / cols) / tile_rows) as i32).neg() + tile_map.top_most;
            let tile = tile_map.map.get(&(tile_x, tile_y)).unwrap();
            let data_point =
                ((((i / cols) % tile_rows) + 1) * random_tile.cols) + ((i % tile_cols) + 1);
            data.push(*tile.data.get(data_point).unwrap());
        }
        Map { rows, cols, data }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, d) in self.data.iter().enumerate() {
            if i != 0 && i % self.cols == 0 {
                writeln!(f)?;
            }
            match d {
                0 => write!(f, ".")?,
                1 => write!(f, "#")?,
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}

static SEA_MONSTER: &str = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";

// returns a vec of the changes for "waves" to be
// as well as width and height
fn parse_sea_monster(input: &str, cols: usize) -> (Vec<usize>, usize, usize) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let diff = input
        .as_bytes()
        .iter()
        .filter(|&&b| b != b'\n' && b != b'\r')
        .enumerate()
        .filter(|(_, &b)| b != b' ')
        .map(|(i, _)| i % width + ((i / width) * cols))
        .collect::<Vec<_>>();
    (diff, width, height)
}

impl Map {
    fn number_of_sea_monsers(&self) -> usize {
        let (diffs, width, height) = parse_sea_monster(SEA_MONSTER, self.cols);
        for map in self.orientations() {
            let monsters = (0..(map.rows - height + 1) * map.cols)
                .filter(|i| (self.cols - i % self.cols) >= width)
                .filter(|&i| {
                    diffs
                        .iter()
                        .map(|&d| map.data.get(d + i).unwrap())
                        .all(|&d| d == 1)
                })
                .collect::<Vec<_>>();
            if !monsters.is_empty() {
                return monsters.len();
            }
        }
        0
    }

    fn flip(&mut self, flip: Flip) {
        match flip {
            Flip::Horizontal => {
                let half = self.data.len() / 2;
                for i in 0..half - (half % self.cols) {
                    let top = i;
                    let bottom = ((self.rows - (i / self.rows) - 1) * self.rows) + i % self.cols;
                    self.data.swap(top, bottom);
                }
            }
            Flip::Verticle => {
                for i in 0..self.data.len() {
                    if i % self.cols >= self.cols / 2 {
                        continue;
                    }
                    let left = i;
                    let right = i + self.cols - (i % self.cols) - 1;
                    self.data.swap(left, right);
                }
            }
        }
    }

    fn transpose(&mut self) {
        for i in 1..self.data.len() - self.cols {
            if i % self.cols <= i / self.rows {
                continue;
            }
            let top_right = i;
            let bottom_left = ((i % self.cols) * self.rows) + (i / self.rows);
            self.data.swap(top_right, bottom_left);
        }
    }

    fn rotate(&mut self, rotate: Rotate) {
        match rotate {
            Rotate::ClockWise => {
                self.transpose();
                self.flip(Flip::Verticle);
            }
            Rotate::CounterClockWise => {
                self.transpose();
                self.flip(Flip::Horizontal);
            }
        }
    }

    fn orientations(&self) -> impl Iterator<Item = Self> {
        let mut flipped = self.clone();
        flipped.flip(Flip::Horizontal);
        successors(Some(self.clone()), |t| {
            let mut t = t.clone();
            t.rotate(Rotate::CounterClockWise);
            Some(t)
        })
        .take(4)
        .chain(successors::<Self, _>(Some(flipped), |t| {
            let mut t = t.clone();
            t.rotate(Rotate::CounterClockWise);
            Some(t)
        }))
        .take(8)
    }
}

#[derive(Parser)]
#[grammar = "day20.pest"]
struct InputParser;

#[aoc_generator(day20)]
fn parse_input(input: &str) -> Vec<Tile> {
    let rules = InputParser::parse(Rule::file, input).expect("could not parse file");
    rules
        .filter(|r| r.as_rule() == Rule::tile)
        .map(parse_tile)
        .collect()
}

fn parse_tile(rule: Pair<Rule>) -> Tile {
    let mut inners = rule.into_inner();
    let id = inners
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    let data = inners.map(parse_row).collect::<Vec<_>>();
    Tile::new(id, data)
}

fn parse_row(rule: Pair<Rule>) -> Vec<u8> {
    rule.into_inner()
        .map(|c| match c.as_str() {
            "#" => 1,
            "." => 0,
            _ => unreachable!(),
        })
        .collect::<Vec<u8>>()
}

#[aoc(day20, part1)]
fn part1(tiles: &[Tile]) -> usize {
    let map = TileMap::new(tiles.to_owned());
    map.get_corners().into_iter().map(|t| t.id).product()
}

#[aoc(day20, part2)]
#[allow(clippy::clippy::naive_bytecount)]
fn part2(tiles: &[Tile]) -> usize {
    let map: Map = TileMap::new(tiles.to_owned()).into();
    let num_monsters = map.number_of_sea_monsers();
    let (diffs, _, _) = parse_sea_monster(SEA_MONSTER, map.cols);
    let not_waves = num_monsters * diffs.len();
    map.data.iter().filter(|&&b| b == 1).count() - not_waves
}

#[cfg(test)]
mod test {
    use super::*;

    static TILES: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    fn parsing_input() {
        parse_input(TILES);
    }

    #[test]
    fn running_part1() {
        let tiles = parse_input(TILES);
        assert_eq!(part1(&tiles), 20899048083289)
    }

    #[test]
    fn running_part2() {
        let tiles = parse_input(TILES);
        assert_eq!(part2(&tiles), 273)
    }
}
