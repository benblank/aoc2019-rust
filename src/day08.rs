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

pub fn part2() {
    let input = fs::read(INPUT_PATH)
        .expect("could not read input file")
        .iter()
        .map(|digit| (*digit as char).to_digit(10).expect("bad digit"))
        .collect::<Vec<_>>();
    let layers = input.chunks(WIDTH * HEIGHT).rev().collect::<Vec<_>>();
    let mut image = layers[0].to_owned();

    for i in 1..(WIDTH * HEIGHT) {
        for layer in &layers {
            if layer[i] != 2 {
                image[i] = layer[i];
            }
        }
    }

    for row in image.chunks(WIDTH) {
        println!(
            "{}",
            row.iter()
                .map(|number| if *number == 0 { " " } else { "#" })
                .collect::<Vec<_>>()
                .join("")
        );
    }
}
