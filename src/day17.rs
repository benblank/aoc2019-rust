use crate::intcomp::{read_program, Intcomp};

const INPUT_PATH: &str = "day17.input.txt";

pub fn part1() {
    let initializer = read_program(INPUT_PATH);
    let mut intcomp = Intcomp::new(&initializer);

    intcomp.execute();

    let mut output = Vec::new();

    while let Some(code) = intcomp.receive_output() {
        output.push(code);
    }

    let output = output
        .split(|code| *code == 10)
        .filter(|row| !row.is_empty())
        .collect::<Vec<_>>();

    let mut sum = 0;

    for (y, row) in output.iter().enumerate() {
        for (x, code) in row.iter().enumerate() {
            if *code == 35
                && y > 0
                && output[y - 1][x] == 35
                && y < output.len() - 1
                && output[y + 1][x] == 35
                && x > 0
                && output[y][x - 1] == 35
                && x < row.len() - 1
                && output[y][x + 1] == 35
            {
                sum += x * y;
            }
        }
    }

    println!("Sum of alignment parameters: {}", sum);
}
