use crate::intcomp::Intcomp;
use std::fs;
use std::io;
use std::str;

const INPUT_PATH: &str = "day02.input.txt";

pub fn part1() {
    let input = fs::read(INPUT_PATH).unwrap();
    let initializer = input
        .split(|byte| byte == &b',')
        .map(|number| str::from_utf8(&number).unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let stdin = io::stdin();
    let mut intcomp = Intcomp::new(stdin.lock(), io::stdout(), &initializer);

    intcomp.write_memory(1, 12);
    intcomp.write_memory(2, 2);
    intcomp.execute();

    println!("Value at position 0: {}", intcomp.read_memory(0));
}

pub fn part2() {
    let input = fs::read(INPUT_PATH).unwrap();
    let initializer = input
        .split(|byte| byte == &b',')
        .map(|number| str::from_utf8(&number).unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for noun in 0..100 {
        for verb in 0..100 {
            let stdin = io::stdin();
            let mut intcomp = Intcomp::new(stdin.lock(), io::stdout(), &initializer);

            intcomp.write_memory(1, noun);
            intcomp.write_memory(2, verb);
            intcomp.execute();

            if intcomp.read_memory(0) == 1969_0720 {
                println!("Target noun: {}", noun);
                println!("Target verb: {}", verb);

                return;
            }
        }
    }
}
