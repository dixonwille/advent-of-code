use anyhow::{Error, Ok, Result};

const INPUT: &str = include_str!("inputs/day06.txt");

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

type Parsed = Vec<char>;

fn parse(input: &str) -> Result<Parsed> {
    Ok(input.chars().collect())
}

fn part_a(signal: Parsed) -> Result<usize> {
    for (i, window) in signal.windows(4).enumerate() {
        let mut w = window.to_owned();
        w.sort_unstable();
        w.dedup();
        if w.len() == 4 {
            return Ok(i + 4);
        }
    }
    Err(Error::msg("starting sequence not found"))
}

fn part_b(signal: Parsed) -> Result<usize> {
    for (i, window) in signal.windows(14).enumerate() {
        let mut w = window.to_owned();
        w.sort_unstable();
        w.dedup();
        if w.len() == 14 {
            return Ok(i + 14);
        }
    }
    Err(Error::msg("starting sequence not found"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_a() {
        let parsed = parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap();
        assert_eq!(part_a(parsed).unwrap(), 7);

        let parsed = parse("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap();
        assert_eq!(part_a(parsed).unwrap(), 5);

        let parsed = parse("nppdvjthqldpwncqszvftbrmjlhg").unwrap();
        assert_eq!(part_a(parsed).unwrap(), 6);

        let parsed = parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap();
        assert_eq!(part_a(parsed).unwrap(), 10);

        let parsed = parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap();
        assert_eq!(part_a(parsed).unwrap(), 11);
    }

    #[test]
    fn test_part_b() {
        let parsed = parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap();
        assert_eq!(part_b(parsed).unwrap(), 19);

        let parsed = parse("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap();
        assert_eq!(part_b(parsed).unwrap(), 23);

        let parsed = parse("nppdvjthqldpwncqszvftbrmjlhg").unwrap();
        assert_eq!(part_b(parsed).unwrap(), 23);

        let parsed = parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap();
        assert_eq!(part_b(parsed).unwrap(), 29);

        let parsed = parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap();
        assert_eq!(part_b(parsed).unwrap(), 26);
    }
}
