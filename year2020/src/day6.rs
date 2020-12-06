/// https://adventofcode.com/2020/day/6
use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Eq, PartialEq)]
struct Declaration {
    yes: HashSet<char>,
}

#[derive(Debug, Eq, PartialEq)]
struct DeclarationGroup {
    group: Vec<Declaration>,
}

impl DeclarationGroup {
    fn condense_yes (&self) -> HashSet<char> {
        self.group.iter().fold(HashSet::new(), |mut set, g| {
            for g in &g.yes {
                set.insert(g.to_owned());
            }
            set
        })
    }
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<DeclarationGroup> {
    Vec::new()
}

#[aoc(day6, part1)]
fn part1(declarations: &[DeclarationGroup]) -> usize {
    declarations.iter().map(|g| g.condense_yes().len()).sum()
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
                    group: vec![
                        Declaration { yes: d1g3 },
                        Declaration { yes: d2g3 }
                    ]
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
}
