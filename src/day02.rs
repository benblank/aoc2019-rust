use crate::intcomp::execute;
use std::fs;
use std::str;

const INPUT_PATH: &str = "day02.input.txt";

pub fn part1() {
    let input = fs::read(INPUT_PATH).unwrap();
    let mut memory = input
        .split(|byte| byte == &b',')
        .map(|number| str::from_utf8(&number).unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    memory[1] = 12;
    memory[2] = 2;

    execute(&mut memory);

    println!("Value at position 0: {}", memory[0]);
}

pub fn part2() {
    let input = fs::read(INPUT_PATH).unwrap();
    let initializer = input
        .split(|byte| byte == &b',')
        .map(|number| str::from_utf8(&number).unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = initializer.to_vec();

            memory[1] = noun;
            memory[2] = verb;

            execute(&mut memory);

            if memory[0] == 1969_0720 {
                println!("Target noun: {}", noun);
                println!("Target verb: {}", verb);

                return;
            }
        }
    }
}
