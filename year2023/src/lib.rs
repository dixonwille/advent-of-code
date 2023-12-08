use lib_aoc::prelude::*;
use std::{fs, path::Path};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

pub struct Solutions {}

impl Solver for Solutions {
    fn load(day: u8) -> String {
        fs::read_to_string(format!("inputs/day{day:02}.txt"))
            .expect("Puzzle input could not be read")
    }

    fn load_test(day: u8, part: bool) -> String {
        let test1 = format!("inputs/tests/day{day:02}.txt");
        let test2 = format!("inputs/tests/day{day:02}b.txt");
        let path = match part {
            PART_ONE => Path::new(&test1),
            PART_TWO => {
                let test = Path::new(&test2);
                if test.exists() {
                    test
                } else {
                    Path::new(&test1)
                }
            }
        };
        fs::read_to_string(path).expect("Puzzle input could not be read")
    }
}
