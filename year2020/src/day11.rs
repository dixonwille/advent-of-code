/// https://adventofcode.com/2020/day/11
use std::{
    hash::{Hash},
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
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> FloorPlan {
    FloorPlan::new(
        input
            .lines()
            .map(|l| {
                l.as_bytes()
                    .iter()
                    .map(|b| match b {
                        b'.' => Position::Floor,
                        b'L' => Position::Empty,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
}

// Works better since we find the indexes we want to look for before playing simulation
// https://github.com/timvisee/advent-of-code-2020/blob/master/day11a/src/main.rs
#[aoc(day11, part1)]
fn part1(floor_plan: &FloorPlan) -> usize {
    let neighbors = get_neighbors(floor_plan);
    let (mut cur, mut prev) = (floor_plan.clone(), floor_plan.clone());

    loop {
        for (i, visible) in &neighbors {
            let occup = visible
                .iter()
                .filter(|o| prev.plan[**o] == Position::Occupied)
                .count();
            let (cur_seat, prev_seat) = (&mut cur.plan[*i], &prev.plan[*i]);

            if prev_seat == &Position::Empty && occup == 0 {
                *cur_seat = Position::Occupied;
            } else if prev_seat == &Position::Occupied && occup >= 4 {
                *cur_seat = Position::Empty;
            } else {
                *cur_seat = prev_seat.clone()
            }
        }
        // current iteration is now previous
        // we can use what is in previous as next current as we are always overwritting it's values
        std::mem::swap(&mut cur, &mut prev);
        if cur == prev {
            break;
        }
    }
    cur.plan
        .iter()
        .filter(|p| p == &&Position::Occupied)
        .count()
}

// Works better since we find the indexes we want to look for before playing simulation
// https://github.com/timvisee/advent-of-code-2020/blob/master/day11b/src/main.rs
#[aoc(day11, part2)]
fn part2(floor_plan: &FloorPlan) -> usize {
    let aof = get_aof(floor_plan);
    let (mut cur, mut prev) = (floor_plan.clone(), floor_plan.clone());

    loop {
        for (i, visible) in &aof {
            let occup = visible
                .iter()
                .filter(|o| prev.plan[**o] == Position::Occupied)
                .count();
            let (cur_seat, prev_seat) = (&mut cur.plan[*i], &prev.plan[*i]);

            if prev_seat == &Position::Empty && occup == 0 {
                *cur_seat = Position::Occupied;
            } else if prev_seat == &Position::Occupied && occup >= 5 {
                *cur_seat = Position::Empty;
            } else {
                *cur_seat = prev_seat.clone()
            }
        }
        // current iteration is now previous
        // we can use what is in previous as next current as we are always overwritting it's values
        std::mem::swap(&mut cur, &mut prev);
        if cur == prev {
            break;
        }
    }
    cur.plan
        .iter()
        .filter(|p| p == &&Position::Occupied)
        .count()
}

fn get_neighbors(floor_plan: &FloorPlan) -> Vec<(usize, Vec<usize>)> {
    floor_plan
        .plan
        .iter()
        .enumerate()
        .filter(|(_, p)| p != &&Position::Floor)
        .map(|(i, _)| {
            (
                i,
                (0..9)
                    .filter(|p| p != &4) // exclude middle
                    // generate x and y neighbors for i
                    .map(|p| {
                        (
                            (i % floor_plan.cols) as isize + p as isize % 3 - 1,
                            (i / floor_plan.cols) as isize + p as isize / 3 - 1,
                        )
                    })
                    // make sure we don't breach boundaries
                    .filter(|(x, y)| {
                        *x >= 0
                            && *y >= 0
                            && *x < floor_plan.cols as isize
                            && *y < floor_plan.rows as isize
                    })
                    // turn back into an index on array
                    .map(|(x, y)| (y * floor_plan.cols as isize + x) as usize)
                    // Only want neighbors that are seats (not a floor)
                    .filter(|i| floor_plan.plan[*i] != Position::Floor)
                    .collect(),
            )
        })
        .collect()
}

fn get_aof(floor_plan: &FloorPlan) -> Vec<(usize, Vec<usize>)> {
    floor_plan
        .plan
        .iter()
        .enumerate()
        .filter(|(_, p)| p != &&Position::Floor)
        .map(|(i, _)| {
            (
                i,
                (0..9)
                    .filter(|p| p != &4) // exclude middle
                    // generate relative x and y to i for neighbors
                    .map(|p| (p as isize % 3 - 1, p as isize / 3 - 1))
                    .filter_map(|(rx, ry)| {
                        (1..)
                            // get x and y from index and apply offsets with multiples (f) for scanning out
                            .map(|f| {
                                (
                                    (i % floor_plan.cols) as isize + rx * f,
                                    (i / floor_plan.cols) as isize + ry * f,
                                )
                            })
                            // Make sure we don't breach boundaries
                            .take_while(|(x, y)| {
                                *x >= 0
                                    && *y >= 0
                                    && *x < floor_plan.cols as isize
                                    && *y < floor_plan.rows as isize
                            })
                            // Turn the x and y into an index
                            .map(|(x, y)| (y * floor_plan.cols as isize + x) as usize)
                            // only want the first seat (not a floor)
                            .find(|i| floor_plan.plan[*i] != Position::Floor)
                    })
                    .collect(),
            )
        })
        .collect()
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
