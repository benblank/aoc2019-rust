mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod intcomp;

#[cfg(test)]
extern crate maplit;

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
            _ => eprintln!("Day {} hasn't been written yet!", arg),
        }
    }
}
