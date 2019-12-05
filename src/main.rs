mod day01;

use std::env;

fn main() {
    for arg in env::args().skip(1) {
        match arg.as_ref() {
            "1.1" => day01::part1(),
            "1.2" => day01::part2(),
            _ => eprintln!("Day {} hasn't been written yet!", arg),
        }
    }
}
