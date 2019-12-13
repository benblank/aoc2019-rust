use crate::intcomp::{read_program, Intcomp};
use std::collections::HashMap;

const INPUT_PATH: &str = "day11.input.txt";

fn turn_left(facing: (i32, i32)) -> (i32, i32) {
    match facing {
        (-1, 0) => (0, 1),
        (0, -1) => (-1, 0),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        _ => panic!("invalid facing {:?}", facing),
    }
}

fn turn_right(facing: (i32, i32)) -> (i32, i32) {
    match facing {
        (-1, 0) => (0, -1),
        (0, -1) => (1, 0),
        (0, 1) => (-1, 0),
        (1, 0) => (0, 1),
        _ => panic!("invalid facing {:?}", facing),
    }
}

pub fn part1() {
    let initializer = read_program(INPUT_PATH);
    let mut intcomp = Intcomp::new(&initializer);
    let mut hull = HashMap::new();
    let mut position = (0, 0);
    let mut facing = (0, -1); // "Up".

    loop {
        let input = if hull.contains_key(&position) {
            *hull.get(&position).unwrap()
        } else {
            0
        };

        intcomp.send_input(input);
        intcomp.execute();

        if let Some(color) = intcomp.receive_output() {
            let direction = intcomp
                .receive_output()
                .expect("no turn direction supplied");

            hull.insert(position, color);

            facing = if direction == 0 {
                turn_left(facing)
            } else {
                turn_right(facing)
            };

            position = (position.0 + facing.0, position.1 + facing.1);
        } else {
            break;
        }
    }

    println!("Total panels painted: {}", hull.len());
}
