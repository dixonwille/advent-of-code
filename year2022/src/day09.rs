use std::collections::HashSet;

use anyhow::{Ok, Result};

const INPUT: &str = include_str!("inputs/day09.txt");

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

#[derive(Debug, PartialEq)]
enum Move {
    Up(isize),
    Down(isize),
    Right(isize),
    Left(isize),
}

type Parsed = Vec<Move>;

fn parse(input: &str) -> Result<Parsed> {
    Ok(input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }
            let parts = l.split(' ').collect::<Vec<_>>();
            let steps: isize = parts[1].parse().unwrap();
            Some(match parts[0] {
                "U" => Move::Up(steps),
                "D" => Move::Down(steps),
                "L" => Move::Left(steps),
                "R" => Move::Right(steps),
                _ => unreachable!(),
            })
        })
        .collect::<Vec<_>>())
}

fn move_tail(head: &(isize, isize), tail: &mut (isize, isize)) {
    let is_adjacent = (head.0 - 1..=head.0 + 1)
        .map(|x| (head.1 - 1..=head.1 + 1).map(move |y| (x, y)))
        .flatten()
        .any(|p| p.0 == tail.0 && p.1 == tail.1);
    if is_adjacent {
        return;
    }
    match (head.0.cmp(&tail.0), head.1.cmp(&tail.1)) {
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => {
            unreachable!("handled in is_adjacent statement")
        }
        (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => tail.0 -= 1,
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => tail.0 += 1,
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => tail.1 -= 1,
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => tail.1 += 1,
        (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => {
            tail.0 -= 1;
            tail.1 -= 1;
        }
        (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => {
            tail.0 -= 1;
            tail.1 += 1;
        }
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => {
            tail.0 += 1;
            tail.1 -= 1;
        }
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => {
            tail.0 += 1;
            tail.1 += 1
        }
    }
}

fn move_tails(knots: &mut [(isize, isize); 10]) {
    for i in 1..10 {
        let h = knots.get(i - 1).unwrap();
        let mut t = knots.get(i).unwrap().clone();
        move_tail(&h, &mut t);
        knots[i] = t;
    }
}

fn part_a(movement: Parsed) -> Result<usize> {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut set = HashSet::new();
    for m in movement {
        match m {
            Move::Up(s) => {
                for _ in 0..s {
                    head.1 += 1;
                    move_tail(&head, &mut tail);
                    set.insert(tail);
                }
            }
            Move::Down(s) => {
                for _ in 0..s {
                    head.1 -= 1;
                    move_tail(&head, &mut tail);
                    set.insert(tail);
                }
            }
            Move::Right(s) => {
                for _ in 0..s {
                    head.0 += 1;
                    move_tail(&head, &mut tail);
                    set.insert(tail);
                }
            }
            Move::Left(s) => {
                for _ in 0..s {
                    head.0 -= 1;
                    move_tail(&head, &mut tail);
                    set.insert(tail);
                }
            }
        }
    }
    Ok(set.len())
}

fn part_b(movement: Parsed) -> Result<usize> {
    let mut knots: [(isize, isize); 10] = [(0, 0); 10];
    let mut set = HashSet::new();
    for m in movement {
        match m {
            Move::Up(s) => {
                for _ in 0..s {
                    knots[0].1 += 1;
                    move_tails(&mut knots);
                    set.insert(knots[9]);
                }
            }
            Move::Down(s) => {
                for _ in 0..s {
                    knots[0].1 -= 1;
                    move_tails(&mut knots);
                    set.insert(knots[9]);
                }
            }
            Move::Right(s) => {
                for _ in 0..s {
                    knots[0].0 += 1;
                    move_tails(&mut knots);
                    set.insert(knots[9]);
                }
            }
            Move::Left(s) => {
                for _ in 0..s {
                    knots[0].0 -= 1;
                    move_tails(&mut knots);
                    set.insert(knots[9]);
                }
            }
        }
    }
    Ok(set.len())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2

";

    const TEST_INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20

";

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(TEST_INPUT).unwrap(),
            vec![
                Move::Right(4),
                Move::Up(4),
                Move::Left(3),
                Move::Down(1),
                Move::Right(4),
                Move::Down(1),
                Move::Left(5),
                Move::Right(2),
            ]
        )
    }

    #[test]
    fn test_part_a() {
        let parsed = parse(TEST_INPUT).unwrap();
        assert_eq!(part_a(parsed).unwrap(), 13)
    }

    #[test]
    fn test_part_b() {
        let parsed = parse(TEST_INPUT).unwrap();
        assert_eq!(part_b(parsed).unwrap(), 1);
        let parsed = parse(TEST_INPUT_2).unwrap();
        assert_eq!(part_b(parsed).unwrap(), 36);
    }
}
