use crate::Solutions;
use lib_aoc::prelude::*;

#[derive(Debug)]
pub struct Map {
    x: usize,
    y: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn to_walker(&self, start: usize, start_dir: Direction) -> MapWalker {
        MapWalker::new(start, self, start_dir)
    }

    fn find_loop(&self) -> (Direction, Vec<usize>) {
        let (start_idx, _) = self
            .tiles
            .iter()
            .enumerate()
            .find(|(_, &t)| t == Tile::Start)
            .unwrap();

        // Need to check South first since I am using that to determine Ray-Casting
        if let Some(c) = self.to_walker(start_idx, Direction::South).get_loop() {
            return (Direction::South, c);
        }
        if let Some(c) = self.to_walker(start_idx, Direction::North).get_loop() {
            return (Direction::North, c);
        }
        if let Some(c) = self.to_walker(start_idx, Direction::East).get_loop() {
            return (Direction::East, c);
        }
        if let Some(c) = self.to_walker(start_idx, Direction::West).get_loop() {
            return (Direction::West, c);
        }
        unreachable!()
    }
}

struct MapWalker<'a> {
    curr: usize,
    curr_tile: &'a Tile,
    from_dir: Direction,
    map: &'a Map,
}

impl<'a> MapWalker<'a> {
    fn new(start: usize, map: &'a Map, start_dir: Direction) -> MapWalker {
        let curr_tile = map.tiles.get(start).unwrap();
        MapWalker {
            curr: start,
            from_dir: start_dir,
            map,
            curr_tile,
        }
    }

    fn get_loop(&mut self) -> Option<Vec<usize>> {
        let mut idxs = vec![self.curr];
        if self.curr_tile == &Tile::Start {
            let mv = match self.from_dir {
                Direction::North => self.north(),
                Direction::South => self.south(),
                Direction::East => self.east(),
                Direction::West => self.west(),
            };
            match mv {
                Some(Tile::Ground) | None => return None,
                _ => {}
            };
            idxs.push(self.curr);
        }
        loop {
            if self.curr_tile == &Tile::Start {
                break;
            }

            let mv = match (self.curr_tile, &self.from_dir) {
                (Tile::NorthToSouth, Direction::North) => self.south(),
                (Tile::NorthToSouth, Direction::South) => self.north(),
                (Tile::EastToWest, Direction::East) => self.west(),
                (Tile::EastToWest, Direction::West) => self.east(),
                (Tile::NorthToEast, Direction::North) => self.east(),
                (Tile::NorthToEast, Direction::East) => self.north(),
                (Tile::NorthToWest, Direction::North) => self.west(),
                (Tile::NorthToWest, Direction::West) => self.north(),
                (Tile::SouthToEast, Direction::South) => self.east(),
                (Tile::SouthToEast, Direction::East) => self.south(),
                (Tile::SouthToWest, Direction::South) => self.west(),
                (Tile::SouthToWest, Direction::West) => self.south(),
                (Tile::Start, _) => unreachable!(),
                _ => None,
            };

            if mv.is_none() {
                return None;
            }
            idxs.push(self.curr);
        }
        Some(idxs)
    }

    fn north(&mut self) -> Option<&'a Tile> {
        if self.curr / self.map.x == 0 {
            None
        } else {
            self.curr = self.curr - self.map.x;
            self.curr_tile = self.map.tiles.get(self.curr).unwrap();
            self.from_dir = Direction::South;
            Some(self.curr_tile)
        }
    }

    fn south(&mut self) -> Option<&'a Tile> {
        if self.curr / self.map.x == self.map.y - 1 {
            None
        } else {
            self.curr = self.curr + self.map.x;
            self.curr_tile = self.map.tiles.get(self.curr).unwrap();
            self.from_dir = Direction::North;
            Some(self.curr_tile)
        }
    }

    fn east(&mut self) -> Option<&'a Tile> {
        if self.curr % self.map.x >= self.map.x - 1 {
            None
        } else {
            self.curr = self.curr + 1;
            self.curr_tile = self.map.tiles.get(self.curr).unwrap();
            self.from_dir = Direction::West;
            Some(self.curr_tile)
        }
    }

    fn west(&mut self) -> Option<&'a Tile> {
        if self.curr % self.map.x == 0 {
            None
        } else {
            self.curr = self.curr - 1;
            self.curr_tile = self.map.tiles.get(self.curr).unwrap();
            self.from_dir = Direction::East;
            Some(self.curr_tile)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    NorthToSouth,
    EastToWest,
    NorthToEast,
    NorthToWest,
    SouthToEast,
    SouthToWest,
    Start,
    Ground,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Tile::NorthToSouth),
            '-' => Ok(Tile::EastToWest),
            'L' => Ok(Tile::NorthToEast),
            'J' => Ok(Tile::NorthToWest),
            '7' => Ok(Tile::SouthToWest),
            'F' => Ok(Tile::SouthToEast),
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::Start),
            _ => Err("Unkonwn tile"),
        }
    }
}

impl Solution<DAY_10> for Solutions {
    type Input<'i> = Map;
    type Output = usize;

    fn parse(puzzle: &str) -> Map {
        let lines = puzzle
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| Tile::try_from(c))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        Map {
            x: lines.get(0).unwrap().len(),
            y: lines.len(),
            tiles: lines.into_iter().flatten().collect(),
        }
    }

    fn part_one(input: &Map) -> usize {
        let (_, c) = input.find_loop();
        c.len() / 2
    }
    fn part_two(input: &Map) -> usize {
        let (d, mut c) = input.find_loop();
        c.sort();
        let mut walls = 0usize;
        let mut count = 0usize;
        for (i, tile) in input.tiles.iter().enumerate() {
            if i % input.x == 0 && walls % 2 == 1 {
                panic!("In the loop on a new line! You messed up!");
            }
            // Ray-Cast bottom half of cell
            if let Ok(_) = c.binary_search(&i) {
                if tile == &Tile::NorthToSouth
                    || tile == &Tile::SouthToEast
                    || tile == &Tile::SouthToWest
                    || (tile == &Tile::Start && d == Direction::South)
                {
                    walls += 1;
                }
                continue;
            }
            count += walls % 2
        }
        count
    }
}

impl Test<DAY_10> for Solutions {
    fn expected(part: bool) -> usize {
        match part {
            PART_ONE => 8,
            PART_TWO => 10,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solutions;
    use lib_aoc::prelude::*;

    derive_tests!(Solutions, DAY_10);
}
