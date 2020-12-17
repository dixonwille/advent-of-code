use std::{collections::HashMap, ops::Range};

/// https://adventofcode.com/2020/day/15
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::{tag, take_till},
    character::complete::digit1,
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

type Validations = HashMap<String, [Range<usize>; 2]>;
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

fn parse_validations(input: &str) -> IResult<&str, HashMap<String, [Range<usize>; 2]>> {
    let (input, out) = separated_list1(
        tag("\n"),
        map(
            separated_pair(
                take_till(|c| c == ':'),
                tag(": "),
                separated_pair(parse_range, tag(" or "), parse_range),
            ),
            |(s, (st, nd))| (s.to_owned(), [st, nd]),
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

fn consolidate_ranges(ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
    let mut ranges = ranges;
    loop {
        let mut consolidated = Vec::new();
        if ranges.is_empty() {
            return consolidated;
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
            break consolidated;
        }
        ranges = consolidated;
    }
}

fn filter_tickets(validations: &Validations, nearby: &[Ticket]) -> Vec<Ticket> {
    let mut ranges = Vec::new();
    validations.iter().for_each(|(_, validations)| validations.iter().for_each(|v| ranges.push(v.clone())));
    let valid_ranges = consolidate_ranges(ranges);
    let mut valid = Vec::new();
    'ticket: for ticket in nearby{
        for field in ticket {
            for v in &valid_ranges {
                if !v.contains(field) {
                    continue 'ticket;
                }
            }
        }
        valid.push(ticket.clone());
    }
    valid
}

#[aoc(day16, part1)]
fn part1((validations, _, nearby): &(Validations, Ticket, Vec<Ticket>)) -> usize {
    let mut ranges = Vec::new();
    validations.iter().for_each(|(_, validations)| validations.iter().for_each(|v| ranges.push(v.clone())));
    let valid = consolidate_ranges(ranges);
    let mut invalid = Vec::new();
    for ticket in nearby {
        'field: for field in ticket {
            for v in &valid {
                if v.contains(field) {
                    continue 'field;
                }
            }
            invalid.push(*field)
        }
    }
    invalid.iter().sum()
}

#[aoc(day16, part2)]
fn part2((validations, _own, nearby): &(Validations, Ticket, Vec<Ticket>)) -> usize {
    let valid = filter_tickets(validations, nearby);
    // turn flip the vectors so that each column is in it's own Vec
    // so VecSizeM of VecSizeN becomes VecSizeN of VecSizeN
    // go through each column set and find the valid fields for that column
    // should be able to use process of elimination from here
    0
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
            [Range { start: 1, end: 3 }, Range { start: 5, end: 7 }],
        );
        map.insert(
            "row".to_string(),
            [Range { start: 6, end: 11 }, Range { start: 33, end: 44 }],
        );
        map.insert(
            "departure seat".to_string(),
            [Range { start: 13, end: 40 }, Range { start: 45, end: 50 }],
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
