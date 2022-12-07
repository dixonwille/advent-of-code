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
        _ => unimplemented!(),
    }
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

#[derive(Parser)]
struct Cli {
    day: usize,
    part: usize,
}
