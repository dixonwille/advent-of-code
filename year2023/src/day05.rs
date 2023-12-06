use std::{ops::Range, usize};

use crate::Solutions;
use lib_aoc::prelude::*;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, u64},
    combinator::{map, map_res, opt},
    error::ParseError,
    multi::{many1, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult, Parser,
};

// Parse a line
fn line<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    preceded(line_ending, inner)
}

fn get_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(tag("seeds: "), separated_list1(tag(" "), u64), line_ending)(input)
}

fn get_map_from_to(input: &str) -> IResult<&str, (String, String)> {
    line(terminated(
        map(
            separated_pair(alpha1, tag("-to-"), alpha1),
            |(from, to): (&str, &str)| (from.to_owned(), to.to_owned()),
        ),
        tag(" map:"),
    ))(input)
}

fn get_map_details(input: &str) -> IResult<&str, Vec<(Range<u64>, Range<u64>)>> {
    many1(map_res(line(separated_list1(tag(" "), u64)), |l| {
        if l.len() != 3 {
            Err("map does not contain dest, src, and len")
        } else {
            let dest = l.get(0).unwrap().to_owned();
            let src = l.get(1).unwrap().to_owned();
            let size = l.get(2).unwrap().to_owned();
            Ok((
                Range {
                    start: dest,
                    end: dest + size,
                },
                Range {
                    start: src,
                    end: src + size,
                },
            ))
        }
    }))(input)
}

fn get_map(input: &str) -> IResult<&str, Map> {
    map(
        terminated(pair(get_map_from_to, get_map_details), opt(line_ending)),
        |((from, to), dicts)| Map { from, to, dicts },
    )(input)
}

fn get_almanac(input: &str) -> IResult<&str, Almanac> {
    map(pair(get_seeds, many1(get_map)), |(seeds, maps)| Almanac {
        seeds,
        maps,
    })(input)
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn seed_location(&self, seed_num: u64) -> u64 {
        self.maps
            .iter()
            .fold(seed_num, |acc, map| map.get_dest(acc))
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    dicts: Vec<(Range<u64>, Range<u64>)>,
}

impl Map {
    fn get_dest(&self, num: u64) -> u64 {
        match self.dicts.iter().find(|(_, src)| src.contains(&num)) {
            Some((dest, src)) => num - src.start + dest.start,
            None => num,
        }
    }

    // Scenarios
    // orig   | new    | out                    | window
    // 15..20 | 12..16 | 15..16 16..20          | 12,15,16,20
    // 15..20 | 16..25 | 15..16 16..20          | 15,16,20,25
    // 15..20 | 12..25 | 15..20                 | 12,15,20,25
    // 15..20 | 16..18 | 15..16 16..18 18..20   | 15,16,18,20
    // 15..20 | 10..15 | 15..20                 | 10,15,15,20
    //
    // Folds using above scenarios for source, then converts those ranges
    // to the destination ranges
    fn fold(&self, orig: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let new: Vec<_> = self
            .dicts
            .iter()
            .map(|(_, src)| vec![src.start, src.end])
            .flatten()
            .collect();
        orig.iter()
            .map(|o| {
                let (start, end) = (o.start, o.end);
                let mut poi = vec![o.start, o.end];
                poi.extend(&new);
                poi.sort();
                poi.windows(2)
                    .filter_map(move |p| {
                        let a = p.get(0).unwrap().to_owned();
                        let b = p.get(1).unwrap().to_owned();
                        if a == b || a < start || b > end {
                            None
                        } else {
                            Some(Range {
                                start: self.get_dest(a),
                                // bounds are exclusive so I need to find the
                                // upper inner bound then increase by 1
                                end: self.get_dest(b - 1) + 1,
                            })
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }
}

impl Solution<DAY_05> for Solutions {
    type Input<'i> = Almanac;
    type Output = usize;

    fn parse(puzzle: &str) -> Almanac {
        let (_, p) = get_almanac(puzzle).unwrap();
        p
    }

    fn part_one(input: &Almanac) -> usize {
        input
            .seeds
            .iter()
            .map(|seed| input.seed_location(*seed))
            .min()
            .unwrap() as usize
    }

    // Looping through all the possible paths is not reasonable >30 minutes.
    // I need a way to break up the ranges (collapse seeds to check fewer options)
    //
    // Think time slicing where you have 2 groups of events and you need to identify when
    // either events from both groups have started and ended on a single timeline.
    // Then test out the each point of intrest (start of an event or the end of
    // an event). The times are the source start, events the ranges, and groups
    // the different maps.
    //
    // I'll need to fold a Vec<Range<u64>> with another Vec<Rangeg<u64>> using the
    // logic above. Then translate those final ranges into the destination ranges
    // to use in the next iteration. The last iteration will be the location ranges
    // and since start is before end, I only need to check those start iterations.
    //
    // My input has 2,547,615,236 brute force checks. I killed the process at 30 minutes.
    // Using this approach I only have 112 different points to check.
    fn part_two(input: &Almanac) -> usize {
        let init_seeds: Vec<_> = input
            .seeds
            .chunks(2)
            .map(|chunk| {
                let start = chunk.get(0).unwrap().to_owned();
                let size = chunk.get(1).unwrap().to_owned();
                Range {
                    start,
                    end: start + size,
                }
            })
            .collect();
        input
            .maps
            .iter()
            .fold(init_seeds, |acc, map| map.fold(acc))
            .iter()
            .map(|r| r.start)
            .min()
            .unwrap() as usize
    }
}

impl Test<DAY_05> for Solutions {
    fn expected(part: bool) -> usize {
        match part {
            PART_ONE => 35,
            PART_TWO => 46,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solutions;
    use lib_aoc::prelude::*;

    derive_tests!(Solutions, DAY_05);
}
