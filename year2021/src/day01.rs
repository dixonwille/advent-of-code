/// https://adventofcode.com/2021/day/1

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| l.parse().expect("positive integers only"))
        .collect()
}

#[aoc(day1, part1)]
fn part1(depths: &[usize]) -> Option<usize> {
    let sec = <&[usize]>::clone(&depths);
    let increased =
        depths.iter().zip(sec.iter().skip(1)).fold(
            0,
            |acc, c| {
                if c.0 < c.1 {
                    acc + 1
                } else {
                    acc
                }
            },
        );
    Some(increased)
}

#[aoc(day1, part2)]
fn part2(depths: &[usize]) -> Option<usize> {
    let sec = <&[usize]>::clone(&depths);
    let third = <&[usize]>::clone(&depths);
    let groups = depths
        .iter()
        .zip(sec.iter().skip(1).zip(third.iter().skip(2)))
        .map(|(f, (s, t))| f + s + t);
    let groups2 = groups.clone();
    let increased = groups
        .zip(groups2.skip(1))
        .fold(0, |acc, c| if c.0 < c.1 { acc + 1 } else { acc });
    Some(increased)
}

#[cfg(test)]
mod test {
    use super::*;

    static DEPTHS: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn parsing_input_test() {
        assert_eq!(
            parse_input(DEPTHS),
            vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
        )
    }

    #[test]
    fn part1_test() {
        let depths = parse_input(DEPTHS);
        assert_eq!(part1(&depths), Some(7))
    }

    #[test]
    fn part2_test() {
        let depths = parse_input(DEPTHS);
        assert_eq!(part2(&depths), Some(5))
    }
}
