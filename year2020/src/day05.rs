/// https://adventofcode.com/2020/day/5
use nom::{
    branch::alt,
    character::complete::{char as c, newline},
    combinator::{all_consuming, map, value},
    multi::{count, separated_list1},
    sequence::tuple,
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
struct Seat {
    row: u8,
    col: u8,
}

impl Seat {
    fn new(row: u8, col: u8) -> Self {
        Seat { row, col }
    }

    fn id(&self) -> usize {
        self.row as usize * 8 + self.col as usize
    }
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<Seat> {
    let (_, seats) = parse_input_nom(input).unwrap();
    seats
}

fn parse_row(input: &str) -> IResult<&str, u8> {
    let (input, val) = count(alt((value('1', c('B')), value('0', c('F')))), 7)(input)?;
    let col_string: String = val.into_iter().collect();
    Ok((input, u8::from_str_radix(&col_string, 2).unwrap()))
}

fn parse_col(input: &str) -> IResult<&str, u8> {
    let (input, val) = count(alt((value('1', c('R')), value('0', c('L')))), 3)(input)?;
    let row_string: String = val.into_iter().collect();
    Ok((input, u8::from_str_radix(&row_string, 2).unwrap()))
}

fn parse_input_nom(input: &str) -> IResult<&str, Vec<Seat>> {
    all_consuming(separated_list1(
        newline,
        map(tuple((parse_row, parse_col)), |info| {
            Seat::new(info.0, info.1)
        }),
    ))(input)
}

#[aoc(day5, part1)]
fn part1(seats: &[Seat]) -> Option<usize> {
    seats.iter().map(|seat| seat.id()).max()
}

#[aoc(day5, part2)]
fn part2(seats: &[Seat]) -> usize {
    let seats: Vec<usize> = seats.iter().map(|seat| seat.id()).collect();
    let min = seats.iter().min().unwrap().to_owned();
    let max = seats.iter().max().unwrap().to_owned();
    let want: usize = (min..max + 1).sum();
    let have: usize = seats.iter().sum();
    want - have
}

#[cfg(test)]
mod test {
    use super::*;

    static SEATS: &str = "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

    #[test]
    fn parsing_input() {
        assert_eq!(
            parse_input(SEATS),
            vec![Seat::new(70, 7), Seat::new(14, 7), Seat::new(102, 4)]
        );
    }
}
