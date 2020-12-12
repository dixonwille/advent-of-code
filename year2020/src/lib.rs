use aoc_runner_derive::aoc_lib;

// TODO would be cool to see how if I can use pest instead of nom and compare results of the parsers (make sure to look at streaming)
// TODO remove use of aoc in favor of my own helpers so I can write meaningful benchmarks with multiple parsers and solutions
// TODO may make sense to write some macros or build.rs to help maintain boiler plate
// TODO support workspaces so that I have all the years in one workspace but CLI can run specific year/day/part

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;

aoc_lib!(year = 2020);
