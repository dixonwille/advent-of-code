/// https://adventofcode.com/2020/day/9

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
fn part1(xmas: &[usize]) -> Option<usize> {
    first_invalid(xmas, 25)
}

// returns index then invalid value
fn first_invalid(xmas: &[usize], preamble_size: usize) -> Option<usize> {
    let mut pre_start = 0;
    let mut idx = pre_start + preamble_size;
    let mut num = xmas[idx];

    loop {
        if find_pair_sum(&xmas[pre_start..idx], num).is_none() {
            return Some(num);
        }
        pre_start += 1;
        idx += 1;
        if idx == xmas.len() {
            break;
        }
        num = xmas[idx];
    }
    None
}

// Stolen from day1. It was faster to sort before hand, but as often this gets called, may be more efficient to not sort
fn find_pair_sum(preamble: &[usize], sum: usize) -> Option<(usize, usize)> {
    for i in 0..(preamble.len() - 1) {
        for j in (i + 1)..preamble.len() {
            if preamble[i] + preamble[j] == sum {
                return Some((preamble[i], preamble[j]));
            }
        }
    }
    None
}

#[aoc(day9, part2)]
fn part2(xmas: &[usize]) -> Option<usize> {
    find_weakness(xmas, 25)
}

fn find_weakness(xmas: &[usize], preamble_size: usize) -> Option<usize> {
    let invalid = first_invalid(xmas, preamble_size)?;
    let contiguous = find_contiguous_sum(xmas, invalid)?;
    Some(contiguous.iter().min()? + contiguous.iter().max()?)
}

fn find_contiguous_sum(xmas: &[usize], sum: usize) -> Option<&[usize]> {
    let mut left_most = 0;
    let mut right_most = 2; // exclusive
    loop {
        let contiguous = &xmas[left_most..right_most];
        let contiguous_sum: usize = contiguous.iter().sum();
        if contiguous_sum > sum {
            left_most += 1;
        }
        if contiguous_sum < sum {
            right_most += 1;
        }
        if contiguous_sum == sum {
            return Some(contiguous);
        }
        if left_most == right_most {
            right_most += 1; // can't add the same value twice
        }
        if right_most == xmas.len() + 1 {
            break;
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    static XMAS: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn prasing_input() {
        assert_eq!(
            parse_input(XMAS),
            vec![
                35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                309, 576
            ]
        )
    }

    #[test]
    fn running_part1() {
        let xmas = parse_input(XMAS);
        assert_eq!(first_invalid(&xmas, 5), Some(127));
    }

    #[test]
    fn running_part2() {
        let xmas = parse_input(XMAS);
        assert_eq!(find_weakness(&xmas, 5), Some(62));
    }
}
