use crate::intcomp::{read_program, Intcomp};
use std::u8;

const INPUT_PATH: &str = "day17.input.txt";

fn print_output(intcomp: &mut Intcomp) {
    while let Some(code) = intcomp.receive_output() {
        // If it's too big to be ASCII, it's the final output.
        if code > u8::MAX as i64 {
            println!("{}", code);
        } else {
            print!("{}", (code as u8) as char);
        }
    }
}

fn send_ascii_line(intcomp: &mut Intcomp, line: &[u8]) {
    for code in line {
        intcomp.send_input(*code as i64);
    }

    intcomp.send_input(10);
}

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

pub fn part2() {
    let initializer = read_program(INPUT_PATH);
    let mut intcomp = Intcomp::new(&initializer);

    intcomp.write_memory(0, 2);
    intcomp.execute();

    print_output(&mut intcomp);
    send_ascii_line(&mut intcomp, b"A,A,B,C,B,C,B,C,C,A");

    intcomp.execute();

    print_output(&mut intcomp);
    send_ascii_line(&mut intcomp, b"L,10,R,8,R,8");

    intcomp.execute();

    print_output(&mut intcomp);
    send_ascii_line(&mut intcomp, b"L,10,L,12,R,8,R,10");

    intcomp.execute();

    print_output(&mut intcomp);
    send_ascii_line(&mut intcomp, b"R,10,L,12,R,10");

    intcomp.execute();

    print_output(&mut intcomp);
    send_ascii_line(&mut intcomp, b"n");

    intcomp.execute();

    print_output(&mut intcomp);
}
