use anyhow::{Ok, Result};
use pathfinding::prelude::bfs;

const INPUT: &str = include_str!("inputs/day12.txt");

pub fn run_part_a() -> Result<()> {
    let i = parse(INPUT)?;
    println!("{}", part_a(i)?);
    Ok(())
}

pub fn run_part_b() -> Result<()> {
    let i = parse(INPUT)?;
    println!("{}", part_b(i)?);
    Ok(())
}

type Parsed = Vec<Vec<Tile>>;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

#[derive(Debug, PartialEq)]
enum Tile {
    Start,
    End,
    Slope(u8),
}

impl Tile {
    fn elevation(&self) -> usize {
        match self {
            Tile::Start => 1,
            Tile::End => 26,
            Tile::Slope(e) => (*e) as usize,
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value == 'S' {
            Ok(Tile::Start)
        } else if value == 'E' {
            Ok(Tile::End)
        } else if value.is_ascii_lowercase() {
            Ok(Tile::Slope(u8::try_from(value)? & 0x1F))
        } else {
            Err(anyhow::Error::msg("unsupported character"))
        }
    }
}

fn parse(input: &str) -> Result<Parsed> {
    input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }
            Some(l.chars().map(Tile::try_from).collect::<Result<Vec<_>>>())
        })
        .collect::<Result<Vec<_>>>()
}

fn find_start_end(map: &Parsed) -> (Pos, Pos) {
    let mut start: Option<Pos> = None;
    let mut end: Option<Pos> = None;
    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if col == &Tile::Start {
                start = Some(Pos(x, y))
            } else if col == &Tile::End {
                end = Some(Pos(x, y))
            }
            if start.is_some() && end.is_some() {
                break;
            }
        }
    }
    (start.unwrap(), end.unwrap())
}

fn find_successors_up(map: &Parsed, node: &Pos) -> Vec<Pos> {
    let &Pos(x, y) = node;
    let cur_ele = map[y][x].elevation();
    let mut positions = Vec::new();
    if x > 0 {
        positions.push(Pos(x - 1, y));
    }
    if x < map[0].len() - 1 {
        positions.push(Pos(x + 1, y));
    }
    if y > 0 {
        positions.push(Pos(x, y - 1));
    }
    if y < map.len() - 1 {
        positions.push(Pos(x, y + 1));
    }
    positions
        .into_iter()
        .filter(|p| map[p.1][p.0].elevation() <= cur_ele + 1)
        .collect()
}

fn part_a(map: Parsed) -> Result<usize> {
    let (start, end) = find_start_end(&map);
    let res = bfs(&start, |p| find_successors_up(&map, p), |p| p == &end)
        .ok_or(anyhow::Error::msg("no route found"))?;
    Ok(res.len() - 1)
}

fn find_successors_down(map: &Parsed, node: &Pos) -> Vec<Pos> {
    let &Pos(x, y) = node;
    let cur_ele = map[y][x].elevation();
    let mut positions = Vec::new();
    if x > 0 {
        positions.push(Pos(x - 1, y));
    }
    if x < map[0].len() - 1 {
        positions.push(Pos(x + 1, y));
    }
    if y > 0 {
        positions.push(Pos(x, y - 1));
    }
    if y < map.len() - 1 {
        positions.push(Pos(x, y + 1));
    }
    positions
        .into_iter()
        .filter(|p| map[p.1][p.0].elevation() >= cur_ele - 1)
        .collect()
}

fn part_b(map: Parsed) -> Result<usize> {
    let (_, end) = find_start_end(&map);
    let res = bfs(
        &end,
        |p| find_successors_down(&map, p),
        |p| map[p.1][p.0].elevation() == 1,
    )
    .ok_or(anyhow::Error::msg("no route found"))?;
    Ok(res.len() - 1)
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi

";

    #[test]
    fn test_parse() {
        let parsed = parse(TEST_INPUT).unwrap();
        assert_eq!(
            parsed,
            vec![
                vec![
                    Tile::Start,
                    Tile::Slope(1),
                    Tile::Slope(2),
                    Tile::Slope(17),
                    Tile::Slope(16),
                    Tile::Slope(15),
                    Tile::Slope(14),
                    Tile::Slope(13),
                ],
                vec![
                    Tile::Slope(1),
                    Tile::Slope(2),
                    Tile::Slope(3),
                    Tile::Slope(18),
                    Tile::Slope(25),
                    Tile::Slope(24),
                    Tile::Slope(24),
                    Tile::Slope(12),
                ],
                vec![
                    Tile::Slope(1),
                    Tile::Slope(3),
                    Tile::Slope(3),
                    Tile::Slope(19),
                    Tile::Slope(26),
                    Tile::End,
                    Tile::Slope(24),
                    Tile::Slope(11),
                ],
                vec![
                    Tile::Slope(1),
                    Tile::Slope(3),
                    Tile::Slope(3),
                    Tile::Slope(20),
                    Tile::Slope(21),
                    Tile::Slope(22),
                    Tile::Slope(23),
                    Tile::Slope(10),
                ],
                vec![
                    Tile::Slope(1),
                    Tile::Slope(2),
                    Tile::Slope(4),
                    Tile::Slope(5),
                    Tile::Slope(6),
                    Tile::Slope(7),
                    Tile::Slope(8),
                    Tile::Slope(9),
                ],
            ]
        );
    }

    #[test]
    fn test_part_a() {
        let parsed = parse(TEST_INPUT).unwrap();
        assert_eq!(part_a(parsed).unwrap(), 31);
    }

    #[test]
    fn test_part_b() {
        let parsed = parse(TEST_INPUT).unwrap();
        assert_eq!(part_b(parsed).unwrap(), 29);
    }
}
