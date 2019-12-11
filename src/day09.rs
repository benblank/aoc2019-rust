use crate::intcomp::{read_program, Intcomp};

const INPUT_PATH: &str = "day09.input.txt";

pub fn part1() {
    let initializer = read_program(INPUT_PATH);
    let mut intcomp = Intcomp::new(&initializer);

    intcomp.send_input(1);
    intcomp.execute();

    println!(
        "BOOST keycode: {}",
        intcomp.receive_output().expect("no output available")
    );
}
