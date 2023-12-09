use std::{cell::Cell, collections::HashMap};

use crate::Solutions;
use lib_aoc::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char as single_char, newline},
    combinator::{map, map_res, opt},
    multi::{fold_many1, many1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};
use num::Integer;
use typed_arena::Arena;

struct Graph<'a> {
    nodes: Arena<Node<'a>>,
}

impl<'a> Default for Graph<'a> {
    fn default() -> Self {
        Graph {
            nodes: Arena::new(),
        }
    }
}

impl<'a> Graph<'a> {
    fn init_part_a(&'a self, value: &HashMap<String, (String, String)>) -> &Node<'_> {
        let mut nodes = HashMap::with_capacity(value.len());
        let mut starts = None;

        // Create all the nodes first
        for (k, _) in value {
            let root = Node::new(k.to_owned(), &self.nodes);
            nodes.insert(k, root);
            if k == "AAA" {
                starts = Some(root);
            }
        }

        // Create all the edges
        for (o, (l, r)) in value {
            let &orig = nodes.get(o).unwrap();
            let &left = nodes.get(l).unwrap();
            let &right = nodes.get(r).unwrap();
            orig.left.set(Some(left));
            orig.right.set(Some(right));
        }
        starts.unwrap()
    }

    fn init_part_b(&'a self, value: &HashMap<String, (String, String)>) -> Vec<&Node<'_>> {
        let mut nodes = HashMap::with_capacity(value.len());
        let mut starts = Vec::new();

        // Create all the nodes first
        for (k, _) in value {
            let root = Node::new(k.to_owned(), &self.nodes);
            nodes.insert(k, root);
            if k.ends_with("A") {
                starts.push(root);
            }
        }

        // Create all the edges
        for (o, (l, r)) in value {
            let &orig = nodes.get(o).unwrap();
            let &left = nodes.get(l).unwrap();
            let &right = nodes.get(r).unwrap();
            orig.left.set(Some(left));
            orig.right.set(Some(right));
        }
        starts
    }

    fn num_steps(&self, start: &Node<'_>, instructions: &Vec<Direction>) -> usize {
        let mut i: usize = 0;
        let mut s = start;
        loop {
            let dir = instructions.get(i % instructions.len()).unwrap();
            let new = match dir {
                Direction::Left => &s.left,
                Direction::Right => &s.right,
            };

            s = new.get().unwrap();

            if s.id.ends_with("Z") {
                break;
            }

            i += 1;
        }
        i + 1
    }
}

struct Node<'a> {
    id: String,
    left: Cell<Option<&'a Node<'a>>>,
    right: Cell<Option<&'a Node<'a>>>,
}

impl<'a> Node<'a> {
    fn new(id: String, arena: &'a Arena<Node<'a>>) -> &'a Node<'a> {
        arena.alloc(Node {
            id,
            left: Cell::new(None),
            right: Cell::new(None),
        })
    }
}

#[derive(Debug)]
pub struct Map {
    instructions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn get_direction(input: &str) -> IResult<&str, Direction> {
    map_res(alt((single_char('L'), single_char('R'))), |c| match c {
        'L' => Ok(Direction::Left),
        'R' => Ok(Direction::Right),
        _ => Err("unkown direction"),
    })(input)
}

fn get_instructions(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(get_direction)(input)
}

fn get_node_paths(input: &str) -> IResult<&str, (&str, &str)> {
    delimited(
        single_char('('),
        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
        single_char(')'),
    )(input)
}

fn get_node(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(alphanumeric1, tag(" = "), get_node_paths)(input)
}

fn get_map(input: &str) -> IResult<&str, Map> {
    map(
        separated_pair(
            get_instructions,
            many1(newline),
            fold_many1(
                terminated(get_node, opt(newline)),
                HashMap::new,
                |mut acc, (key, (left, right))| {
                    acc.insert(key.to_owned(), (left.to_owned(), right.to_owned()));
                    acc
                },
            ),
        ),
        |(instructions, nodes)| Map {
            instructions,
            nodes,
        },
    )(input)
}

impl Solution<DAY_08> for Solutions {
    type Input<'i> = Map;
    type Output = usize;

    fn parse(puzzle: &str) -> Map {
        let (_, m) = get_map(puzzle).unwrap();
        m
    }

    fn part_one(input: &Map) -> usize {
        let g = Graph::default();
        let start = g.init_part_a(&input.nodes);
        g.num_steps(start, &input.instructions)
    }

    fn part_two(input: &Map) -> usize {
        let g = Graph::default();
        let starts = g.init_part_b(&input.nodes);

        starts
            .iter()
            .map(|s| g.num_steps(s, &input.instructions))
            .reduce(|acc, s| acc.lcm(&s))
            .unwrap()
    }
}

impl Test<DAY_08> for Solutions {
    fn expected(part: bool) -> usize {
        match part {
            PART_ONE => 6,
            PART_TWO => 6,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solutions;
    use lib_aoc::prelude::*;

    derive_tests!(Solutions, DAY_08);
}
