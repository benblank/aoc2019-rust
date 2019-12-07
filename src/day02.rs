use crate::intcomp::execute;
use std::io::{self, BufRead};
use std::str;

pub fn part1() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut memory = handle
        .split(b',')
        .map(|number| {
            str::from_utf8(&number.unwrap())
                .unwrap()
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    memory[1] = 12;
    memory[2] = 2;

    execute(&mut memory);

    println!("Value at position 0: {}", memory[0]);
}

pub fn part2() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let initializer = handle
        .split(b',')
        .map(|number| {
            str::from_utf8(&number.unwrap())
                .unwrap()
                .parse::<i32>()
                .unwrap()
        })
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
