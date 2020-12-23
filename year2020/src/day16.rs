/// https://adventofcode.com/2020/day/16
use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::digit1,
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

type Validations = HashMap<String, (Range<usize>, Range<usize>)>;
type Ticket = Vec<usize>;

#[aoc_generator(day16)]
fn parse_input(input: &str) -> (Validations, Ticket, Vec<Ticket>) {
    let (_, out) = parse_input_nom(input).unwrap();
    out
}

fn parse_input_nom(input: &str) -> IResult<&str, (Validations, Ticket, Vec<Ticket>)> {
    let (input, validations) = parse_validations(input)?;
    let (input, _) = tag("\n\nyour ticket:\n")(input)?;
    let (input, own_ticket) = parse_ticket(input)?;
    let (input, _) = tag("\n\nnearby tickets:\n")(input)?;
    let (input, nearby_tickets) = separated_list1(tag("\n"), parse_ticket)(input)?;
    Ok((input, (validations, own_ticket, nearby_tickets)))
}

fn parse_range(input: &str) -> IResult<&str, Range<usize>> {
    map::<_, (&str, &str), _, _, _, _>(separated_pair(digit1, tag("-"), digit1), |(a, b)| Range {
        start: a.parse().unwrap(),
        end: b.parse::<usize>().unwrap() + 1,
    })(input)
}

fn parse_validations(input: &str) -> IResult<&str, Validations> {
    let (input, out) = separated_list1(
        tag("\n"),
        map(
            separated_pair(
                take_till(|c| c == ':'),
                tag(": "),
                separated_pair(parse_range, tag(" or "), parse_range),
            ),
            |(s, (st, nd))| (s.to_owned(), (st, nd)),
        ),
    )(input)?;
    Ok((input, out.into_iter().collect()))
}

fn parse_ticket(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, out) = separated_list1(
        tag(","),
        map::<_, &str, _, _, _, _>(digit1, |d| d.parse().unwrap()),
    )(input)?;
    Ok((input, out))
}

fn consolidate_ranges(ranges: &mut Vec<Range<usize>>) {
    loop {
        let mut consolidated = Vec::new();
        if ranges.is_empty() {
            return;
        }
        consolidated.push(ranges[0].clone());

        'range: for range in ranges[1..].iter() {
            for cons in consolidated.iter_mut() {
                if cons.contains(&range.start) && !cons.contains(&range.end) {
                    cons.end = range.end;
                    continue 'range;
                }
                if !cons.contains(&range.start) && cons.contains(&range.end) {
                    cons.start = range.start;
                    continue 'range;
                }
            }
            consolidated.push(range.clone());
        }
        if consolidated.len() == ranges.len() {
            break;
        }
        *ranges = consolidated;
    }
}

fn filter_tickets<'a>(validations: &Validations, nearby: &'a [Ticket]) -> Vec<&'a Ticket> {
    let mut ranges = validations
        .iter()
        .flat_map(|(_, rules)| vec![rules.0.clone(), rules.1.clone()])
        .collect();
    consolidate_ranges(&mut ranges);
    nearby
        .iter()
        .filter(|fields| {
            fields
                .iter()
                .all(|field| ranges.iter().any(|r| r.contains(field)))
        })
        .collect()
}

fn flip_vec(vectors: Vec<&Ticket>) -> Vec<Vec<usize>> {
    let num_tickets = vectors.len();
    let num_fields = vectors[0].len();
    let mut flipped = Vec::with_capacity(num_fields);
    for _ in 0..num_fields {
        flipped.push(Vec::with_capacity(num_tickets));
    }
    for vector in vectors {
        for (i, field) in vector.iter().enumerate() {
            flipped[i].push(*field);
        }
    }
    flipped
}

fn clean_findings(findings: &mut HashMap<String, Vec<usize>>) {
    let mut eliminated = HashSet::new();

    loop {
        let mut eliminate = HashSet::new();
        // find what I can eliminate
        for (_, find) in findings.iter() {
            if find.len() == 1 && !eliminated.contains(&find[0]) {
                eliminate.insert(find[0]);
            }
        }

        if eliminate.is_empty() {
            break;
        }

        // remove eliminatable elements from map
        for elim in eliminate.iter() {
            for (_, find) in findings.iter_mut() {
                if find.len() == 1 {
                    continue;
                }
                *find = find.clone().into_iter().filter(|f| f != elim).collect();
            }
            eliminated.insert(*elim);
        }
    }
}

#[aoc(day16, part1)]
fn part1((validations, _, nearby): &(Validations, Ticket, Vec<Ticket>)) -> usize {
    let mut ranges = validations
        .iter()
        .flat_map(|(_, rules)| vec![rules.0.clone(), rules.1.clone()])
        .collect();
    consolidate_ranges(&mut ranges);
    nearby
        .iter()
        .map::<Vec<_>, _>(|fields| {
            fields
                .iter()
                .filter(|field| !ranges.iter().any(|r| r.contains(field)))
                .collect()
        })
        .flatten()
        .sum()
}

#[aoc(day16, part2)]
fn part2((validations, own, nearby): &(Validations, Ticket, Vec<Ticket>)) -> usize {
    let fields = flip_vec(filter_tickets(validations, nearby));
    // figure out which fields are valid with which validation
    let mut defs: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, field) in fields.into_iter().enumerate() {
        for (key, rules) in validations {
            if field
                .iter()
                .all(|f| rules.0.contains(&f) || rules.1.contains(&f))
            {
                defs.entry(key.to_string()).or_insert_with(Vec::new).push(i);
            }
        }
    }
    clean_findings(&mut defs);
    defs.iter()
        .filter(|(key, _)| key.starts_with("departure"))
        .map(|(_, find)| own[find[0]])
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    static NOTES: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
departure seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn parsing_input() {
        let mut map = HashMap::new();
        map.insert(
            "class".to_string(),
            (Range { start: 1, end: 4 }, Range { start: 5, end: 8 }),
        );
        map.insert(
            "row".to_string(),
            (Range { start: 6, end: 12 }, Range { start: 33, end: 45 }),
        );
        map.insert(
            "departure seat".to_string(),
            (Range { start: 13, end: 41 }, Range { start: 45, end: 51 }),
        );
        assert_eq!(
            parse_input(NOTES),
            (
                map,
                vec![7, 1, 14],
                vec![
                    vec![7, 3, 47],
                    vec![40, 4, 50],
                    vec![55, 2, 20],
                    vec![38, 6, 12]
                ]
            )
        );
    }

    #[test]
    fn running_part1() {
        let notes = parse_input(NOTES);
        assert_eq!(part1(&notes), 71);
    }

    #[test]
    fn running_part2() {
        let notes = parse_input(NOTES);
        assert_eq!(part2(&notes), 14);
    }
}
