/// https://adventofcode.com/2020/day/10
use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<usize> {
    let mut out: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();
    out.sort_unstable();
    out
}

#[aoc(day10, part1)]
fn part1(adapters: &[usize]) -> usize {
    let (_, diff1, diff3) = adapters
        .iter()
        .fold((0, 0, 1), |i, adapter| match adapter - i.0 {
            1 => (*adapter, i.1 + 1, i.2),
            3 => (*adapter, i.1, i.2 + 1),
            _ => (*adapter, i.1, i.2),
        });
    diff1 * diff3
}

#[aoc(day10, part2)]
fn part2(adapters: &[usize]) -> usize {
    let device_jolts = adapters.iter().max().unwrap() + 3;
    let graph = build_graph(adapters, &device_jolts);
    let mut cache = HashMap::new();
    path_count(&graph, &0, &device_jolts, &mut cache)
}

// Can make assumptions like a node will never fall back to itself so don't need to keep track of visited
fn path_count<'a>(
    graph: &'a HashMap<&'a usize, Vec<&'a usize>>,
    start: &'a usize,
    end: &'a usize,
    cache: &mut HashMap<&'a usize, usize>,
) -> usize {
    if start == end {
        1
    } else {
        graph.get(start).unwrap().iter().fold(0, |count, adj|{
            match cache.get(adj) {
                Some(c) => count + c,
                None => {
                    let c = path_count(graph, adj, end, cache);
                    cache.insert(adj, c);
                    count + c
                }
            }
        })
    }
}

fn build_graph<'a>(
    adapters: &'a [usize],
    device_jolts: &'a usize,
) -> HashMap<&'a usize, Vec<&'a usize>> {
    let mut graph: HashMap<_, Vec<_>> = HashMap::new();

    //Add from wall to first adapter
    for i in 1 as usize..4 {
        if let Ok(idx) = adapters.binary_search(&i) {
            graph
                .entry(&0)
                .or_default()
                .push(adapters.get(idx).unwrap());
        }
    }

    //Add from last adapter to device
    graph
        .entry(adapters.iter().max().unwrap())
        .or_default()
        .push(device_jolts);

    adapters.iter().fold(graph, |mut g, adapter| {
        for i in 1 as usize..4 {
            if let Ok(idx) = adapters.binary_search(&(adapter + i)) {
                g.entry(adapter)
                    .or_default()
                    .push(adapters.get(idx).unwrap());
            }
        }
        g
    })
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
