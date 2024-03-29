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
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod intcomp;

extern crate itertools;
#[cfg(test)]
extern crate maplit;
extern crate permutohedron;

use std::env;

fn main() {
    for arg in env::args().skip(1) {
        match arg.as_ref() {
            "1.1" => day01::part1(),
            "1.2" => day01::part2(),
            "2.1" => day02::part1(),
            "2.2" => day02::part2(),
            "3.1" => day03::part1(),
            "3.2" => day03::part2(),
            "4.1" => day04::part1(),
            "4.2" => day04::part2(),
            "5.1" => day05::part1(),
            "5.2" => day05::part2(),
            "6.1" => day06::part1(),
            "6.2" => day06::part2(),
            "7.1" => day07::part1(),
            "7.2" => day07::part2(),
            "8.1" => day08::part1(),
            "8.2" => day08::part2(),
            "9.1" => day09::part1(),
            "9.2" => day09::part2(),
            "10.1" => day10::part1(),
            "10.2" => day10::part2(),
            "11.1" => day11::part1(),
            "11.2" => day11::part2(),
            "12.1" => day12::part1(),
            "12.2" => day12::part2(),
            "13.1" => day13::part1(),
            "13.2" => day13::part2(),
            "14.1" => day14::part1(),
            "14.2" => day14::part2(),
            "15.1" => day15::part1(),
            "15.2" => day15::part2(),
            "16.1" => day16::part1(),
            "16.2" => day16::part2(),
            "17.1" => day17::part1(),
            "17.2" => day17::part2(),
            _ => eprintln!("Day {} hasn't been written yet!", arg),
        }
    }
}
