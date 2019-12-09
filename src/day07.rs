use crate::intcomp::Intcomp;
use permutohedron::Heap;
use std::fs;
use std::str;

const INPUT_PATH: &str = "day07.input.txt";

pub fn part1() {
    let input = fs::read(INPUT_PATH).unwrap();
    let initializer = input
        .split(|byte| byte == &b',')
        .map(|number| str::from_utf8(&number).unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut max_value = -1;

    for phase_settings in Heap::new(&mut vec![0, 1, 2, 3, 4]) {
        let mut value = 0;

        for phase_setting in phase_settings {
            let stdin = format!("{}\n{}\n", phase_setting, value);
            let mut stdout = Vec::new();
            let mut intcomp = Intcomp::new(stdin.as_bytes(), &mut stdout, &initializer);

            intcomp.execute();

            value = String::from_utf8(stdout)
                .expect("not UTF-8")
                .trim()
                .parse::<i32>()
                .unwrap();
        }

        if value > max_value {
            max_value = value;
        }
    }

    println!("Max thruster signal: {}", max_value);
}
