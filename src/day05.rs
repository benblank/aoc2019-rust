use crate::intcomp::{read_program, Intcomp};

const INPUT_PATH: &str = "day05.input.txt";

pub fn part1() {
    let initializer = read_program(INPUT_PATH);
    let mut intcomp = Intcomp::new(&initializer);

    intcomp.send_input(1);
    intcomp.execute();

    let mut diagnostics = Vec::new();

    while let Some(output) = intcomp.receive_output() {
        diagnostics.push(output);
    }

    println!(
        "Diagnostics: {}",
        diagnostics
            .iter()
            .map(|code| code.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}

pub fn part2() {
    let initializer = read_program(INPUT_PATH);
    let mut intcomp = Intcomp::new(&initializer);

    intcomp.send_input(5);
    intcomp.execute();

    while let Some(output) = intcomp.receive_output() {
        println!("Diagnostic code: {}", output);
    }
}
