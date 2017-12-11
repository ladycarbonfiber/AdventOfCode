mod common;
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

extern crate petgraph;
extern crate itertools;

use std::collections::HashSet;
use std::collections::HashMap;

use common::*;
fn main() {
    println!("Advent of Code 2017");
    //day1::solve(Part::PartOne);
    //day2::solve(Part::PartOne);
    //day2::solve(Part::PartTwo);
    //day3::solve(Part::PartOne);
    //day4::solve(Part::PartOne);
    //day5::solve(Part::PartOne);
    //day5::solve(Part::PartTwo);
    //day6::solve(Part::PartOne);
    //day6::solve(Part::PartTwo);
    //day7::solve(Part::PartOne);
    //day7::solve(Part::PartTwo);
    //day10::solve(Part::PartOne);
    //day10::solve(Part::PartTwo);
    day11::solve(Part::PartOne);
    day11::solve(Part::PartTwo);

}




