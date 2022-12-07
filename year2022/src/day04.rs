use std::ops::RangeInclusive;

use anyhow::Result;

const INPUT: &str = include_str!("inputs/day04.txt");

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

type Parsed = Vec<(RangeInclusive<isize>, RangeInclusive<isize>)>;

fn parse(input: &str) -> Result<Parsed> {
    Ok(input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            let mut r = line
                .split(',')
                .map(|r| r.split('-').map(|n| n.parse::<isize>().unwrap()))
                .flatten();
            Some((
                RangeInclusive::new(r.next().unwrap(), r.next().unwrap()),
                RangeInclusive::new(r.next().unwrap(), r.next().unwrap()),
            ))
        })
        .collect())
}

#[inline]
fn fully_contains(a: &RangeInclusive<isize>, b: &RangeInclusive<isize>) -> bool {
    (b.contains(a.start()) && b.contains(a.end())) || (a.contains(b.start()) && a.contains(b.end()))
}

fn part_a(assignments: Parsed) -> Result<isize> {
    Ok(assignments
        .into_iter()
        .map(|(a, b)| if fully_contains(&a, &b) { 1 } else { 0 })
        .sum::<isize>())
}

#[inline]
fn semi_contains(a: &RangeInclusive<isize>, b: &RangeInclusive<isize>) -> bool {
    b.contains(a.start()) || b.contains(a.end()) || a.contains(b.start()) || a.contains(b.end())
}

fn part_b(assignments: Parsed) -> Result<isize> {
    Ok(assignments
        .into_iter()
        .map(|(a, b)| if semi_contains(&a, &b) { 1 } else { 0 })
        .sum::<isize>())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8

";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse(TEST_INPUT).unwrap(),
            vec![
                (RangeInclusive::new(2, 4), RangeInclusive::new(6, 8)),
                (RangeInclusive::new(2, 3), RangeInclusive::new(4, 5)),
                (RangeInclusive::new(5, 7), RangeInclusive::new(7, 9)),
                (RangeInclusive::new(2, 8), RangeInclusive::new(3, 7)),
                (RangeInclusive::new(6, 6), RangeInclusive::new(4, 6)),
                (RangeInclusive::new(2, 6), RangeInclusive::new(4, 8))
            ]
        )
    }

    #[test]
    fn test_part_a() {
        let parsed = parse(TEST_INPUT).unwrap();
        assert_eq!(part_a(parsed).unwrap(), 2);
    }

    #[test]
    fn test_part_b() {
        let parsed = parse(TEST_INPUT).unwrap();
        assert_eq!(part_b(parsed).unwrap(), 4);
    }
}
