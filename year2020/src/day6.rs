/// https://adventofcode.com/2020/day/6
use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    character::complete::{alpha1, newline},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
struct Declaration {
    yes: HashSet<char>,
}

#[derive(Debug, Eq, PartialEq)]
struct DeclarationGroup {
    group: Vec<Declaration>,
}

impl DeclarationGroup {
    fn condense(&self) -> HashSet<char> {
        self.group.iter().fold(HashSet::new(), |mut set, g| {
            for g in &g.yes {
                set.insert(g.to_owned());
            }
            set
        })
    }

    fn condense_all(&self) -> HashSet<char> {
        let mut set = HashSet::new();
        let size = self.group.len();
        for c in &self.group[0].yes {
            let mut count = 0;
            for i in 1..size {
                if self.group[i].yes.contains(c) {
                    count += 1;
                }
            }
            if count == size - 1 {
                set.insert(c.to_owned());
            }
        }
        set
    }
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<DeclarationGroup> {
    let (_, declarations) = parse_input_nom(input).unwrap();
    declarations
}

fn parse_declaration(input: &str) -> IResult<&str, Declaration> {
    let (input, chars) = alpha1(input)?;
    let mut set = HashSet::new();
    for c in chars.chars() {
        set.insert(c);
    }
    Ok((input, Declaration { yes: set }))
}

fn parse_declaration_group(input: &str) -> IResult<&str, DeclarationGroup> {
    let (input, decs) = separated_list1(newline, parse_declaration)(input)?;
    Ok((input, DeclarationGroup { group: decs }))
}

fn parse_input_nom(input: &str) -> IResult<&str, Vec<DeclarationGroup>> {
    all_consuming(separated_list1(
        tuple((newline, newline)),
        parse_declaration_group,
    ))(input)
}

#[aoc(day6, part1)]
fn part1(declarations: &[DeclarationGroup]) -> usize {
    declarations.iter().map(|g| g.condense().len()).sum()
}

#[aoc(day6, part2)]
fn part2(declarations: &[DeclarationGroup]) -> usize {
    declarations.iter().map(|g| g.condense_all().len()).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static DECLARATIONS: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn parsing_input() {
        let mut d1g1 = HashSet::new();
        d1g1.insert('a');
        d1g1.insert('b');
        d1g1.insert('c');

        let mut d1g2 = HashSet::new();
        d1g2.insert('a');
        let mut d2g2 = HashSet::new();
        d2g2.insert('b');
        let mut d3g2 = HashSet::new();
        d3g2.insert('c');

        let mut d1g3 = HashSet::new();
        d1g3.insert('a');
        d1g3.insert('b');
        let mut d2g3 = HashSet::new();
        d2g3.insert('a');
        d2g3.insert('c');

        let mut d1g4 = HashSet::new();
        d1g4.insert('a');
        let mut d2g4 = HashSet::new();
        d2g4.insert('a');
        let mut d3g4 = HashSet::new();
        d3g4.insert('a');
        let mut d4g4 = HashSet::new();
        d4g4.insert('a');

        let mut d1g5 = HashSet::new();
        d1g5.insert('b');

        assert_eq!(
            parse_input(DECLARATIONS),
            vec![
                DeclarationGroup {
                    group: vec![Declaration { yes: d1g1 }]
                },
                DeclarationGroup {
                    group: vec![
                        Declaration { yes: d1g2 },
                        Declaration { yes: d2g2 },
                        Declaration { yes: d3g2 }
                    ]
                },
                DeclarationGroup {
                    group: vec![Declaration { yes: d1g3 }, Declaration { yes: d2g3 }]
                },
                DeclarationGroup {
                    group: vec![
                        Declaration { yes: d1g4 },
                        Declaration { yes: d2g4 },
                        Declaration { yes: d3g4 },
                        Declaration { yes: d4g4 }
                    ]
                },
                DeclarationGroup {
                    group: vec![Declaration { yes: d1g5 }]
                }
            ]
        )
    }

    #[test]
    fn running_part1() {
        let declarations = parse_input(DECLARATIONS);
        assert_eq!(part1(&declarations), 11);
    }

    #[test]
    fn running_part2() {
        let declarations = parse_input(DECLARATIONS);
        assert_eq!(part2(&declarations), 6);
    }
}
