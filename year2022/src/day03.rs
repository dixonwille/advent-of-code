use anyhow::Result;

const INPUT: &str = include_str!("inputs/day03.txt");

pub fn run_part_a() -> Result<()> {
    let i = parse_a(INPUT)?;
    println!("{}", part_a(i)?);
    Ok(())
}

pub fn run_part_b() -> Result<()> {
    let i = parse_b(INPUT)?;
    println!("{}", part_b(i)?);
    Ok(())
}

type ParsedA = Vec<(Vec<char>, Vec<char>)>;

fn parse_a(input: &str) -> Result<ParsedA> {
    Ok(input
        .lines()
        .filter_map(|i| {
            if i.is_empty() {
                return None;
            }
            let rucksack = i.chars().collect::<Vec<_>>();
            let half = rucksack.len() / 2;
            Some((
                rucksack[0..half].to_owned(),
                rucksack[half..rucksack.len()].to_owned(),
            ))
        })
        .collect())
}

#[inline]
fn priority(c: char) -> u8 {
    (u8::try_from(c).unwrap() & 0x1F) + (u8::from(c.is_ascii_uppercase()) * 26)
}

fn part_a(sacks: ParsedA) -> Result<isize> {
    Ok(sacks
        .into_iter()
        .map(|(mut compa, mut compb)| {
            compa.sort_unstable();
            compb.sort_unstable();
            let mut a_iter = compa.into_iter();
            let mut b_iter = compb.into_iter();
            let mut a = a_iter.next();
            let mut b = b_iter.next();
            while a.is_some() && b.is_some() {
                if a == b {
                    return priority(a.unwrap()) as isize;
                }
                if a > b {
                    b = b_iter.next();
                }
                if a < b {
                    a = a_iter.next();
                }
            }
            0
        })
        .sum::<isize>())
}

type ParsedB = Vec<Vec<char>>;
fn parse_b(input: &str) -> Result<ParsedB> {
    Ok(input
        .lines()
        .filter_map(|i| {
            if i.is_empty() {
                return None;
            }
            Some(i.chars().collect::<Vec<_>>())
        })
        .collect())
}

fn part_b(sacks: ParsedB) -> Result<isize> {
    Ok(sacks
        .chunks(3)
        .into_iter()
        .map(|group| {
            let mut groupa = group[0].to_owned();
            groupa.sort_unstable();
            let mut a_iter = groupa.into_iter();
            let mut a = a_iter.next();

            let mut groupb = group[1].to_owned();
            groupb.sort_unstable();
            let mut b_iter = groupb.into_iter();
            let mut b = b_iter.next();

            let mut groupc = group[2].to_owned();
            groupc.sort_unstable();
            let mut c_iter = groupc.into_iter();
            let mut c = c_iter.next();

            while a.is_some() && b.is_some() && c.is_some() {
                if a == b && b == c {
                    return priority(a.unwrap()) as isize;
                }
                let m = vec![a.unwrap(), b.unwrap(), c.unwrap()]
                    .into_iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| a.cmp(b))
                    .unwrap();
                match m.0 {
                    0 => a = a_iter.next(),
                    1 => b = b_iter.next(),
                    2 => c = c_iter.next(),
                    _ => unreachable!("the vec is always of size 3"),
                }
            }
            0
        })
        .sum::<isize>())
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_a(TEST_INPUT).unwrap(),
            vec![
                (
                    "vJrwpWtwJgWr".chars().collect::<Vec<_>>(),
                    "hcsFMMfFFhFp".chars().collect::<Vec<_>>(),
                ),
                (
                    "jqHRNqRjqzjGDLGL".chars().collect::<Vec<_>>(),
                    "rsFMfFZSrLrFZsSL".chars().collect::<Vec<_>>()
                ),
                (
                    "PmmdzqPrV".chars().collect::<Vec<_>>(),
                    "vPwwTWBwg".chars().collect::<Vec<_>>()
                ),
                (
                    "wMqvLMZHhHMvwLH".chars().collect::<Vec<_>>(),
                    "jbvcjnnSBnvTQFn".chars().collect::<Vec<_>>()
                ),
                (
                    "ttgJtRGJ".chars().collect::<Vec<_>>(),
                    "QctTZtZT".chars().collect::<Vec<_>>()
                ),
                (
                    "CrZsJsPPZsGz".chars().collect::<Vec<_>>(),
                    "wwsLwLmpwMDw".chars().collect::<Vec<_>>()
                )
            ]
        );
        assert_eq!(
            parse_b(TEST_INPUT).unwrap(),
            vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect::<Vec<_>>(),
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"
                    .chars()
                    .collect::<Vec<_>>(),
                "PmmdzqPrVvPwwTWBwg".chars().collect::<Vec<_>>(),
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".chars().collect::<Vec<_>>(),
                "ttgJtRGJQctTZtZT".chars().collect::<Vec<_>>(),
                "CrZsJsPPZsGzwwsLwLmpwMDw".chars().collect::<Vec<_>>(),
            ]
        );
    }

    #[test]
    fn test_priority() {
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('Z'), 52);
        assert_eq!(priority('z'), 26);
    }

    #[test]
    fn test_part_a() {
        let parsed = parse_a(TEST_INPUT).unwrap();
        assert_eq!(part_a(parsed).unwrap(), 157);
    }

    #[test]
    fn test_part_b() {
        let parsed = parse_b(TEST_INPUT).unwrap();
        assert_eq!(part_b(parsed).unwrap(), 70);
    }
}
