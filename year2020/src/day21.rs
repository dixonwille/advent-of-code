/// https://adventofcode.com/2020/day/21
use pest::Parser;
use std::collections::{HashMap, HashSet};

#[derive(Parser)]
#[grammar = "pest/day21.pest"]
struct InputParser;

#[aoc_generator(day21)]
fn parse_input(input: &str) -> Vec<(HashSet<String>, HashSet<String>)> {
    InputParser::parse(Rule::file, input)
        .expect("unable to parse input")
        .filter(|r| r.as_rule() == Rule::listing)
        .map(|l| {
            let mut inner = l.into_inner();
            let ingrediants = inner
                .next()
                .unwrap()
                .into_inner()
                .map(|i| i.as_str().to_string())
                .collect::<HashSet<_>>();
            let allergens = inner
                .next()
                .unwrap()
                .into_inner()
                .map(|a| a.as_str().to_string())
                .collect::<HashSet<_>>();
            (ingrediants, allergens)
        })
        .collect()
}

fn clean_input(
    listing: &[(HashSet<String>, HashSet<String>)],
) -> (Vec<String>, HashMap<String, HashSet<String>>) {
    let mut ingrediants = Vec::new();
    let mut allergens = HashMap::new();
    for (ing, all) in listing {
        for i in ing {
            ingrediants.push(i.clone())
        }
        for a in all {
            let e = allergens.entry(a.clone()).or_insert_with(|| ing.clone());
            *e = e.intersection(ing).cloned().collect();
        }
    }
    (ingrediants, allergens)
}

#[aoc(day21, part1)]
fn part1(listing: &[(HashSet<String>, HashSet<String>)]) -> usize {
    let (ingrediants, allergens) = clean_input(listing);
    let mut base = HashSet::new();
    for (_, a) in allergens {
        base = base.union(&a).cloned().collect();
    }
    ingrediants
        .into_iter()
        .filter(|i| !base.contains(i))
        .count()
}

fn clean_allergens(allergens: &mut HashMap<String, HashSet<String>>) {
    let mut removed = HashSet::new();
    loop {
        let mut to_remove = HashSet::new();
        for (_, ings) in allergens.iter() {
            if ings.len() == 1 {
                let ing = ings.iter().next().unwrap();
                if !removed.contains(ing) {
                    to_remove.insert(ing.to_owned());
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for r in to_remove.iter() {
            for (_, ings) in allergens.iter_mut() {
                if ings.len() == 1 {
                    continue;
                }
                ings.remove(r);
            }
            removed.insert(r.to_owned());
        }
    }
}

#[allow(dead_code)]
fn part2(listing: &[(HashSet<String>, HashSet<String>)]) -> String {
    let (_, mut allergens) = clean_input(listing);
    clean_allergens(&mut allergens);
    let mut allergens = allergens
        .into_iter()
        .map(|(k, v)| (k, v))
        .collect::<Vec<_>>();
    allergens.sort_by(|(a, _), (b, _)| a.cmp(b));
    allergens
        .into_iter()
        .map(|(_, v)| v.into_iter().next().unwrap())
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::Read};

    use super::*;
    static LISTING: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn parsing_input() {
        let map = parse_input(LISTING);
        println!("{:#?}", map);
    }

    #[test]
    fn running_part1() {
        let listing = parse_input(LISTING);
        assert_eq!(part1(&listing), 5);
    }

    #[test]
    fn running_part2() {
        let listing = parse_input(LISTING);
        assert_eq!(part2(&listing), "mxmxvkd,sqjhc,fvjkl");
    }

    #[test]
    #[ignore = "this is actually running the code not the test"]
    fn running() {
        let mut input = String::new();
        File::open("input/2020/day21.txt")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        let listing = parse_input(input.as_str());
        println!("{}", part2(&listing));
    }
}
