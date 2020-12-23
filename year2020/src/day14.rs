/// https://adventofcode.com/2020/day/14
use std::collections::HashMap;

use bit::BitIndex;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    ChnageMask(Mask),
    WriteMem((u64, u64)),
}

#[derive(Debug, Eq, PartialEq)]
struct Mask {
    inner: String,
    or: u64,
    and: u64,
    floating: Vec<u64>,
}

impl Mask {
    fn new(s: String) -> Self {
        let (or, and) = mask_or_and(&s);
        let floating = get_floating(&s);
        Mask {
            inner: s,
            or,
            and,
            floating,
        }
    }

    fn apply_v1(&self, value: u64) -> u64 {
        let mut amt = value | self.or;
        amt &= self.and;
        amt
    }

    // mask[i] == 0 don't change
    // mask[i] == 1 overwrite with 1
    // mask[i] == X floating (both 1 and 0)
    // Can apply the OR mask to tick of rule 1 and 2
    fn apply_v2(&self, value: u64) -> Vec<u64> {
        let orig = value | self.or; // doesn't change 0 and overwrites where 1
        let mut addrs = Vec::new();

        // gives me all possible values from all 0s to all 1s for the floating mask
        // size = 2 ^ num of 'X's
        for i in 0..(2 as u64).pow(self.floating.len() as u32) {
            let mut orig_copy = orig;
            for (j, f) in self.floating.iter().enumerate() {
                orig_copy.set_bit(*f as usize, i.bit(j));
            }
            addrs.push(orig_copy);
        }
        addrs
    }
}

fn get_floating(mask: &str) -> Vec<u64> {
    // mask is 36 characters long and I need LSB to be indexed at 0
    mask.as_bytes()
        .iter()
        .enumerate()
        .filter(|(_, c)| c == &&b'X')
        .map(|(i, _)| 35 - i as u64)
        .collect()
}

fn mask_or_and(mask: &str) -> (u64, u64) {
    let (mut or, mut and) = (0, 0);
    for m in mask.as_bytes() {
        or <<= 1;
        and <<= 1;
        match m {
            b'X' => {
                and += 0b1;
            }
            b'1' => {
                or += 0b1;
                and += 0b1;
            }
            _ => {} // for b'0' do nothing to either
        }
    }
    (or, and)
}

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Vec<Operation> {
    let (_, out) = separated_list1(
        tag("\n"),
        alt((
            preceded(tag("mem"), parse_mem),
            preceded(tag("mask"), parse_mask),
        )),
    )(input)
    .unwrap();
    out
}

fn parse_mask(input: &str) -> IResult<&str, Operation> {
    map(preceded(tag(" = "), take(36 as usize)), |s: &str| {
        Operation::ChnageMask(Mask::new(s.to_owned()))
    })(input)
}

fn parse_mem(input: &str) -> IResult<&str, Operation> {
    map(
        tuple::<_, (&str, &str), _, _>((delimited(tag("["), digit1, tag("] = ")), digit1)),
        |n| Operation::WriteMem((n.0.parse().unwrap(), n.1.parse().unwrap())),
    )(input)
}

#[aoc(day14, part1)]
fn part1(data: &[Operation]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = &Mask::new(String::new());

    for op in data {
        match op {
            Operation::ChnageMask(m) => mask = m,
            Operation::WriteMem((addr, amt)) => {
                mem.insert(*addr, mask.apply_v1(*amt));
            }
        }
    }
    mem.iter().map(|(_, i)| i).sum()
}

#[aoc(day14, part2)]
fn part2(data: &[Operation]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = &Mask::new(String::new());

    for op in data {
        match op {
            Operation::ChnageMask(m) => mask = m,
            Operation::WriteMem((addr, amt)) => {
                for a in mask.apply_v2(*addr) {
                    mem.insert(a, *amt);
                }
            }
        }
    }
    mem.iter().map(|(_, i)| i).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static DATA: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    static DATA2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn parsing_input() {
        assert_eq!(
            parse_input(DATA),
            vec![
                Operation::ChnageMask(Mask::new(
                    "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()
                )),
                Operation::WriteMem((8, 11)),
                Operation::WriteMem((7, 101)),
                Operation::WriteMem((8, 0))
            ]
        );
    }

    #[test]
    fn masking() {
        assert_eq!(
            mask_or_and("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            (64, 68719476733)
        )
    }

    #[test]
    fn running_part1() {
        let data = parse_input(DATA);
        assert_eq!(part1(&data), 165)
    }

    #[test]
    fn running_part2() {
        let data = parse_input(DATA2);
        assert_eq!(part2(&data), 208)
    }
}
