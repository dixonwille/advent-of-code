/// https://adventofcode.com/2020/day/19
// Things I would do differently
// I would create a CYK package that included a way to specify a CFG that then cleaned it properly to CNF
// The way it is done here, we parse and clean in a single map where as with the CYK package I would parse then add rules
use pest::{iterators::Pair, Parser};
use std::collections::{HashMap, HashSet};

#[derive(Parser)]
#[grammar = "day19.pest"]
struct InputParser;

#[derive(Debug, PartialEq, Eq, Clone)]
enum RuleDef<T, V> {
    Terminal(T),
    Variable(V),
    And(Vec<Box<RuleDef<T, V>>>),
    Or(Vec<Box<RuleDef<T, V>>>),
}

#[aoc_generator(day19)]
fn parse_input(input: &str) -> (HashMap<usize, RuleDef<String, usize>>, Vec<String>) {
    let inputs = InputParser::parse(Rule::file, input).expect("could not parse input");
    let mut rules = HashMap::new();
    let mut messages = Vec::new();
    for input in inputs {
        match input.as_rule() {
            Rule::rule => {
                let mut inner_rules = input.into_inner();
                let rule_num = inner_rules
                    .next()
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();
                let definition = inner_rules.next().unwrap();
                let r = match definition.as_rule() {
                    Rule::string => RuleDef::Terminal(
                        definition.into_inner().next().unwrap().as_str().to_string(),
                    ),
                    Rule::groupings => parse_groups(definition.into_inner().collect()),
                    _ => unreachable!(),
                };
                rules.insert(rule_num, r);
            }
            Rule::message => messages.push(input.as_str().to_string()),
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }
    (rules, messages)
}

// Really dumb version of UNIT and BIN cleaning only
// https://en.wikipedia.org/wiki/Chomsky_normal_form#UNIT:_Eliminate_unit_rules
// https://en.wikipedia.org/wiki/Chomsky_normal_form#BIN:_Eliminate_right-hand_sides_with_more_than_2_nonterminals
fn clean_rules(rules: &mut HashMap<usize, RuleDef<String, usize>>) {
    let mut new_key: usize = 500;
    let mut overwrite = Vec::new();
    for (k, rule) in rules.iter() {
        match rule {
            RuleDef::Terminal(_) => {}
            RuleDef::And(and) => {
                if and.len() > 2 {
                    overwrite.push((
                        *k,
                        RuleDef::And(vec![and[0].clone(), Box::new(RuleDef::Variable(new_key))]),
                    ));
                    new_key += 1;
                    for a in and.iter().take(and.len() - 2).skip(1) {
                        overwrite.push((
                            new_key - 1,
                            RuleDef::And(vec![a.clone(), Box::new(RuleDef::Variable(new_key))]),
                        ));
                        new_key += 1;
                    }
                    overwrite.push((
                        new_key - 1,
                        RuleDef::And(vec![and[and.len() - 2].clone(), and[and.len() - 1].clone()]),
                    ));
                }
            }
            RuleDef::Variable(i) => overwrite.push((*k, rules.get(&i).unwrap().clone())),
            RuleDef::Or(inner_rules) => {
                // Yes this overwrites every Or with itself if it doesn't need too...
                let mut ow = Vec::new();
                for inner in inner_rules.iter() {
                    match &**inner {
                        RuleDef::Terminal(_) => ow.push((*inner).clone()),
                        RuleDef::Or(or) => {
                            for o in or {
                                ow.push((*o).clone());
                            }
                        }
                        RuleDef::And(and) => {
                            if and.len() > 2 {
                                ow.push(Box::new(RuleDef::And(vec![
                                    and[0].clone(),
                                    Box::new(RuleDef::Variable(new_key)),
                                ])));
                                new_key += 1;
                                for a in and.iter().take(and.len() - 2).skip(1) {
                                    overwrite.push((
                                        new_key - 1,
                                        RuleDef::And(vec![
                                            a.clone(),
                                            Box::new(RuleDef::Variable(new_key)),
                                        ]),
                                    ));
                                    new_key += 1;
                                }
                                overwrite.push((
                                    new_key - 1,
                                    RuleDef::And(vec![
                                        and[and.len() - 2].clone(),
                                        and[and.len() - 1].clone(),
                                    ]),
                                ));
                            } else {
                                ow.push((*inner).clone());
                            }
                        }
                        RuleDef::Variable(v) => ow.push(Box::new(rules.get(&v).unwrap().clone())),
                    }
                }
                overwrite.push((*k, RuleDef::Or(ow)));
            }
        }
    }
    for (k, ow) in overwrite.into_iter() {
        rules.insert(k, ow);
    }
}

fn parse_groups(groups: Vec<Pair<Rule>>) -> RuleDef<String, usize> {
    if groups.len() == 1 {
        let variables = groups
            .into_iter()
            .next()
            .unwrap()
            .into_inner()
            .collect::<Vec<_>>();
        parse_variables(variables)
    } else {
        RuleDef::Or(
            groups
                .into_iter()
                .map(|g| Box::new(parse_variables(g.into_inner().collect())))
                .collect(),
        )
    }
}

fn parse_variables(variables: Vec<Pair<Rule>>) -> RuleDef<String, usize> {
    if variables.len() == 1 {
        let variable = variables.into_iter().next().unwrap();
        RuleDef::Variable(variable.as_str().parse::<usize>().unwrap())
    } else {
        RuleDef::And(
            variables
                .into_iter()
                .map(|v| Box::new(RuleDef::Variable(v.as_str().parse::<usize>().unwrap())))
                .collect(),
        )
    }
}

type RuleMap = HashMap<Vec<usize>, Vec<usize>>;
type TermMap = HashMap<String, Vec<usize>>;

// Doing this for faster lookups over a set of messages
// returns the mapping of rules and terminals
// this should be recursive or cleaned up so copy code in loops isn't needed
fn cnf_rules(rules: &HashMap<usize, RuleDef<String, usize>>) -> (RuleMap, TermMap) {
    let mut rule_map = HashMap::new();
    let mut term_map = HashMap::new();
    for rule in rules {
        cnf_rules_recurs(rule, &mut rule_map, &mut term_map);
    }
    (rule_map, term_map)
}

fn cnf_rules_recurs(
    (rule_id, rule): (&usize, &RuleDef<String, usize>),
    rule_map: &mut RuleMap,
    term_map: &mut TermMap,
) {
    match rule {
        RuleDef::Terminal(t) => {
            let e = term_map.entry(t.to_owned()).or_insert_with(Vec::new);
            e.push(*rule_id);
        }
        RuleDef::And(and) => {
            let mut r = Vec::new();
            for a in and {
                match &**a {
                    RuleDef::And(_) | RuleDef::Or(_) | RuleDef::Terminal(_) => unreachable!(),
                    RuleDef::Variable(v) => r.push(*v),
                }
            }
            let e = rule_map.entry(r).or_insert_with(Vec::new);
            e.push(*rule_id);
        }
        RuleDef::Or(or) => {
            for o in or {
                cnf_rules_recurs((rule_id, &**o), rule_map, term_map);
            }
        }
        RuleDef::Variable(_) => unreachable!(),
    }
}

fn vec_multiple<'grammer>(a: &'grammer [usize], b: &'grammer [usize]) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    for ai in a {
        for bi in b {
            out.push(vec![*ai, *bi])
        }
    }
    out
}

