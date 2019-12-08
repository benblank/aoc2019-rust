use crate::intcomp::Intcomp;
use std::fs;
use std::str;

const INPUT_PATH: &str = "day05.input.txt";

pub fn part1() {
    let input = fs::read(INPUT_PATH).unwrap();
    let memory = input
        .split(|byte| byte == &b',')
        .map(|number| str::from_utf8(&number).unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // Executing the program creates its own ouput.
    Intcomp::new(&memory).execute();
}
