use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match (cli.day, cli.part) {
        (1, 1) => day01::run_part_a(),
        (1, 2) => day01::run_part_b(),
        (2, 1) => day02::run_part_a(),
        (2, 2) => day02::run_part_b(),
        (3, 1) => day03::run_part_a(),
        (3, 2) => day03::run_part_b(),
        (4, 1) => day04::run_part_a(),
        (4, 2) => day04::run_part_b(),
        (5, 1) => day05::run_part_a(),
        (5, 2) => day05::run_part_b(),
        (6, 1) => day06::run_part_a(),
        (6, 2) => day06::run_part_b(),
        (7, 1) => day07::run_part_a(),
        (7, 2) => day07::run_part_b(),
        (8, 1) => day08::run_part_a(),
        (8, 2) => day08::run_part_b(),
        (9, 1) => day09::run_part_a(),
        (9, 2) => day09::run_part_b(),
        (10, 1) => day10::run_part_a(),
        (10, 2) => day10::run_part_b(),
        (11, 1) => day11::run_part_a(),
        (11, 2) => day11::run_part_b(),
        (12, 1) => day12::run_part_a(),
        (12, 2) => day12::run_part_b(),
        _ => unimplemented!(),
    }
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

#[derive(Parser)]
struct Cli {
    day: usize,
    part: usize,
}