fn can_cyk_parse((rule_map, term_map): &(RuleMap, TermMap), input: &str) -> bool {
    let mut matrix = Vec::new();
    // start off by pushing the terminals of the string into the matrix
    matrix.push(
        input
            .chars()
            .map(|i| term_map.get(&i.to_string()).unwrap().clone())
            .collect::<Vec<Vec<_>>>(),
    );

    // size of sub string
    for size in 2..=input.len() {
        let mut row: Vec<Vec<usize>> = Vec::new();
        // pointer to start of the str
        for i in 0..=input.len() - size {
            // j is a pointer to the end of the str
            let j = i + size;

            let mut targets = Vec::new();
            // k is the split of the string
            for k in 1..size {
                let left = matrix[k - 1].get(i).unwrap();
                let right = matrix[j - (i + k) - 1].get(i + k).unwrap();
                targets.push(vec_multiple(left, right));
            }

            let mut entry = HashSet::new();
            for target in targets.into_iter().flatten() {
                if let Some(rule_ids) = rule_map.get(&target) {
                    for rule_id in rule_ids {
                        entry.insert(*rule_id);
                    }
                }
            }
            row.push(entry.into_iter().collect());
        }
        matrix.push(row);
    }
    matrix[matrix.len() - 1][0].contains(&0)
}

