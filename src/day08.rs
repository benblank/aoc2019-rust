use std::fs;
use std::{str, usize};

const HEIGHT: usize = 6;
const INPUT_PATH: &str = "day08.input.txt";
const WIDTH: usize = 25;

pub fn part1() {
    let input = fs::read(INPUT_PATH)
        .expect("could not read input file")
        .iter()
        .map(|digit| (*digit as char).to_digit(10).expect("bad digit"))
        .collect::<Vec<_>>();

    let mut least_zeroes = usize::MAX;
    let mut result = 0;

    for layer in input.chunks(WIDTH * HEIGHT) {
        let zeroes = layer.iter().filter(|i| **i == 0).count();

        if zeroes < least_zeroes {
            let ones = layer.iter().filter(|i| **i == 1).count();
            let twos = layer.iter().filter(|i| **i == 2).count();

            least_zeroes = zeroes;
            result = ones * twos;
        }
    }

    println!("Result: {}", result);
}
