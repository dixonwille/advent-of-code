use anyhow::{Error, Result};

const INPUT: &str = include_str!("inputs/day01.txt");

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

type Parsed = Vec<Vec<isize>>;

fn parse(input: &str) -> Result<Parsed> {
    Ok(input
        .lines()
        .map(|i| i.parse::<isize>().ok())
        .fold(
            (Vec::new(), Vec::new()),
            |(mut f, mut int), cal| match cal {
                Some(i) => {
                    int.push(i);
                    (f, int)
                }
                None => {
                    f.push(int);
                    (f, Vec::new())
                }
            },
        )
        .0)
}

fn part_a(elf_calories: Parsed) -> Result<isize> {
    elf_calories
        .into_iter()
        .map(|cal| cal.into_iter().sum::<isize>())
        .max()
        .ok_or(Error::msg("could not find max"))
}

fn part_b(elf_calories: Parsed) -> Result<isize> {
    let mut sums = elf_calories
        .into_iter()
        .map(|cal| cal.into_iter().sum::<isize>())
        .collect::<Vec<_>>();
    sums.sort_unstable();
    sums.reverse();
    Ok(sums.into_iter().take(3).sum::<isize>())
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse(TEST_INPUT).unwrap(),
            vec![
                vec![1000, 2000, 3000],
                vec![4000],
                vec![5000, 6000],
                vec![7000, 8000, 9000],
                vec![10000]
            ]
        );
    }

    #[test]
    fn test_part_a() {
        let parsed = parse(TEST_INPUT).unwrap();
        assert_eq!(part_a(parsed).unwrap(), 24000);
    }

    #[test]
    fn test_part_b() {
        let parsed = parse(TEST_INPUT).unwrap();
        assert_eq!(part_b(parsed).unwrap(), 45000);
    }
}
