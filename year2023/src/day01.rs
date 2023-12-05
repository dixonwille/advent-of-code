use crate::Solutions;
use lib_aoc::prelude::*;

impl Solution<DAY_01> for Solutions {
    type Input<'i> = Vec<&'i str>;
    type Output = usize;

    fn parse(puzzle: &str) -> Vec<&'_ str> {
        puzzle.lines().collect()
    }

    fn part_one(input: &Vec<&'_ str>) -> usize {
        input
            .iter()
            .map(|line| {
                let digits: Vec<_> = line.chars().filter(|c| c.is_digit(10)).collect();
                usize::from_str_radix(
                    &format!("{}{}", digits.first().unwrap(), digits.last().unwrap()),
                    10,
                )
                .unwrap()
            })
            .sum()
    }

    fn part_two(input: &Vec<&'_ str>) -> usize {
        input
            .iter()
            .map(|line| {
                let chars: Vec<_> = line.chars().collect();
                let digits: Vec<_> = (0..chars.len())
                    .filter_map(|i| {
                        let chunk: String = chars.iter().skip(i).collect();
                        let first = chunk.chars().next().unwrap();
                        if first.is_digit(10) {
                            Some(first)
                        } else if chunk.starts_with("one") {
                            Some('1')
                        } else if chunk.starts_with("two") {
                            Some('2')
                        } else if chunk.starts_with("three") {
                            Some('3')
                        } else if chunk.starts_with("four") {
                            Some('4')
                        } else if chunk.starts_with("five") {
                            Some('5')
                        } else if chunk.starts_with("six") {
                            Some('6')
                        } else if chunk.starts_with("seven") {
                            Some('7')
                        } else if chunk.starts_with("eight") {
                            Some('8')
                        } else if chunk.starts_with("nine") {
                            Some('9')
                        } else {
                            None
                        }
                    })
                    .collect();
                usize::from_str_radix(
                    &format!("{}{}", digits.first().unwrap(), digits.last().unwrap()),
                    10,
                )
                .unwrap()
            })
            .sum()
    }
}

impl Test<DAY_01> for Solutions {
    fn expected(part: bool) -> usize {
        match part {
            PART_ONE => 142,
            PART_TWO => 281,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solutions;
    use lib_aoc::prelude::*;

    derive_tests!(Solutions, DAY_01);
}