#[aoc(day19, part1)]
fn part1((rules, messages): &(HashMap<usize, RuleDef<String, usize>>, Vec<String>)) -> usize {
    let mut rules = rules.clone();
    clean_rules(&mut rules);
    let cnf = cnf_rules(&rules);
    messages.iter().filter(|m| can_cyk_parse(&cnf, m)).count()
}

#[aoc(day19, part2)]
fn part2((rules, messages): &(HashMap<usize, RuleDef<String, usize>>, Vec<String>)) -> usize {
    let mut rules = rules.clone();
    rules.insert(
        8,
        RuleDef::Or(vec![
            Box::new(RuleDef::Variable(42)),
            Box::new(RuleDef::And(vec![
                Box::new(RuleDef::Variable(42)),
                Box::new(RuleDef::Variable(8)),
            ])),
        ]),
    );
    rules.insert(
        11,
        RuleDef::Or(vec![
            Box::new(RuleDef::And(vec![
                Box::new(RuleDef::Variable(42)),
                Box::new(RuleDef::Variable(31)),
            ])),
            Box::new(RuleDef::And(vec![
                Box::new(RuleDef::Variable(42)),
                Box::new(RuleDef::Variable(11)),
                Box::new(RuleDef::Variable(31)),
            ])),
        ]),
    );
    clean_rules(&mut rules);
    let cnf = cnf_rules(&rules);
    messages.iter().filter(|m| can_cyk_parse(&cnf, m)).count()
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r###"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"###;

    // #[test]
    // fn parsing_input() {
    //     let mut rules: HashMap<usize, _> = HashMap::new();
    //     rules.insert(0, RuleDef::Variable(vec![vec![4, 1, 5]]));
    //     rules.insert(1, RuleDef::Variable(vec![vec![2, 3], vec![3, 2]]));
    //     rules.insert(2, RuleDef::Variable(vec![vec![4, 4], vec![5, 5]]));
    //     rules.insert(3, RuleDef::Variable(vec![vec![4, 5], vec![5, 4]]));
    //     rules.insert(4, RuleDef::Terminal("a".to_string()));
    //     rules.insert(5, RuleDef::Terminal("b".to_string()));
    //     assert_eq!(
    //         parse_input(INPUT),
    //         (
    //             rules,
    //             vec![
    //                 "ababbb".to_string(),
    //                 "bababa".to_string(),
    //                 "abbbab".to_string(),
    //                 "aaabbb".to_string(),
    //                 "aaaabbb".to_string()
    //             ]
    //         )
    //     );
    // }

    #[test]
    fn parsing_input() {
        println!("{:#?}", parse_input(INPUT));
        println!("{:#?}", parse_input(INPUT2));
    }

    static INPUT2: &str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

    #[test]
    fn running_part1() {
        let input = parse_input(INPUT2);
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn running_part2() {
        let input = parse_input(INPUT2);
        assert_eq!(part2(&input), 12);
    }
}
