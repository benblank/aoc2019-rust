use crate::intcomp::{read_program, Intcomp};
use std::collections::HashMap;

const INPUT_PATH: &str = "day13.input.txt";

pub fn part1() {
    let initializer = read_program(INPUT_PATH);
    let mut intcomp = Intcomp::new(&initializer);
    let mut screen = HashMap::new();

    intcomp.execute();

    while let Some(x) = intcomp.receive_output() {
        let y = intcomp.receive_output().expect("no y coordinate available");
        let tile_id = intcomp.receive_output().expect("no tile id availble");

        screen.insert((x, y), tile_id);
    }

    let block_count = screen.values().filter(|tile_id| **tile_id == 2).count();

    println!("Block tile count: {}", block_count);
}
