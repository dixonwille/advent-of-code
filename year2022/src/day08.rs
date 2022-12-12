use anyhow::{Ok, Result};

const INPUT: &str = include_str!("inputs/day08.txt");

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

type Parsed = (usize, usize, Vec<u8>);

fn parse(input: &str) -> Result<Parsed> {
    Ok(input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }
            Some(l.chars().map(|c| c.to_digit(10).unwrap() as u8))
        })
        .fold(
            (0usize, 0usize, Vec::new()),
            |(mut x, mut y, mut trees), t| {
                y += 1;
                trees.extend(t);
                if x == 0 {
                    x = trees.len();
                }
                (x, y, trees)
            },
        ))
}

fn part_a((x_max, y_max, trees): Parsed) -> Result<usize> {
    Ok(trees
        .iter()
        .enumerate()
        .filter_map(|(i, t)| {
            let x = i % x_max; //2
            let y = i / x_max; //1
            if y == 0 || y == y_max - 1 || x == 0 || x == x_max - 1 {
                return None;
            }
            let up = (0..y).map(|v| trees[v * x_max + x]).max().unwrap();
            let down = (y + 1..y_max).map(|v| trees[v * x_max + x]).max().unwrap();
            let left = (0..x).map(|h| trees[h + y * x_max]).max().unwrap();
            let right = (x + 1..x_max).map(|h| trees[h + y * x_max]).max().unwrap();
            if &up < t || &down < t || &left < t || &right < t {
                Some(t)
            } else {
                None
            }
        })
        .count()
        + (x_max * 2)
        + (y_max * 2)
        - 4)
}

fn part_b((x_max, y_max, trees): Parsed) -> Result<usize> {
    Ok(trees
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let x = i % x_max; //2
            let y = i / x_max; //1
            if y == 0 || y == y_max - 1 || x == 0 || x == x_max - 1 {
                return 0;
            }
            let up = (0..y)
                .rev()
                .map(|v| trees[v * x_max + x])
                .enumerate()
                .find(|(_, u)| u >= t)
                .or(Some((y - 1, 0)))
                .unwrap()
                .0
                + 1;
            let down = (y + 1..y_max)
                .map(|v| trees[v * x_max + x])
                .enumerate()
                .find(|(_, u)| u >= t)
                .or(Some((y_max - y - 2, 0)))
                .unwrap()
                .0
                + 1;
            let left = (0..x)
                .rev()
                .map(|h| trees[h + y * x_max])
                .enumerate()
                .find(|(_, u)| u >= t)
                .or(Some((x - 1, 0)))
                .unwrap()
                .0
                + 1;
            let right = (x + 1..x_max)
                .map(|h| trees[h + y * x_max])
                .enumerate()
                .find(|(_, u)| u >= t)
                .or(Some((x_max - x - 2, 0)))
                .unwrap()
                .0
                + 1;
            up * down * left * right
        })
        .max()
        .unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "30373
25512
65332
33549
35390

";

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(TEST_INPUT).unwrap(),
            (
                5,
                5,
                "3037325512653323354935390"
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            )
        );
    }

    #[test]
    fn test_part_a() {
        let parsed = parse(TEST_INPUT).unwrap();
        assert_eq!(part_a(parsed).unwrap(), 21);
    }

    #[test]
    fn test_part_b() {
        let parsed = parse(TEST_INPUT).unwrap();
        assert_eq!(part_b(parsed).unwrap(), 8);
    }
}
