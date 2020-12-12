/// https://adventofcode.com/2020/day/10

// I like this guys approach better https://github.com/timvisee/advent-of-code-2020
// TODO: dig into it more and make it more legible (comment the code etc.)
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    character::complete::char as c,
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list1},
    IResult,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Position {
    Empty,
    Occupied,
    Floor,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct FloorPlan {
    rows: usize,
    cols: usize,
    plan: Vec<Position>,
}

impl FloorPlan {
    fn new(raw: Vec<Vec<Position>>) -> Self {
        FloorPlan {
            rows: raw.len(),
            cols: raw[0].len(),
            plan: raw.into_iter().flatten().collect(),
        }
    }

    fn occupied(&self) -> usize {
        self.plan.iter().fold(0, |count, pos| match pos {
            Position::Occupied => count + 1,
            _ => count,
        })
    }

    fn state(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn step_part1(&self, crowded: usize) -> Self {
        let mut new_plan = Vec::with_capacity(self.plan.len());
        for i in 0..self.plan.len() {
            new_plan.push(self.step_index_part1(i, crowded));
        }
        FloorPlan {
            rows: self.rows,
            cols: self.cols,
            plan: new_plan,
        }
    }

    fn step_index_part1(&self, idx: usize, crowded: usize) -> Position {
        let cur = self.plan.get(idx).unwrap();
        if cur == &Position::Floor {
            return Position::Floor;
        }
        let mut occupied = Vec::with_capacity(8); // set capacity to max size
        let read_right = idx % self.cols != self.cols - 1;
        let read_left = idx % self.cols != 0;
        // only read if I am not in the top row
        if idx >= self.cols {
            occupied.push(self.neighbor(idx, top_vec)); // top
            if read_left {
                occupied.push(self.neighbor(idx, top_left_vec)); // top left
            }
            if read_right {
                occupied.push(self.neighbor(idx, top_right_vec)); // top right
            }
        }
        if read_left {
            occupied.push(self.neighbor(idx, left_vec)); // left
        }
        if read_right {
            occupied.push(self.neighbor(idx, right_vec)); // right
        }
        // only read if I am not in the bottom row
        if idx < self.cols * (self.rows - 1) {
            occupied.push(self.neighbor(idx, bottom_vec)); // bottom
            if read_left {
                occupied.push(self.neighbor(idx, bottom_left_vec)); // bottom left
            }
            if read_right {
                occupied.push(self.neighbor(idx, bottom_right_vec)); // bottom right
            }
        }
        let count = occupied.into_iter().fold(0, |count, pos| match pos {
            Position::Floor | Position::Empty => count,
            Position::Occupied => count + 1,
        });
        match (cur, count) {
            (Position::Empty, 0) => Position::Occupied,
            (Position::Occupied, c) if c >= crowded => Position::Empty,
            _ => cur.clone(),
        }
    }

    fn step_part2(&self, crowded: usize) -> Self {
        let mut new_plan = Vec::with_capacity(self.plan.len());
        for i in 0..self.plan.len() {
            new_plan.push(self.step_index_part2(i, crowded));
        }
        FloorPlan {
            rows: self.rows,
            cols: self.cols,
            plan: new_plan,
        }
    }

    fn step_index_part2(&self, idx: usize, crowded: usize) -> Position {
        let cur = self.plan.get(idx).unwrap();
        if cur == &Position::Floor {
            return Position::Floor;
        }
        let mut occupied = Vec::with_capacity(8);
        let read_right = idx % self.cols != self.cols - 1;
        let read_left = idx % self.cols != 0;
        // only read if I am not in the top row
        if idx >= self.cols {
            occupied.push(self.vec(idx, top_vec)); // top
            if read_left {
                occupied.push(self.vec(idx, top_left_vec)); // top left
            }
            if read_right {
                occupied.push(self.vec(idx, top_right_vec)); // top right
            }
        }
        if read_left {
            occupied.push(self.vec(idx, left_vec)); // left
        }
        if read_right {
            occupied.push(self.vec(idx, right_vec)); // right
        }
        // only read if I am not in the bottom row
        if idx < self.cols * (self.rows - 1) {
            occupied.push(self.vec(idx, bottom_vec)); // bottom
            if read_left {
                occupied.push(self.vec(idx, bottom_left_vec)); // bottom left
            }
            if read_right {
                occupied.push(self.vec(idx, bottom_right_vec)); // bottom right
            }
        }
        let count = occupied.into_iter().fold(0, |count, pos| match pos {
            Position::Floor | Position::Empty => count,
            Position::Occupied => count + 1,
        });
        match (cur, count) {
            (Position::Empty, 0) => Position::Occupied,
            (Position::Occupied, c) if c >= crowded => Position::Empty,
            _ => cur.clone(),
        }
    }

    fn neighbor<F>(&self, idx: usize, f: F) -> Position
    where
        F: Fn((usize, usize), usize, usize) -> Option<usize>,
    {
        let i = f((self.rows, self.cols), idx, 0);
        match i {
            Some(j) => match self.plan.get(j).unwrap() {
                Position::Floor => Position::Floor,
                Position::Empty => Position::Empty,
                Position::Occupied => Position::Occupied,
            },
            None => Position::Floor,
        }
    }

    fn vec<F>(&self, idx: usize, f: F) -> Position
    where
        F: Fn((usize, usize), usize, usize) -> Option<usize>,
    {
        let mut it = 0;
        loop {
            let i = f((self.rows, self.cols), idx, it);
            match i {
                Some(j) => match self.plan.get(j).unwrap() {
                    Position::Floor => {}
                    Position::Empty => break Position::Empty,
                    Position::Occupied => break Position::Occupied,
                },
                None => break Position::Floor,
            }

            it += 1;
        }
    }
}

fn top_vec((_, cols): (usize, usize), idx: usize, it: usize) -> Option<usize> {
    let i = it + 1;
    if idx / cols < i {
        return None;
    }
    Some(idx - cols * i)
}

fn top_left_vec((_, cols): (usize, usize), idx: usize, it: usize) -> Option<usize> {
    let i = it + 1;
    if idx / cols < i || idx % cols < i {
        return None;
    }
    Some(idx - cols * i - i)
}

fn top_right_vec((_, cols): (usize, usize), idx: usize, it: usize) -> Option<usize> {
    let i = it + 1;
    if idx / cols < i || cols - (idx % cols) - 1 < i {
        return None;
    }
    Some(idx - cols * i + i)
}

fn left_vec((_, cols): (usize, usize), idx: usize, it: usize) -> Option<usize> {
    let i = it + 1;
    if idx % cols < i {
        return None;
    }
    Some(idx - i)
}

fn right_vec((_, cols): (usize, usize), idx: usize, it: usize) -> Option<usize> {
    let i = it + 1;
    if cols - (idx % cols) - 1 < i {
        return None;
    }
    Some(idx + i)
}

fn bottom_vec((rows, cols): (usize, usize), idx: usize, it: usize) -> Option<usize> {
    let i = it + 1;
    if rows - (idx / cols) - 1 < i {
        return None;
    }
    Some(idx + cols * i)
}

fn bottom_left_vec((rows, cols): (usize, usize), idx: usize, it: usize) -> Option<usize> {
    let i = it + 1;
    if rows - (idx / cols) - 1 < i || idx % cols < i {
        return None;
    }
    Some(idx + cols * i - i)
}

fn bottom_right_vec((rows, cols): (usize, usize), idx: usize, it: usize) -> Option<usize> {
    let i = it + 1;
    if rows - (idx / cols) - 1 < i || cols - (idx % cols) - 1 < i {
        return None;
    }
    Some(idx + cols * i + i)
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> FloorPlan {
    let (_, seats) = parse_input_nom(input).unwrap();
    seats
}

fn parse_input_nom(input: &str) -> IResult<&str, FloorPlan> {
    map(
        all_consuming(separated_list1(
            c('\n'),
            many1(alt((
                value(Position::Floor, c('.')),
                value(Position::Empty, c('L')),
            ))),
        )),
        FloorPlan::new,
    )(input)
}

#[aoc(day11, part1)]
fn part1(floor_plan: &FloorPlan) -> usize {
    let mut current = floor_plan.clone();
    let mut state = floor_plan.state();
    loop {
        let new_floor_plan = current.step_part1(4);
        let new_state = new_floor_plan.state();
        if new_state == state {
            break;
        }
        current = new_floor_plan;
        state = new_state;
    }
    current.occupied()
}

#[aoc(day11, part2)]
fn part2(floor_plan: &FloorPlan) -> usize {
    let mut current = floor_plan.clone();
    let mut state = floor_plan.state();
    loop {
        let new_floor_plan = current.step_part2(5);
        let new_state = new_floor_plan.state();
        if new_state == state {
            break;
        }
        current = new_floor_plan;
        state = new_state;
    }
    current.occupied()
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn parsing_input() {
        use Position::*;
        assert_eq!(
            parse_input(INPUT),
            FloorPlan {
                rows: 10,
                cols: 10,
                plan: vec![
                    Empty, Floor, Empty, Empty, Floor, Empty, Empty, Floor, Empty, Empty, Empty,
                    Empty, Empty, Empty, Empty, Empty, Empty, Floor, Empty, Empty, Empty, Floor,
                    Empty, Floor, Empty, Floor, Floor, Empty, Floor, Floor, Empty, Empty, Empty,
                    Empty, Floor, Empty, Empty, Floor, Empty, Empty, Empty, Floor, Empty, Empty,
                    Floor, Empty, Empty, Floor, Empty, Empty, Empty, Floor, Empty, Empty, Empty,
                    Empty, Empty, Floor, Empty, Empty, Floor, Floor, Empty, Floor, Empty, Floor,
                    Floor, Floor, Floor, Floor, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
                    Empty, Empty, Empty, Empty, Floor, Empty, Empty, Empty, Empty, Empty, Empty,
                    Floor, Empty, Empty, Floor, Empty, Empty, Empty, Empty, Empty, Floor, Empty,
                    Empty
                ]
            }
        );
    }

    #[test]
    fn running_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 37);
    }

    #[test]
    fn running_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 26);
    }
}
