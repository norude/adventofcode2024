#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

use clap::{Parser, Subcommand};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    day: SelectedDay,
}
#[derive(Subcommand)]
enum SelectedDay {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}
fn main() {
    let cli = Cli::parse();
    #[allow(clippy::match_same_arms)]
    match cli.day {
        SelectedDay::Day1 => { print!("day 1 part a:"); day1::a(); print!("day 1 part b:"); day1::b(); }
        SelectedDay::Day2 => { print!("day 2 part a:"); day2::a(); print!("day 2 part b:"); day2::b(); }
        SelectedDay::Day3 => { print!("day 3 part a:"); day3::a(); print!("day 3 part b:"); day3::b(); }
        SelectedDay::Day4 => { print!("day 4 part a:"); day4::a(); print!("day 4 part b:"); day4::b(); }
        SelectedDay::Day5 => { print!("day 5 part a:"); day5::a(); print!("day 5 part b:"); day5::b(); }
        SelectedDay::Day6 => { print!("day 6 part a:"); day6::a(); print!("day 6 part b:"); day6::b(); }
        SelectedDay::Day7 => { print!("day 7 part a:"); day7::a(); print!("day 7 part b:"); day7::b(); }
        SelectedDay::Day8 => { print!("day 8 part a:"); day8::a(); print!("day 8 part b:"); day8::b(); }
        SelectedDay::Day9 => { print!("day 9 part a:"); day9::a(); print!("day 9 part b:"); day9::b(); }
        SelectedDay::Day10 => { print!("day 10 part a:"); day10::a(); print!("day 10 part b:"); day10::b(); }
        SelectedDay::Day11 => { print!("day 11 part a:"); day11::a(); print!("day 11 part b:"); day11::b(); }
        SelectedDay::Day12 => { print!("day 12 part a:"); day12::a(); print!("day 12 part b:"); day12::b(); }
        SelectedDay::Day13 => { print!("day 13 part a:"); day13::a(); print!("day 13 part b:"); day13::b(); }
        SelectedDay::Day14 => { print!("day 14 part a:"); day14::a(); print!("day 14 part b:"); day14::b(); }
        SelectedDay::Day15 => { print!("day 15 part a:"); day15::a(); print!("day 15 part b:"); day15::b(); }
        SelectedDay::Day16 => { print!("day 16 part a:"); day16::a(); print!("day 16 part b:"); day16::b(); }
        SelectedDay::Day17 => { print!("day 17 part a:"); day17::a(); print!("day 17 part b:"); day17::b(); }
        SelectedDay::Day18 => { print!("day 18 part a:"); day18::a(); print!("day 18 part b:"); day18::b(); }
        SelectedDay::Day19 => { print!("day 19 part a:"); day19::a(); print!("day 19 part b:"); day19::b(); }
        SelectedDay::Day20 => { print!("day 20 part a:"); day20::a(); print!("day 20 part b:"); day20::b(); }
        SelectedDay::Day21 => { print!("day 21 part a:"); day21::a(); print!("day 21 part b:"); day21::b(); }
        SelectedDay::Day22 => { /* print!("day 22 part a:"); day22::a(); print!("day 22 part b:"); day22::b(); */ }
        SelectedDay::Day23 => { /* print!("day 23 part a:"); day23::a(); print!("day 23 part b:"); day23::b(); */ }
        SelectedDay::Day24 => { /* print!("day 24 part a:"); day24::a(); print!("day 24 part b:"); day24::b(); */ }
        SelectedDay::Day25 => { /* print!("day 25 part a:"); day25::a(); print!("day 25 part b:"); day25::b(); */ }
    }
}
