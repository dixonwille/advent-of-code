/// https://adventofcode.com/2020/day/23
use std::hash::Hash;
use std::{cell::RefCell, fmt::Display};
use std::{collections::HashMap, rc::Rc};

#[aoc_generator(day23)]
fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .into_iter()
        .map(|b| b.to_digit(10).unwrap() as usize)
        .collect()
}

type NodeRef<T> = Rc<RefCell<Node<T>>>;
type NodeOption<T> = Option<NodeRef<T>>;

#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    next: NodeOption<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> NodeRef<T> {
        Rc::new(RefCell::new(Node { data, next: None }))
    }
}

#[derive(Debug, Clone)]
struct WrappedLinkedList<T> {
    cursor: NodeOption<T>,
    length: usize,
}

impl<T: Display + std::cmp::PartialEq<usize>> Display for WrappedLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.cursor {
            Some(ref current) => {
                let mut start = Rc::clone(current);
                // navigate to the 1 in the sequence
                while start.borrow().data != 1 {
                    if let Some(ref next) = Rc::clone(&start).borrow().next {
                        start = Rc::clone(next);
                    }
                }
                // print out whatever is not 1
                for _ in 0..self.length {
                    if start.borrow().data != 1 {
                        write!(f, "{}", start.borrow().data)?;
                    }
                    if let Some(ref next) = Rc::clone(&start).borrow().next {
                        start = Rc::clone(next);
                    }
                }
                Ok(())
            }
            None => Ok(()),
        }
    }
}

impl<T: std::cmp::PartialEq<usize>> WrappedLinkedList<T> {
    fn new(mut list: Vec<T>) -> WrappedLinkedList<T> {
        let size = list.len();
        let mut wlist = WrappedLinkedList {
            cursor: None,
            length: size,
        };
        let mut last: NodeOption<T> = None;
        while let Some(item) = list.pop() {
            let new_node = Node::new(item);
            if let Some(node) = wlist.cursor.take() {
                new_node.borrow_mut().next = Some(Rc::clone(&node));
            }
            if last.is_none() {
                last = Some(Rc::clone(&new_node));
            }
            wlist.cursor = Some(new_node);
        }
        let first = wlist.cursor.take().unwrap();
        last.unwrap().borrow_mut().next = Some(Rc::clone(&first));
        wlist.cursor = Some(first);
        wlist
    }

    fn move_cursor(&mut self) {
        if let Some(current) = self.cursor.take() {
            if let Some(ref next) = current.borrow().next {
                self.cursor = Some(Rc::clone(next));
            }
        }
    }

    fn move_cursor_to_one(&mut self) {
        while self.cursor.as_ref().unwrap().borrow().data != 1 {
            self.move_cursor();
        }
    }

    fn next_three(&self) -> (NodeRef<T>, NodeRef<T>, NodeRef<T>) {
        let a = Rc::clone(
            self.cursor
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap(),
        );
        let b = Rc::clone(a.borrow().next.as_ref().unwrap());
        let c = Rc::clone(b.borrow().next.as_ref().unwrap());
        (a, b, c)
    }
}

fn play_game<T>(cups: &mut WrappedLinkedList<T>, rounds: usize)
where
    T: std::ops::Sub<usize, Output = usize>
        + PartialEq<usize>
        + Copy
        + Hash
        + Eq
        + std::borrow::Borrow<usize>,
{
    let mut cache = HashMap::new();
    for _ in 0..rounds {
        // get the cups to move
        let (a, b, c) = cups.next_three();
        // find the target value
        let mut t = cups.length;
        if cups.cursor.as_ref().unwrap().borrow().data != 1 {
            t = cups.cursor.as_ref().unwrap().borrow().data - 1;
        }
        while a.borrow().data == t || b.borrow().data == t || c.borrow().data == t {
            if t == 1 {
                t = cups.length;
            } else {
                t -= 1;
            }
        }

        // move a local cursor in our list to the target
        let dest = match cache.get(&t) {
            None => {
                let mut dest = Rc::clone(
                    cups.cursor
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .next
                        .as_ref()
                        .unwrap(),
                );
                while dest.borrow().data != t {
                    if let Some(ref next) = Rc::clone(&dest).borrow().next {
                        cache
                            .entry(next.borrow().data)
                            .or_insert_with(|| Rc::clone(next));
                        dest = Rc::clone(next);
                    }
                }
                dest
            }
            Some(dest) => Rc::clone(dest),
        };

        // update links
        cups.cursor.as_ref().unwrap().borrow_mut().next =
            Some(Rc::clone(c.borrow().next.as_ref().unwrap()));
        c.borrow_mut().next = Some(Rc::clone(dest.borrow().next.as_ref().unwrap()));
        dest.borrow_mut().next = Some(Rc::clone(&a));
        cups.move_cursor();
    }
}

#[aoc(day23, part1)]
fn part1(cups: &[usize]) -> String {
    let mut cups = WrappedLinkedList::new(cups.to_vec());
    play_game(&mut cups, 100);
    format!("{}", cups)
}
#[aoc(day23, part2)]
fn part2(cups: &[usize]) -> usize {
    let mut cups = cups.to_vec();
    cups.resize(1_000_000, 0);
    cups = cups
        .into_iter()
        .enumerate()
        .map(|(i, c)| match c {
            0 => i + 1,
            _ => c,
        })
        .collect();
    let mut cups = WrappedLinkedList::new(cups);
    play_game(&mut cups, 10_000_000);
    cups.move_cursor_to_one();
    let (a, b, _) = cups.next_three();
    let prod = a.borrow().data * b.borrow().data;
    prod
}

#[cfg(test)]
mod test {
    use super::*;
    static CUPS: &str = "389125467";

    #[test]
    fn parsing_input() {
        let cups = parse_input(CUPS);
        assert_eq!(cups, vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);
    }

    #[test]
    fn running_part1() {
        let cups = parse_input(CUPS);
        assert_eq!(part1(&cups), "67384529");
    }

    #[test]
    #[ignore = "long running test"]
    fn running_part2() {
        let cups = parse_input(CUPS);
        assert_eq!(part2(&cups), 149245887792);
    }
}
