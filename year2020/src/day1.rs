/// https://adventofcode.com/2020/day/1
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().expect("posative integers only")).collect()
}

fn find_pair_sum(report: &[usize], sum: usize) -> Option<(usize, usize)> {
    for i in 0..(report.len() - 1) {
        for j in (i+1)..report.len() {
            if report[i] + report[j] == sum {
                return Some((report[i], report[j]));
            }
        }
    }
    None
}

fn find_pair_sum_search(report: &[usize], sum: usize) -> Option<(usize, usize)> {
    for value in report {
        let want = sum - value;
        if want <= 0 {
            continue
        }
        match report.binary_search(&want) {
            Ok(v) => return Some((*value, report[v])),
            Err(_) => continue,
        }
    }
    None
}

#[aoc(day1, part1, unsorted)]
pub fn part1_unsorted(report: &Vec<usize>) -> Option<usize> {
    match find_pair_sum(report, 2020) {
        Some((left, right)) => Some(left * right),
        None => None
    }
}

#[aoc(day1, part1, sorted)]
pub fn part1_sorted(report: &Vec<usize>) -> Option<usize> {
    let mut report = report.clone();
    report.sort_unstable();
    match find_pair_sum(&report, 2020) {
        Some((left, right)) => Some(left * right),
        None => None
    }
}

#[aoc(day1, part1, sorted_search)]
pub fn part1_sorted_search(report: &Vec<usize>) -> Option<usize> {
    let mut report = report.clone();
    report.sort_unstable();
    match find_pair_sum_search(&report, 2020) {
        Some((left, right)) => Some(left * right),
        None => None
    }
}

#[aoc(day1, part1, sorted_ends)]
pub fn part1_sorted_ends(report: &Vec<usize>) -> Option<usize> {
    let mut report = report.clone();
    report.sort_unstable();
    let mut left = 0;
    let mut right = report.len()-1;
    loop {
        if left >= right {
            return None
        }
        let leftv = report[left];
        let rightv = report[right];
        match leftv + rightv {
            2020 => return Some(leftv *rightv),
            sum if sum > 2020 => {
                right -= 1;
            }
            sum if sum < 2020 => {
                left += 1;
            }
            _ => unreachable!("match statement checks for equality, greater than and less than 2020")
        }
    }
}

#[aoc(day1, part2, unsorted)]
pub fn part2_unsorted(report: &Vec<usize>) -> Option<usize> {
    for i in 0..(report.len() - 2) {
        for j in (i+1)..(report.len() - 1) {
            if report[i] + report[j] >= 2020 {
                continue;
            }
            for k in (j+1)..report.len() {
                if report[i] + report[j] + report[k] == 2020 {
                    return Some(report[i] * report[j] * report[k]);
                }
            }
        }
    }
    None
}

#[aoc(day1, part2, sorted)]
pub fn part2_sorted(report: &Vec<usize>) -> Option<usize> {
    let mut report = report.clone();
    report.sort_unstable();
    for i in 0..(report.len() - 2) {
        match find_pair_sum(&report[i+1..], 2020-report[i]) {
            None => continue,
            Some((left,right)) => return Some(left * right * report[i])
        }
    }
    None
}

#[aoc(day1, part2, sorted_search)]
pub fn part2_sorted_search(report: &Vec<usize>) -> Option<usize> {
    let mut report = report.clone();
    report.sort_unstable();
    for i in 0..(report.len() - 2) {
        match find_pair_sum_search(&report[i+1..], 2020-report[i]) {
            None => continue,
            Some((left,right)) => return Some(left * right * report[i])
        }
    }
    None
}

#[allow(dead_code, unused_imports)]
mod test {
    use super::*;

    static REPORT: [usize; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    fn part1() {
        assert_eq!(part1_unsorted(&REPORT.to_vec()), Some(514579));
        assert_eq!(part1_sorted(&REPORT.to_vec()), Some(514579));
        assert_eq!(part1_sorted_search(&REPORT.to_vec()), Some(514579));
        assert_eq!(part1_sorted_ends(&REPORT.to_vec()), Some(514579));
    }

    #[test]
    fn part2() {
        assert_eq!(part2_sorted(&REPORT.to_vec()), Some(241861950));
        assert_eq!(part2_unsorted(&REPORT.to_vec()), Some(241861950));
        assert_eq!(part2_sorted_search(&REPORT.to_vec()), Some(241861950));
    }
}
