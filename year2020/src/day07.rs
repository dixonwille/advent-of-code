/// https://adventofcode.com/2020/day/7
use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char as c, digit1},
    combinator::{all_consuming, map, opt, value},
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone)]
struct Bag {
    description: String,
    can_contain: Option<Vec<Rule>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Rule {
    count: usize,
    description: String, // Don't want to box a Bag on the heap as I can just lookup a Bag by the description (ID)
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> HashMap<String, Bag> {
    let (_, rules) = parse_input_nom(input).unwrap();
    rules
}

fn parse_description(input: &str) -> IResult<&str, String> {
    map(
        separated_pair(alpha1, c(' '), alpha1),
        |desc: (&str, &str)| desc.0.to_owned() + " " + desc.1,
    )(input)
}

fn parse_rules(input: &str) -> IResult<&str, Option<Vec<Rule>>> {
    alt((
        value(None, tag("no other bags")),
        map(
            separated_list1(
                tag(", "),
                map(
                    terminated(
                        separated_pair(digit1, c(' '), parse_description),
                        tuple((tag(" bag"), opt(c('s')))),
                    ),
                    |(count, desc)| Rule {
                        count: count.parse().unwrap(),
                        description: desc,
                    },
                ),
            ),
            Some,
        ),
    ))(input)
}

fn parse_bag(input: &str) -> IResult<&str, Bag> {
    map(
        separated_pair(
            parse_description,
            delimited(
                c(' '),
                separated_pair(tag("bags"), c(' '), tag("contain")),
                c(' '),
            ),
            parse_rules,
        ),
        |(desc, rules)| Bag {
            description: desc,
            can_contain: rules,
        },
    )(input)
}

fn parse_input_nom(input: &str) -> IResult<&str, HashMap<String, Bag>> {
    all_consuming(fold_many1(
        terminated(parse_bag, tuple((c('.'), opt(c('\n'))))),
        HashMap::new(),
        |mut map, bag| {
            map.insert(bag.description.clone(), bag);
            map
        },
    ))(input)
}

#[aoc(day7, part1)]
fn part1(rules: &HashMap<String, Bag>) -> usize {
    let inverse = reverse(rules);
    let mut can_be_in = HashSet::new();
    can_be_contained_in("shiny gold", &inverse, &mut can_be_in);
    can_be_in.len()
}

fn can_be_contained_in<'a>(
    description: &str,
    map: &HashMap<&str, HashSet<&'a str>>,
    can_be_in: &mut HashSet<&'a str>,
) {
    if can_be_in.contains(description) {
        return;
    }
    match map.get(description) {
        None => {}
        Some(set) => {
            for s in set {
                can_be_contained_in(s, map, can_be_in);
                can_be_in.insert(s);
            }
        }
    };
}

fn reverse<'a>(original: &'a HashMap<String, Bag>) -> HashMap<&'a str, HashSet<&'a str>> {
    original.iter().fold(HashMap::new(), |mut map, (_, bag)| {
        if let Some(can_contain) = &bag.can_contain {
            for rule in can_contain {
                match map.get_mut(rule.description.as_str()) {
                    None => {
                        let mut set = HashSet::new();
                        set.insert(bag.description.as_str());
                        map.insert(&rule.description, set);
                    }
                    Some(cur) => {
                        (*cur).insert(bag.description.as_str());
                    }
                };
            }
        }
        map
    })
}

#[aoc(day7, part2)]
fn part2(rules: &HashMap<String, Bag>) -> usize {
    bags_inside("shiny gold", rules)
}

fn bags_inside(description: &str, map: &HashMap<String, Bag>) -> usize {
    let out = match map.get(description) {
        None => 0,
        Some(bag) => match &bag.can_contain {
            None => 0,
            Some(rules) => {
                rules.iter().fold(0, |mut count, rule| {
                    count += rule.count; // count the bags themself
                    let inside = bags_inside(rule.description.as_str(), map); // get how many bags are inside this one
                    count += rule.count * inside;
                    count
                })
            }
        },
    };
    out
}

#[cfg(test)]
mod test {

    use super::*;

    static RULES: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    static RULES2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn parsing_input() {
        let mut map = HashMap::new();
        map.insert(
            "light red".to_owned(),
            Bag {
                description: "light red".to_owned(),
                can_contain: Some(vec![
                    Rule {
                        count: 1,
                        description: "bright white".to_owned(),
                    },
                    Rule {
                        count: 2,
                        description: "muted yellow".to_owned(),
                    },
                ]),
            },
        );
        map.insert(
            "dark orange".to_owned(),
            Bag {
                description: "dark orange".to_owned(),
                can_contain: Some(vec![
                    Rule {
                        count: 3,
                        description: "bright white".to_owned(),
                    },
                    Rule {
                        count: 4,
                        description: "muted yellow".to_owned(),
                    },
                ]),
            },
        );
        map.insert(
            "bright white".to_owned(),
            Bag {
                description: "bright white".to_owned(),
                can_contain: Some(vec![Rule {
                    count: 1,
                    description: "shiny gold".to_owned(),
                }]),
            },
        );
        map.insert(
            "muted yellow".to_owned(),
            Bag {
                description: "muted yellow".to_owned(),
                can_contain: Some(vec![
                    Rule {
                        count: 2,
                        description: "shiny gold".to_owned(),
                    },
                    Rule {
                        count: 9,
                        description: "faded blue".to_owned(),
                    },
                ]),
            },
        );
        map.insert(
            "shiny gold".to_owned(),
            Bag {
                description: "shiny gold".to_owned(),
                can_contain: Some(vec![
                    Rule {
                        count: 1,
                        description: "dark olive".to_owned(),
                    },
                    Rule {
                        count: 2,
                        description: "vibrant plum".to_owned(),
                    },
                ]),
            },
        );
        map.insert(
            "dark olive".to_owned(),
            Bag {
                description: "dark olive".to_owned(),
                can_contain: Some(vec![
                    Rule {
                        count: 3,
                        description: "faded blue".to_owned(),
                    },
                    Rule {
                        count: 4,
                        description: "dotted black".to_owned(),
                    },
                ]),
            },
        );
        map.insert(
            "vibrant plum".to_owned(),
            Bag {
                description: "vibrant plum".to_owned(),
                can_contain: Some(vec![
                    Rule {
                        count: 5,
                        description: "faded blue".to_owned(),
                    },
                    Rule {
                        count: 6,
                        description: "dotted black".to_owned(),
                    },
                ]),
            },
        );
        map.insert(
            "faded blue".to_owned(),
            Bag {
                description: "faded blue".to_owned(),
                can_contain: None,
            },
        );
        map.insert(
            "dotted black".to_owned(),
            Bag {
                description: "dotted black".to_owned(),
                can_contain: None,
            },
        );
        assert_eq!(parse_input(&RULES), map);
    }

    #[test]
    fn running_part1() {
        let rules = parse_input(RULES);
        assert_eq!(part1(&rules), 4)
    }

    #[test]
    fn running_part2() {
        let rules = parse_input(RULES);
        assert_eq!(part2(&rules), 32);

        let rules = parse_input(RULES2);
        assert_eq!(part2(&rules), 126);
    }
}
