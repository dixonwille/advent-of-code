use anyhow::{Ok, Result};

const INPUT: &str = include_str!("inputs/day10.txt");

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

type Parsed = ();

fn parse(_input: &str) -> Result<Parsed> {
    unimplemented!()
}

fn part_a(_signal: Parsed) -> Result<usize> {
    unimplemented!()
}

fn part_b(_signal: Parsed) -> Result<usize> {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {}

    #[test]
    fn test_part_a() {}

    #[test]
    fn test_part_b() {}
}
