use aoc_runner_derive::aoc_lib;

// TODO would be cool to see how if I can use pest instead of nom and compare results of the parsers (make sure to look at streaming)
// TODO remove use of aoc in favor of my own helpers so I can write meaningful benchmarks with multiple parsers and solutions
// TODO may make sense to write some macros or build.rs to help maintain boiler plate
// TODO support workspaces so that I have all the years in one workspace but CLI can run specific year/day/part

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;

aoc_lib!(year = 2020);
