/// https://adventofcode.com/2020/day/5

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
    input.lines().map(|l| {
        let line = l.as_bytes();
        let row = line.iter().take(7).map(|b| match b {
            b'B' => "1",
            b'F' => "0",
            _ => unreachable!()
        }).collect::<String>();
        let row = u8::from_str_radix(&row, 2).unwrap();
        let col = line.iter().skip(7).map(|b| match b {
            b'R' => "1",
            b'L' => "0",
            _ => unreachable!()
        }).collect::<String>();
        let col = u8::from_str_radix(&col, 2).unwrap();
        Seat::new(row, col)
    }).collect()
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
