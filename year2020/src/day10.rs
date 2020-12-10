/// https://adventofcode.com/2020/day/10
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<usize> {
    let mut out: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();
    out.sort_unstable();
    out
}

#[aoc(day10, part1)]
fn part1(adapters: &[usize]) -> usize {
    let (_, diff1, diff3) = adapters.iter().fold((0,0,1), |i, adapter| {
        match adapter - i.0{
            1 => (*adapter, i.1 + 1, i.2),
            3 => (*adapter, i.1, i.2 + 1),
            _ => (*adapter, i.1, i.2)
        }
    });
    diff1 * diff3
}

#[aoc(day10, part2)]
fn part2(_input: &[usize]) -> usize {
    // create graph
    // backtrack to find total number of paths
    // https://www.includehelp.com/data-structure-tutorial/count-all-the-possible-path-between-two-vertices.aspx
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    static ADAPTERS: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn parsing_input() {
        assert_eq!(
            parse_input(ADAPTERS),
            vec![
                1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34,
                35, 38, 39, 42, 45, 46, 47, 48, 49
            ]
        )
    }

    #[test]
    fn running_part1() {
        let input = parse_input(ADAPTERS);
        assert_eq!(part1(&input), 220)
    }

    #[test]
    fn running_part2() {
        let input = parse_input(ADAPTERS);
        assert_eq!(part2(&input), 19208)
    }
}
