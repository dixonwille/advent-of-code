/// https://adventofcode.com/2021/day/3

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|l| u16::from_str_radix(l, 2).expect("not a binary number"))
        .collect()
}

#[aoc(day3, part1)]
fn part1(diags: &[u16]) -> Option<usize> {
    let (gamma, epsilon) = get_gamma_epsilon(diags, 12);
    Some(gamma * epsilon)
}

fn get_gamma_epsilon(diags: &[u16], size: u8) -> (usize, usize) {
    let gamma = (0..size)
        .rev()
        .map(|i| {
            let mask = 1 << i;
            get_most_common(diags, mask)
        })
        .fold(0_usize, |acc, b| (acc << 1) + b as usize);
    let epsilon = gamma ^ (0..size).rev().fold(0, |acc, _| (acc << 1) + 1);
    (gamma, epsilon)
}

#[aoc(day3, part2)]
fn part2(diags: &[u16]) -> Option<usize> {
    let (o2, co2) = get_o2_co2_ratings(diags, 12);
    Some(o2 * co2)
}

fn get_o2_co2_ratings(diags: &[u16], size: u8) -> (usize, usize) {
    let mut o2 = diags.to_owned();
    let mut co2 = diags.to_owned();
    let mut current = (size - 1) as i8;
    while (o2.len() > 1 || co2.len() > 1) && current != -1 {
        let mask = 1 << current;
        o2 = filter_o2(&o2, mask);
        co2 = filter_co2(&co2, mask);
        current -= 1;
    }
    (o2[0].into(), co2[0].into())
}

// could possibly make faster by sorting the array first and use edge detection
// on each bit to determine if either 1 or 0 is the most common for that bit
fn get_most_common(diags: &[u16], mask: u16) -> u8 {
    let half_diags = diags.len() as f32 / 2.0;
    let on_bits = diags
        .iter()
        .fold(0, |acc, d| if d & mask == mask { acc + 1 } else { acc });
    if on_bits as f32 >= half_diags {
        1
    } else {
        0
    }
}


// I wonder if there is a way to be more memory effecient than to copy the values
// like it is doing when length is 0. I am not modifying the original source and 
// really just need a reference to the values in the original array. New vec makes
// sense but copying the values doesn't
fn filter_o2(diags: &[u16], mask: u16) -> Vec<u16> {
    if diags.len() == 1 {
        return diags.to_owned();
    }

    let most = get_most_common(diags, mask);
    filter(diags, mask, most)
}

fn filter_co2(diags: &[u16], mask: u16) -> Vec<u16> {
    if diags.len() == 1 {
        return diags.to_owned();
    }

    let least = get_most_common(diags, mask) ^ 0b1;
    filter(diags, mask, least)
}

fn filter(diags: &[u16], mask: u16, value: u8) -> Vec<u16> {
    diags
        .iter()
        .filter(|diag| match value {
            1 => *diag & mask == mask,
            0 => *diag & mask == 0,
            _ => false,
        })
        .map(|d| d.to_owned())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    static DIAG: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn parsing_input_test() {
        assert_eq!(
            parse_input(DIAG),
            vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10]
        )
    }

    #[test]
    fn part1_test() {
        let diags = parse_input(DIAG);
        let (gamma, epsilon) = get_gamma_epsilon(&diags, 5);
        assert_eq!(gamma * epsilon, 198)
    }

    #[test]
    fn part2_test() {
        let diags = parse_input(DIAG);
        let (o2, co2) = get_o2_co2_ratings(&diags, 5);
        assert_eq!(o2 * co2, 230)
    }
}
