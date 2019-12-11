use crate::intcomp::{read_program, Intcomp};

const INPUT_PATH: &str = "day02.input.txt";

pub fn part1() {
    let initializer = read_program(INPUT_PATH);
    let mut intcomp = Intcomp::new(&initializer);

    intcomp.write_memory(1, 12);
    intcomp.write_memory(2, 2);
    intcomp.execute();

    println!("Value at position 0: {}", intcomp.read_memory(0));
}

pub fn part2() {
    let initializer = read_program(INPUT_PATH);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcomp = Intcomp::new(&initializer);

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
